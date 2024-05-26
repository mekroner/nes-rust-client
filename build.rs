fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("./grpc/Health.proto")?;

    tonic_build::compile_protos("./grpc/google/protobuf/any.proto")?;
    tonic_build::compile_protos("./grpc/SerializableExpression.proto")?;
    tonic_build::compile_protos("./grpc/SerializableOperator.proto")?;
    tonic_build::compile_protos("./grpc/SerializableQueryPlan.proto")?;
    Ok(())
}
