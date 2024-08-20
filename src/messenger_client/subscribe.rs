
use crate::messenger_client::models::MessagingService;
use kaishi::generated::MessageTagFilter;
use tonic::Request;
use futures_util::StreamExt;

impl MessagingService {
    pub async fn subscribe_messages(&self, filter_tags: Vec<String>) {
        let message_filter = MessageTagFilter {
            tags: filter_tags,
        };

        let mut client = self.client.lock().await;

        let response = client.subscribe_messages_by_tags(Request::new(message_filter)).await;

        match response {
            Ok(response) => {
                let mut response_stream = response.into_inner();
                while let Some(result) = response_stream.next().await {
                    match result {
                        Ok(message) => println!("Received message: {:?}", message),
                        Err(e) => println!("Error receiving message: {:?}", e),
                    }
                }
            }
            Err(e) => {
                println!("Failed to subscribe to messages: {:?}", e);
            }
        }
    }
}

