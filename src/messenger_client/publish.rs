
use crate::messenger_client::models::MessagingService;
use kaishi::generated::MessageRequest;
use tonic::Request;

impl MessagingService {
    pub async fn publish_message(&self, message: String, tags: Option<Vec<String>>) {
        let message_request = MessageRequest {
            message_text: message,
            tags: tags.unwrap_or_else(|| vec![self.tag.clone()]),
            gps_coordinates: None,
        };

        let mut client = self.client.lock().await;
        if let Err(e) = client.publish_message(Request::new(message_request)).await {
            println!("Failed to publish message: {:?}", e);
        }
    }
}

