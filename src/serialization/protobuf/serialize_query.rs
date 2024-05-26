use crate::runtime::query::Query;
use self::nes_protobuf::*;

mod nes_protobuf {
    tonic::include_proto!("nes");
} 

pub async fn execute_query(query: Query, placement: String) -> Result<(), Box<dyn std::error::Error>>{
    // let mut  client = Query 
    todo!();
}

pub fn serialize(query: Query) {
    // let request = SerializableQueryPlan::
    todo!()
}

pub fn serialize_operator() {}
