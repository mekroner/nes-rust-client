use std::fmt::Display;

use crate::query::stringify::stringify_query;
use crate::query::{Query, QueryBuilder};
use crate::serialization::protobuf::serialize_query::serialize_request;

use super::query_state::QueryState;

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

pub struct RuntimeError(String);

impl Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<reqwest::Error> for RuntimeError {
    fn from(value: reqwest::Error) -> Self {
        RuntimeError(format!("{value}"))
    }
}

#[derive(Debug)]
pub struct QueryCatalogEntry {
    pub query_id: i64,
    pub query_status: QueryState,
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

    /// This function returns true if runtime is connected and false if not.
    pub async fn check_connection(&self) -> bool {
        log::debug!("Checking connection.");
        let Ok(response) = reqwest::get(self.coordinator_url("/v1/nes/connectivity/check")).await
        else {
            return false;
        };
        log::trace!("Response status: {}", response.status());
        let Ok(body) = response.text().await else {
            return false;
        };
        log::trace!("Response body: {}", body);
        let Ok(json_value) = serde_json::from_str::<serde_json::Value>(&body) else {
            return false;
        };
        let Some(serde_json::Value::Bool(is_connected)) = json_value.get("success") else {
            return false;
        };
        *is_connected
    }

    pub fn from_source(&self, source_name: impl Into<String>) -> QueryBuilder {
        QueryBuilder::from_source(source_name)
    }

    pub async fn execute_query(
        &self,
        query: &Query,
        placement: PlacementStrategy,
    ) -> Result<i64, RuntimeError> {
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
            let Some(serde_json::Value::String(message)) = json_value.get("message") else {
                return Err(RuntimeError(
                    "The response by the coordinator did not contain a query ID.".into(),
                ));
            };
            return Err(RuntimeError(format!("Error message: {message}.")));
        };
        let Some(query_id) = number.as_i64() else {
            return Err(RuntimeError("Expected query_id to be i64".into()));
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
            let Some(serde_json::Value::String(status_string)) = val.get("queryStatus") else {
                panic!("The response by the coordinator did not contain field queryStatus")
            };
            let Some(serde_json::Value::String(query_string)) = val.get("queryString") else {
                panic!("The response by the coordinator did not contain field queryString")
            };

            let query_status = match status_string.try_into() {
                Ok(ok) => ok,
                Err(err) => panic!("{err}"),
            };
            let entry = QueryCatalogEntry {
                query_id,
                query_status,
                query_string: query_string.to_string(),
            };
            queries.push(entry);
        }
        Ok(queries)
    }

    /// Returns the status of a query given the queries id. If the query is not registeded with the
    /// coordinator return None.
    pub async fn query_status(&self, query_id: i64) -> Result<Option<QueryState>, reqwest::Error> {
        log::debug!("Extracting status of query with id {query_id}.");
        let queries = self.registered_queries().await?;
        let Some(entry) = queries.iter().find(|e| e.query_id == query_id) else {
            return Ok(None);
        };
        Ok(Some(entry.query_status.clone()))
    }

    /// Returns an error if something went wrong
    pub async fn stop_query(&self, query_id: i64) -> Result<(), reqwest::Error> {
        log::debug!("Stopping query with id {query_id}.");
        let client = reqwest::Client::builder().build().unwrap();
        let response = client
            .delete(
                self.coordinator_url(&(format!("/v1/nes/query/stop-query?queryId={}", query_id))),
            )
            .send()
            .await?;
        log::trace!("Response status: {}", response.status());
        Ok(())
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

    fn coordinator_url<'a>(&self, end_point: &'a str) -> String {
        format!(
            "http://{}:{}{}",
            self.config.host, self.config.port, end_point
        )
    }
}
