
use tonic::{Request, transport::Endpoint};
use generated::messenger_service_client::MessengerServiceClient;
use generated::matching_service_client::MatchingServiceClient;
use generated::{MessageFilter, AnalyzeTextRequest};
use tokio_stream::iter;
use dialoguer::{Input, Select};

pub mod generated {
    include!(concat!(env!("OUT_DIR"), "/messenger.rs"));  // Use the correct proto file for MessengerService
    include!(concat!(env!("OUT_DIR"), "/matching.rs"));
}

async fn process_analysis_request(client: &mut MatchingServiceClient<tonic::transport::Channel>, query_sentence: String) -> Result<(), Box<dyn std::error::Error>> {
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create an endpoint for the MessengerService
    println!("Connecting to MessengerService at http://0.0.0.0:50052");
    let messenger_endpoint = Endpoint::from_static("http://0.0.0.0:50052")
        .keep_alive_while_idle(true)
        .keep_alive_timeout(std::time::Duration::from_secs(200000))
        .timeout(std::time::Duration::from_secs(60));

    // Create a separate endpoint for the MatchingService
    println!("Connecting to MatchingService at http://0.0.0.0:50053");
    let matching_endpoint = Endpoint::from_static("http://0.0.0.0:50053")
        .keep_alive_while_idle(true)
        .keep_alive_timeout(std::time::Duration::from_secs(200000))
        .timeout(std::time::Duration::from_secs(60));

    // Establish connections to each service
    let messenger_channel = messenger_endpoint.connect().await?;
    let matching_channel = matching_endpoint.connect().await?;

    // Create clients for each service
    let mut messenger_client = MessengerServiceClient::new(messenger_channel);
    let mut matching_client = MatchingServiceClient::new(matching_channel);

    // 1. Handle MessengerService request
    let message_filter = MessageFilter {
        tags: vec!["info".to_string()],  // Specify the tags you want to filter on
    };
    let filter_request = Request::new(message_filter);

    let message_response = messenger_client.subscribe_messages(filter_request).await?;
    let mut message_stream = message_response.into_inner();

    // Process the MessengerService stream
    tokio::spawn(async move {
        while let Some(message) = message_stream.message().await.unwrap() {
            println!("Received message: {}", message.message_text);
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
            .default(0)
            .interact()?;

        match selection {
            0 => {
                process_analysis_request(&mut matching_client, default_sentence).await?;
            },
            1 => {
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
