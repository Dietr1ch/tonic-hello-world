fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("schema/hello_world.proto")?;
    Ok(())
}
