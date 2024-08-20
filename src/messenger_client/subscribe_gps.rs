
use crate::messenger_client::models::MessagingService;
use kaishi::generated::{MessageLocationFilter, GpsCoordinates};
use tonic::Request;
use futures_util::StreamExt;

impl MessagingService {
    pub async fn subscribe_messages_by_location(&self, gps_coordinates: GpsCoordinates) {
        let location_filter = MessageLocationFilter {
            location: Some(gps_coordinates),
        };

        let mut client = self.client.lock().await;

        let response = client.subscribe_messages_by_location(Request::new(location_filter)).await;

        match response {
            Ok(response) => {
                let mut response_stream = response.into_inner();
                while let Some(result) = response_stream.next().await {
                    match result {
                        Ok(message) => println!("Received message with GPS: {:?}", message.gps_coordinates),
                        Err(e) => println!("Error receiving message: {:?}", e),
                    }
                }
            }
            Err(e) => {
                println!("Failed to subscribe to messages by location: {:?}", e);
            }
        }
    }
}

