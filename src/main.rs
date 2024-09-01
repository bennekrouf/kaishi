// mod messenger_client;
mod process_analysis_request;
mod display_menu;

use tokio::sync::watch;
use futures_util::FutureExt;
use tonic::transport::Endpoint;

use std::sync::Arc;
use tokio::sync::Mutex;

use std::env;
use dotenvy::from_path;
use std::path::Path;
use kaishi::generated::matching_service_client::MatchingServiceClient;
use messengerc::{connect_to_messenger_service, MessagingService};
use crate::display_menu::display_menu_and_process_user_input;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load the environment variables from a custom file
    let custom_env_path = Path::new("proto-definitions/.service");
    from_path(custom_env_path).expect("Failed to load environment variables from custom path");

    let _matching_socket = env::var("MATCHING_SOCKET")?;

    let addr:String = env::var("MATCHING_ADDR")?.parse()?;
    // Create a separate endpoint for the MatchingService
    println!("Connecting to MatchingService at :{}", &addr);
    let matching_endpoint = Endpoint::from_shared(addr.clone());

    let matching_channel = matching_endpoint.expect("REASON").connect().await?;
    let mut matching_client = MatchingServiceClient::new(matching_channel);

    // --- -----  Messenger ---------------
    let messenger_tag = env::var("MESSENGER_TAG")?;

    // Create and initialize the gRPC client for the messaging service
    let messenger_client = connect_to_messenger_service().await
        .ok_or("Failed to connect to messenger service")?;

    let messaging_service = MessagingService::new(
        Arc::new(Mutex::new(messenger_client)),
        "kaishi".to_string(),
    );

    // Publish a message through the messaging service
    let message = format!("KaishiService server listening on {}", addr);
    if let Err(e) = messaging_service.publish_message(message.clone(), None).await {
        eprintln!("Failed to publish message: {:?}", e);
    }

    // Set up a watch channel to notify when the subscription task ends
    let (tx, mut rx) = watch::channel(());

    // Spawn the subscription task in the background
    // let _subscription_task = tokio::spawn(async move {
    //     messaging_service_clone.subscribe_messages(vec!["toto".to_string()]).await;
    //     let _ = tx.send(());
    // });

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

