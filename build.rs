fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(false)
        .out_dir("src/google")
        .compile(
            &["proto/google/cloud/aiplatform/v1/prediction_service.proto"],
            &["proto"],
        )?;
    Ok(())
}
