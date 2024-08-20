mod publish;
mod messenger_client;
mod process_analysis_request;

use tonic::transport::Endpoint;
use std::sync::Arc;
use dotenv::dotenv;
use std::env;
use kaishi::generated::matching_service_client::MatchingServiceClient;
use messenger_client::{connect::connect_to_messenger_service, models::MessagingService};
use dialoguer::{Input, Select};
use crate::process_analysis_request::process_analysis_request;
 
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    // Create a separate endpoint for the MatchingService
    println!("Connecting to MatchingService at http://0.0.0.0:50053");
    let matching_endpoint = Endpoint::from_static("http://0.0.0.0:50053")
        .keep_alive_while_idle(true)
        .keep_alive_timeout(std::time::Duration::from_secs(200000))
        .timeout(std::time::Duration::from_secs(60));

    // let matching_channel = matching_endpoint.connect().await?;
    // let mut matching_client = MatchingServiceClient::new(matching_channel);

    // --- -----  Messenger ---------------
    let messenger_tag = env::var("MESSENGER_TAG")?;
    // Attempt to connect to MessengerService
    let messenger_client = match connect_to_messenger_service().await {
        Some(client) => Arc::new(tokio::sync::Mutex::new(client)),
        None => {
            eprintln!("Failed to connect to MessengerService.");
            return Ok(());
        }
    };

    let messaging_service = Arc::new(MessagingService::new(messenger_client, messenger_tag.clone()));
    let messaging_service_clone = Arc::clone(&messaging_service);

    messaging_service.publish_message("Hey c'est moi Client !!!".to_string(), Some(vec!["client".to_string()])).await;
    tokio::spawn(async move {
        messaging_service_clone.subscribe_messages(vec!["matcher".to_string()]).await;
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
                "Send a message".to_string(),  // New option for sending a message
                "Exit".to_string(),
            ])
            // .default(0)
            .interact()?;

        match selection {
            // 0 => {
            //     process_analysis_request(&mut matching_client, default_sentence).await?;
            // },
            // 1 => {
            //     let query_sentence: String = Input::new()
            //         .with_prompt("Enter a sentence to analyze:")
            //         .interact_text()?;
            //
            //     process_analysis_request(&mut matching_client, query_sentence).await?;
            // },
            2 => {
                let query_sentence: String = Input::new()
                    .with_prompt("Enter a message to matcher:")
                    .interact_text()?;
                    messaging_service.publish_message(query_sentence, Some(vec!["matcher".to_string()])).await;
            },
            3 => {
                println!("Exiting...");
                break;
            },
            _ => (),
        }
    }

    Ok(())
}

