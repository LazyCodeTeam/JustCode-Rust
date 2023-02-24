use aws_sdk_sqs::model::SendMessageBatchRequestEntry;
use common_domain::error::{Error, Result};
use common_infra::sqs_client::get_sqs_client;
use task_domain::model::modification::Modification;

use crate::{config::CONFIG, dto::modification_dto::ModificationDto};

const BATCH_SIZE: usize = 10;

pub async fn add_modifications_to_queue(modifications: Vec<Modification>) -> Result<()> {
    let dtos =
        modifications
            .into_iter()
            .map::<ModificationDto, _>(Into::into)
            .enumerate()
            .map(|(index, dto)| -> Result<SendMessageBatchRequestEntry> {
                Ok(SendMessageBatchRequestEntry::builder()
                    .id(index.to_string())
                    .message_body(serde_json::to_string(&dto).map_err(|e| {
                        Error::unknown(format!("Failed to parse modification: {e:?}"))
                    })?)
                    .build())
            })
            .collect::<Result<Vec<_>>>()?
            .chunks(BATCH_SIZE)
            .map(|chunk| chunk.into())
            .collect::<Vec<Vec<_>>>();

    let client = get_sqs_client().await;

    for chunk in dtos {
        client
            .send_message_batch()
            .queue_url(&CONFIG.task_migration_sqs_queue)
            .set_entries(Some(chunk))
            .send()
            .await
            .map_err(|e| Error::unknown(format!("Failed to add modifications to queue: {e:?}")))?;
    }

    Ok(())
}
