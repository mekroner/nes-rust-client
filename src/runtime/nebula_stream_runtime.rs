use crate::query::{Query, QueryBuilder};
use crate::serialization::protobuf::serialize_query::serialize_request;

pub enum PlacementStrategy {
    BottomUp,
}

impl ToString for PlacementStrategy {
    fn to_string(&self) -> String {
        match self {
            PlacementStrategy::BottomUp => "BottomUp",
        }
        .to_string()
    }
}

pub struct NebulaStreamConfig {
    host: String,
    port: String,
}

pub struct NebulaStreamRuntime {
    config: NebulaStreamConfig,
}

impl NebulaStreamRuntime {
    // FIXME: This shoudl use make use or rusts IP address type
    pub fn new(host: impl Into<String>, port: i32) -> Self {
        let config = NebulaStreamConfig {
            host: host.into(),
            port: port.to_string(),
        };
        Self { config }
    }

    // FIXME: This should return false if reqwest cannot connect to coord
    pub async fn check_connection(&self) -> Result<bool, reqwest::Error> {
        log::info!("Checking connection.");
        let response = reqwest::get(self.coordinator_url("/v1/nes/connectivity/check")).await?;
        log::debug!("Response status: {}", response.status());
        let body = response.text().await?;
        log::debug!("Response body: {}", body);
        let json_value: serde_json::Value = serde_json::from_str(&body).unwrap();
        let Some(serde_json::Value::Bool(is_connected)) = json_value.get("success") else {
            panic!("Expected body to contain success field")
        };
        Ok(*is_connected)
    }

    pub fn from_source(&self, source_name: impl Into<String>) -> QueryBuilder {
        QueryBuilder::from_source(source_name)
    }

    pub async fn execute_query(
        &self,
        query: &Query,
        placement: PlacementStrategy,
    ) -> Result<i64, reqwest::Error> {
        log::info!("Attempting to Execute Query: {:?}", query);
        let client = reqwest::Client::builder().build().unwrap();
        let request = serialize_request(query, placement);
        let response = client
            .post(self.coordinator_url("/v1/nes/query/execute-query-ex"))
            .body(request)
            .send()
            .await?;

        log::debug!("Response status: {}", response.status());
        let body = response.text().await?;
        log::debug!("Response body: {}", body);
        let json_value: serde_json::Value = serde_json::from_str(&body).unwrap();
        let Some(serde_json::Value::Number(number)) = json_value.get("queryId") else {
            panic!("The response by the coordinator did not contain a query ID")
        };
        let Some(query_id) = number.as_i64() else {
            panic!("Expected query ID to be i64!");
        };
        Ok(query_id)
    }

    pub fn query_status(query_id: i64) -> Result<String, reqwest::Error> {
        todo!();
    }

    pub fn stop_query(query_id: i64) -> Result<String, reqwest::Error> {
        todo!();
    }

    pub async fn logical_sources(&self) -> Result<Vec<String>, reqwest::Error> {
        log::info!("Requesting logical sources.");
        let response =
            reqwest::get(self.coordinator_url("/v1/nes/sourceCatalog/allLogicalSource")).await?;
        log::debug!("Response status: {}", response.status());
        let body = response.text().await?;
        log::debug!("Response body: {}", body);
        let serde_json::Value::Array(json_arr) =
            serde_json::from_str(&body).expect("Parsing JSON should not fail!")
        else {
            panic!("Body should be a JSON Array!")
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
