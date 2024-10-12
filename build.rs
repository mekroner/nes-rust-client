extern crate prost_build;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // prost_build::compile_protos("./grpc/Health.proto")?;

    let mut config = prost_build::Config::new();
    config.enable_type_names();
    config.protoc_arg("--experimental_allow_proto3_optional");
    config.compile_protos(&["./grpc/WorkerLocation.proto"], &["grpc/"])?;
    config.compile_protos(&["grpc/SerializableExpression.proto"], &["grpc/"])?;
    config.compile_protos(&["./grpc/SerializableOperator.proto"], &["grpc/"])?;
    config.compile_protos(&["./grpc/SerializableQueryPlan.proto"], &["grpc/"])?;
    Ok(())
}
