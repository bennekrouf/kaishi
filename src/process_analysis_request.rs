use tonic::Request;
use kaishi::generated::matching_service_client::MatchingServiceClient;
use kaishi::generated::AnalyzeTextRequest;
use tokio_stream::iter;

pub async fn process_analysis_request(client: &mut MatchingServiceClient<tonic::transport::Channel>, query_sentence: String) -> Result<(), Box<dyn std::error::Error>> {
    let analyze_request = AnalyzeTextRequest {
        query_sentence,
        additional_info: "".into(), // Fill as needed
    };

    let analyze_stream = iter(vec![analyze_request]);

    let request = Request::new(analyze_stream);

    let matching_response = client.analyze_text(request).await?;
    let mut matching_stream = matching_response.into_inner();

    while let Some(message) = matching_stream.message().await? {
        println!("[Client] {}", message.result);
        println!("[Client] Requires more info: {}", message.requires_more_info);
    }

    Ok(())
}

