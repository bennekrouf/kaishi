
use kaishi::generated::matching_service_client::MatchingServiceClient;
// use messenger_client::{connect::connect_to_messenger_service, models::MessagingService};
use messengerc::MessagingService;

use dialoguer::{Input, Select};
use crate::process_analysis_request::process_analysis_request;
 
pub async fn display_menu_and_process_user_input(
    matching_client: &mut MatchingServiceClient<tonic::transport::Channel>,
    messaging_service: &MessagingService
) -> Result<(), Box<dyn std::error::Error>> {
    let default_sentence = "Send 2 laptops to John Mackenzie".to_string();
    let select_option_text = format!("Run with default sentence ({})", &default_sentence);
    let selection = Select::new()
        .with_prompt("Select an option")
        .items(&[
            select_option_text,
            "Input a sentence".to_string(),
            "Send a message".to_string(),
            "Exit".to_string(),
        ])
        .interact()?;

    match selection {
        0 => process_analysis_request(matching_client, default_sentence).await?,
        1 => {
            let query_sentence: String = Input::new()
                .with_prompt("Enter a sentence to analyze:")
                .interact_text()?;
            process_analysis_request(matching_client, query_sentence).await?;
        },
        2 => {
            let query_sentence: String = Input::new()
                .with_prompt("Enter a message to matcher:")
                .interact_text()?;
            messaging_service.publish_message(query_sentence, Some(vec!["client".to_string()])).await?;
        },
        3 => {
            println!("Exiting...");
            return Ok(());
        },
        _ => (),
    }

    Ok(())
}

