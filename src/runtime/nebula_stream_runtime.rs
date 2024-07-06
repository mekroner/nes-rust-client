use crate::query::stringify::stringify_query;
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

#[derive(Debug)]
pub struct QueryCatalogEntry {
    pub query_id: i64,
    pub query_status: String,
    pub query_string: String,
}

#[derive(Debug)]
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
        log::debug!("Checking connection.");
        let response = reqwest::get(self.coordinator_url("/v1/nes/connectivity/check")).await?;
        log::trace!("Response status: {}", response.status());
        let body = response.text().await?;
        log::trace!("Response body: {}", body);
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
        log::debug!("Attempting to Execute Query: {}", stringify_query(query));
        let client = reqwest::Client::builder().build().unwrap();
        let request = serialize_request(query, placement);
        let response = client
            .post(self.coordinator_url("/v1/nes/query/execute-query-ex"))
            .body(request)
            .send()
            .await?;

        log::trace!("Response status: {}", response.status());
        let body = response.text().await?;
        log::trace!("Response body: {}", body);
        let json_value: serde_json::Value = serde_json::from_str(&body).unwrap();
        let Some(serde_json::Value::Number(number)) = json_value.get("queryId") else {
            panic!("The response by the coordinator did not contain a query ID")
        };
        let Some(query_id) = number.as_i64() else {
            panic!("Expected query ID to be i64!");
        };
        Ok(query_id)
    }

    pub async fn registered_queries(&self) -> Result<Vec<QueryCatalogEntry>, reqwest::Error> {
        log::debug!("Requesting registered queries.");
        let response =
            reqwest::get(self.coordinator_url("/v1/nes/queryCatalog/allRegisteredQueries")).await?;
        log::trace!("Response status: {}", response.status());
        let body = response.text().await?;
        log::trace!("Response body: {}", body);
        let serde_json::Value::Array(json_arr) =
            serde_json::from_str(&body).expect("Parsing JSON should not fail!")
        else {
            panic!("Body should be a JSON Array!")
        };
        let mut queries = Vec::new();
        for val in json_arr {
            let Some(serde_json::Value::Number(number)) = val.get("queryId") else {
                panic!("The response by the coordinator did not contain field queryId")
            };
            let Some(query_id) = number.as_i64() else {
                panic!("Expected query ID to be i64!");
            };
            let Some(serde_json::Value::String(query_status)) = val.get("queryStatus") else {
                panic!("The response by the coordinator did not contain field queryStatus")
            };
            let Some(serde_json::Value::String(query_string)) = val.get("queryString") else {
                panic!("The response by the coordinator did not contain field queryString")
            };
            let entry = QueryCatalogEntry {
                query_id,
                query_status: query_status.to_string(),
                query_string: query_string.to_string(),
            };
            queries.push(entry);
        }
        Ok(queries)
    }

    /// Returns the status of a query given the queries id. If the query is not registeded with the
    /// coordinator return None.
    pub async fn query_status(&self, query_id: i64) -> Result<Option<String>, reqwest::Error> {
        let queries = self.registered_queries().await?;
        log::debug!("Extracting status of query with id {query_id}.");
        let Some(entry) = queries.iter().find(|e| e.query_id == query_id) else {
            return Ok(None);
        };
        Ok(Some(entry.query_status.clone()))
    }

    pub fn stop_query(&self, query_id: i64) -> Result<String, reqwest::Error> {
        unimplemented!();
    }

    pub async fn logical_sources(&self) -> Result<Vec<String>, reqwest::Error> {
        log::debug!("Requesting logical sources.");
        let response =
            reqwest::get(self.coordinator_url("/v1/nes/sourceCatalog/allLogicalSource")).await?;
        log::trace!("Response status: {}", response.status());
        let body = response.text().await?;
        log::trace!("Response body: {}", body);
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
