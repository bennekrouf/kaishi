
pub mod generated {
    include!(concat!(env!("OUT_DIR"), "/messenger.rs"));  // Use the correct proto file for MessengerService
    include!(concat!(env!("OUT_DIR"), "/matching.rs"));
}

