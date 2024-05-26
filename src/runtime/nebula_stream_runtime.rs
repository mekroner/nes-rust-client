use std::collections::HashMap;

use self::health::HealthCheckRequest;

use super::query::{Query, QueryId};
use crate::{
    runtime::nebula_stream_runtime::health::health_client::HealthClient,
    serialization::cpp::serialize_query,
};

mod health {
    tonic::include_proto!("grpc.health.v1");
}

pub struct NebulaStreamConfig {
    host: String,
    port: String,
}

pub struct NebulaStreamRuntime {
    config: NebulaStreamConfig,
}

impl NebulaStreamRuntime {
    pub fn new(host: String, port: i32) -> Self {
        let config = NebulaStreamConfig {
            host,
            port: port.to_string(),
        };
        Self { config }
    }

    pub async fn check_health(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut client = HealthClient::connect("http://127.0.0.1:4000").await?;
        let request = HealthCheckRequest {
            service: "NES_DEFAULT_HEALTH_CHECK_SERVICE".to_string(),
        };
        let response = client.check(request).await?;
        println!("{:?}", response.into_inner());
        Ok(())
    }

    pub async fn check_connection(&self) -> Result<bool, reqwest::Error> {
        let response = reqwest::get(self.coordinator_url("/v1/nes/connectivity/check")).await?;
        println!("Status: {}", response.status());
        let body = response.text().await?;
        println!("Body: {}", body);
        Ok(true)
    }

    pub fn from_source(source_name: String) -> Query {
        unimplemented!();
    }

    // FIXME: This should support grpc, so we need a RequestHandler Interface
    pub async fn execute_query(
        &self,
        query: Query,
        placement: String,
    ) -> Result<QueryId, reqwest::Error> {
        let serialized_query = serialize_query::serialize(query);

        let mut payload = HashMap::new();
        payload.insert("placement", placement);
        payload.insert("userQuery", serialized_query);

        let client = reqwest::Client::builder().build().unwrap();
        let response = client
            .post(self.coordinator_url("/v1/nes/query/execute-query"))
            .json(&payload)
            .send()
            .await?;
        // handle response
        let body = response.text().await?;
        println!("{}", body);
        todo!()
    }

    pub async fn logical_sources(&self) -> Result<Vec<String>, reqwest::Error> {
        let response =
            reqwest::get(self.coordinator_url("/v1/nes/sourceCatalog/allLogicalSource")).await?;
        let body = response.text().await?;
        let value: serde_json::Value = serde_json::from_str(&body).unwrap();
        let serde_json::Value::Array(json_arr) = value else {
            panic!("Value should be a JSON Array!")
        };
        let mut source_list = Vec::new();
        for val in json_arr {
            let serde_json::Value::Object(json_obj) = val else {
                panic!("Value should be a JSON Object!")
            };
            let source = json_obj.keys().next().unwrap();
            source_list.push(source.clone());
        }
        Ok(source_list)
    }

    fn coordinator_url(&self, end_point: &'static str) -> String {
        format!(
            "http://{}:{}{}",
            self.config.host, self.config.port, end_point
        )
    }
}
