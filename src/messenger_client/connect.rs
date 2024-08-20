
use dotenv::dotenv;
use std::env;
use tonic::transport::{Channel, Endpoint};
use kaishi::generated::messenger_service_client::MessengerServiceClient;

pub async fn connect_to_messenger_service() -> Option<MessengerServiceClient<Channel>> {
    dotenv().ok();
    let messenger_service_addr = env::var("MESSENGER_ADDR").ok()?;

    let messenger_service_endpoint = Endpoint::from_shared(messenger_service_addr.to_string())
        .expect("Invalid messenger service address")
        .keep_alive_while_idle(true)
        .keep_alive_timeout(std::time::Duration::from_secs(200000))
        .timeout(std::time::Duration::from_secs(60));

    match messenger_service_endpoint.connect().await {
        Ok(channel) => {
            println!("Connected to MessengerService.");
            Some(MessengerServiceClient::new(channel))
        },
        Err(e) => {
            println!("Failed to connect to MessengerService: {:?}", e);
            None
        }
    }
}

