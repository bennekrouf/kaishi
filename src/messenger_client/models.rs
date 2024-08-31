
use std::sync::Arc;
use tokio::sync::Mutex;

use kaishi::generated::messenger_service_client::MessengerServiceClient;

pub struct MessagingService {
    pub client: Arc<Mutex<MessengerServiceClient<tonic::transport::Channel>>>,
    pub tag: String,
}

impl MessagingService {
    pub fn new(client: Arc<Mutex<MessengerServiceClient<tonic::transport::Channel>>>, tag: String) -> Self {
        MessagingService {
            client,
            tag,
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct GPSCoordinates {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct GPSFilter {
    pub coordinates: GPSCoordinates,
}

