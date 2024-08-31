mod messenger_client;
mod process_analysis_request;
mod display_menu;

use tokio::sync::watch;
use futures_util::FutureExt;
use tonic::transport::Endpoint;
use std::sync::Arc;
use dotenv::dotenv;
use std::env;
use kaishi::generated::matching_service_client::MatchingServiceClient;
use messenger_client::{connect::connect_to_messenger_service, models::MessagingService};
use crate::display_menu::display_menu_and_process_user_input;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let _matching_socket = env::var("MATCHING_SOCKET")?;

    let addr:String = env::var("MATCHING_ADDR")?.parse()?;
    // Create a separate endpoint for the MatchingService
    println!("Connecting to MatchingService at :{}", addr);
    let matching_endpoint = Endpoint::from_shared(addr);

    let matching_channel = matching_endpoint.expect("REASON").connect().await?;
    let mut matching_client = MatchingServiceClient::new(matching_channel);

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

    let _ = messaging_service.publish_message("Hey c'est moi Client !!!".to_string(), Some(vec!["client".to_string()])).await;

    // Set up a watch channel to notify when the subscription task ends
    let (tx, mut rx) = watch::channel(());

    // Spawn the subscription task in the background
    let _subscription_task = tokio::spawn(async move {
        messaging_service_clone.subscribe_messages(vec!["toto".to_string()]).await;
        let _ = tx.send(());
    });

    // let subscription_task = tokio::spawn(async move {
    //     messaging_service_clone.subscribe_messages(vec!["matcher".to_string()]).await;
    // });
    loop {
        tokio::select! {
            _ = display_menu_and_process_user_input(&mut matching_client, &messaging_service).fuse() => {
                // Handle user input
            },
            _ = rx.changed() => {
                // If the subscription task ends, break the loop
                println!("Subscription task ended.");
                break;
            },
        }
    }
    // subscription_task.await??;
    Ok(())
}

