use std::error::Error;

use grpc_gemini_demo::google::cloud::aiplatform::v1::{
    part::Data, prediction_service_client::PredictionServiceClient, Content,
    GenerateContentRequest, Part,
};
use tonic::{
    metadata::MetadataValue,
    transport::{Channel, ClientTlsConfig},
    Request,
};

const AUTH_SCOPE: &[&str] = &["https://www.googleapis.com/auth/cloud-platform"];
const MODEL: &str = "gemini-1.0-pro-002";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let project_id = std::env::var("PROJECT_ID")?;
    let location_id = std::env::var("LOCATION_ID")?;

    let token_provider = gcp_auth::provider().await?;
    let token = token_provider.token(AUTH_SCOPE).await?;

    let bearer_token = format!("Bearer {}", token.as_str());
    let header_value: MetadataValue<_> = bearer_token.parse()?;

    let domain_name = format!("{location_id}-aiplatform.googleapis.com");
    let http_endpoint = format!("https://{domain_name}");

    let tls_config = ClientTlsConfig::new().with_native_roots();

    let channel = Channel::from_shared(http_endpoint)?
        .tls_config(tls_config)?
        .connect()
        .await?;

    let mut service = PredictionServiceClient::with_interceptor(channel, |mut req: Request<()>| {
        req.metadata_mut()
            .insert("authorization", header_value.clone());
        Ok(req)
    });

    let model =
        format!("projects/{project_id}/locations/{location_id}/publishers/google/models/{MODEL}");
    let generate_content_request = GenerateContentRequest {
        model,
        contents: vec![Content {
            role: "user".to_string(),
            parts: vec![Part {
                data: Some(Data::Text("Why is the sky blue".to_string())),
                ..Default::default()
            }],
            ..Default::default()
        }],
        ..Default::default()
    };

    let request = Request::new(generate_content_request);
    let response = service.generate_content(request).await?;

    println!("{response:?}");
    Ok(())
}
