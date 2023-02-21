use common_domain::error::Result;
use common_infra::sqs_client::get_sqs_client;

use crate::config::CONFIG;

pub async fn add_actions_to_queue() -> Result<()> {
    let client = get_sqs_client().await;
    client
        .send_message()
        .queue_url(&CONFIG.task_migration_sqs_queuq)
        .message_body("test")
        .send()
        .await
        .unwrap();

    Ok(())
}
