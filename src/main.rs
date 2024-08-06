
use tonic::{Request, transport::Endpoint};
use generated::log_service_client::LogServiceClient;
use generated::matching_service_client::MatchingServiceClient;
use generated::{LogRequest, AnalyzeTextRequest};
use tokio_stream::iter;
use dialoguer::{Input, Select};

pub mod generated {
    include!(concat!(env!("OUT_DIR"), "/log.rs"));
    include!(concat!(env!("OUT_DIR"), "/matching.rs"));
}

async fn process_analysis_request(client: &mut MatchingServiceClient<tonic::transport::Channel>, query_sentence: String) -> Result<(), Box<dyn std::error::Error>> {
    let analyze_request = AnalyzeTextRequest {
        query_sentence,
        additional_info: "".into(), // Fill as needed
    };

    // Create a stream of AnalyzeTextRequest items
    let analyze_stream = iter(vec![analyze_request]);

    // Wrap the stream in a Request
    let request = Request::new(analyze_stream);

    // Send the request and receive a stream of AnalyzeTextReply
    let matching_response = client.analyze_text(request).await?;
    let mut matching_stream = matching_response.into_inner();

    // Process the MatchingService stream
    while let Some(message) = matching_stream.message().await? {
        println!("[Client] Received matching response: {}", message.result);
        println!("[Client] Requires more info: {}", message.requires_more_info);
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create an endpoint with keep-alive settings
    let endpoint = Endpoint::from_static("http://0.0.0.0:50052")
        .keep_alive_while_idle(true)
        .keep_alive_timeout(std::time::Duration::from_secs(200000))
        .timeout(std::time::Duration::from_secs(60));

    // Establish a single connection to the gRPC server
    let channel = endpoint.connect().await?;

    // Create clients using the same connection
    let mut log_client = LogServiceClient::new(channel.clone());
    let mut matching_client = MatchingServiceClient::new(channel);

    // 1. Handle LogService request
    let log_request = Request::new(LogRequest {
        filter: "info".into(),
    });

    let log_response = log_client.stream_logs(log_request).await?;
    let mut log_stream = log_response.into_inner();

    // Process the LogService stream
    tokio::spawn(async move {
        while let Some(message) = log_stream.message().await.unwrap() {
            println!("[Client] Received log message: {}", message.log_message);
        }
    });

    // 2. Menu loop
    loop {
        let default_sentence = "Send 2 laptops to John Mackenzie".to_string();
        let select_option_text = format!("Run with default sentence ({})", &default_sentence);
        let selection = Select::new()
            .with_prompt("Select an option")
            .items(&[
                select_option_text,
                "Input a sentence".to_string(),
                "Exit".to_string(),
            ])
            .default(0) // Default to "Run with default sentence"
            .interact()?;

        match selection {
            0 => {
                // Run with the default sentence
                
                process_analysis_request(&mut matching_client, default_sentence).await?;
            },
            1 => {
                // Input a sentence
                let query_sentence: String = Input::new()
                    .with_prompt("Enter a sentence to analyze:")
                    .interact_text()?;

                process_analysis_request(&mut matching_client, query_sentence).await?;
            },
            2 => {
                println!("Exiting...");
                break;
            },
            _ => (),
        }
    }

    Ok(())
}

