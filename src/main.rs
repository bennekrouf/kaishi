
use tonic::Request;
use generated::log_service_client::LogServiceClient;
use generated::matching_service_client::MatchingServiceClient;
use generated::{LogRequest, AnalyzeTextRequest};
// use futures_util::StreamExt; // Import StreamExt for the Stream trait
// use tokio_stream::wrappers::ReceiverStream;

pub mod generated {
    include!(concat!(env!("OUT_DIR"), "/log.rs"));
    include!(concat!(env!("OUT_DIR"), "/matching.rs"));
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the gRPC server
    let mut log_client = LogServiceClient::connect("http://0.0.0.0:50052").await?;
    let mut matching_client = MatchingServiceClient::connect("http://0.0.0.0:50052").await?;

    // 1. Handle LogService request
    let log_request = Request::new(LogRequest {
        filter: "info".into(),
    });

    let log_response = log_client.stream_logs(log_request).await?;
    let mut log_stream = log_response.into_inner();

    // Process the LogService stream
    tokio::spawn(async move {
        while let Some(message) = log_stream.message().await.unwrap() {
            println!("Received log message: {}", message.log_message);
        }
    });

    // 2. Handle MatchingService request
    let analyze_request = AnalyzeTextRequest {
        query_sentence: "Send 2 laptops to John Mackenzie".into(),
        additional_info: "".into(), // Fill as needed
    };

    // Create a stream of AnalyzeTextRequest items
    let analyze_stream = tokio_stream::iter(vec![analyze_request]);

    // Wrap the stream in a Request
    let request = Request::new(analyze_stream);

    // Send the request and receive a stream of AnalyzeTextReply
    let matching_response = matching_client.analyze_text(request).await?;
    let mut matching_stream = matching_response.into_inner();

    // Process the MatchingService stream
    while let Some(message) = matching_stream.message().await? {
        println!("Received matching response: {}", message.result);
        println!("Requires more info: {}", message.requires_more_info);
    }

    Ok(())
}

