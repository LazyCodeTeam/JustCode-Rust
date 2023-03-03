use common_domain::error::{ErrorOutput, ErrorType};
use common_domain::{define_repo, error::Error, error::Result};
use content_domain::model::expected_technology_data::ExpectedTechnologyData;
use content_domain::model::full_content::FullContent;
use content_domain::model::modification::Modification;

define_repo! {
    pub struct LoadContentRepository<A, B, C, D, E> {
        pub get_full_content: Fn() -> Result<FullContent> as A,
        pub add_modifications_to_queue: Fn(modifications: Vec<Modification>) -> Result<()> as B,
        pub is_transaction_in_progress: Fn() -> Result<bool> as C,
        pub begin_transaction: Fn(modifications: u64) -> Result<()> as D,
        pub increase_queue_items_count: Fn(modifications: u64) -> Result<()> as E,
    }
}

pub async fn load_tasks<A, B, C, D, E>(
    content: Vec<ExpectedTechnologyData>,
    repo: LoadContentRepository<A, B, C, D, E>,
) -> Result<()>
where
    A: GetFullContentType,
    B: AddModificationsToQueueType,
    C: IsTransactionInProgressType,
    D: BeginTransactionType,
    E: IncreaseQueueItemsCountType,
{
    let result = (repo.get_full_content)().await?;
    let changes = result.detect_changes(content);

    if changes.is_empty() {
        return Ok(());
    }

    let is_transaction_in_progress = (repo.is_transaction_in_progress)().await?;
    if is_transaction_in_progress {
        return Err(transaction_already_in_progress_error());
    }

    let changes_count = changes.len() as u64;
    (repo.begin_transaction)(changes_count).await?;

    (repo.add_modifications_to_queue)(changes).await?;

    (repo.increase_queue_items_count)(changes_count).await
}

fn transaction_already_in_progress_error() -> Error {
    Error {
        error_type: ErrorType::Conflict,
        debug_message: "Transaction already in progress - load_tasks".to_owned(),
        output: Box::new(ErrorOutput {
            message: "Tasks upload already in progress".to_owned(),
            code: "upload_already_in_progress".to_owned(),
            ..Default::default()
        }),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn should_return_error_when_transaction_already_in_progress() {
        let (ctx, _is_transaction_in_progress_lock) = mock_is_transaction_in_progress::ctx().await;
        ctx.expect().once().returning(|| Ok(true));

        let (ctx, _get_raw_tasks_tree_lock) = mock_get_full_content::ctx().await;
        ctx.expect().once().returning(|| Ok(FullContent::default()));

        let (ctx, _begin_transaction_lock) = mock_begin_transaction::ctx().await;
        ctx.expect().never();

        let (ctx, _add_modifications_to_queue_lock) = mock_add_modifications_to_queue::ctx().await;
        ctx.expect().never();

        let (ctx, _proceed_transaction_lock) = mock_increase_queue_items_count::ctx().await;
        ctx.expect().never();

        let repo = LoadContentRepository {
            get_full_content: mock_get_full_content::call,
            add_modifications_to_queue: mock_add_modifications_to_queue::call,
            is_transaction_in_progress: mock_is_transaction_in_progress::call,
            begin_transaction: mock_begin_transaction::call,
            increase_queue_items_count: mock_increase_queue_items_count::call,
        };

        let result = load_tasks(vec![ExpectedTechnologyData::default()], repo).await;

        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap(),
            transaction_already_in_progress_error()
        );
    }

    #[tokio::test]
    async fn should_return_ok_when_no_changes() {
        let (ctx, _is_transaction_in_progress_lock) = mock_is_transaction_in_progress::ctx().await;
        ctx.expect().never();

        let (ctx, _get_raw_tasks_tree_lock) = mock_get_full_content::ctx().await;
        ctx.expect().once().returning(|| Ok(FullContent::default()));

        let (ctx, _begin_transaction_lock) = mock_begin_transaction::ctx().await;
        ctx.expect().never();

        let (ctx, _add_modifications_to_queue_lock) = mock_add_modifications_to_queue::ctx().await;
        ctx.expect().never();

        let (ctx, _proceed_transaction_lock) = mock_increase_queue_items_count::ctx().await;
        ctx.expect().never();

        let repo = LoadContentRepository {
            get_full_content: mock_get_full_content::call,
            add_modifications_to_queue: mock_add_modifications_to_queue::call,
            is_transaction_in_progress: mock_is_transaction_in_progress::call,
            begin_transaction: mock_begin_transaction::call,
            increase_queue_items_count: mock_increase_queue_items_count::call,
        };

        let result = load_tasks(vec![], repo).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn should_return_ok_when_changes() {
        let (ctx, _is_transaction_in_progress_lock) = mock_is_transaction_in_progress::ctx().await;
        ctx.expect().once().returning(|| Ok(false));

        let (ctx, _get_raw_tasks_tree_lock) = mock_get_full_content::ctx().await;
        ctx.expect().once().returning(|| Ok(FullContent::default()));

        let (ctx, _begin_transaction_lock) = mock_begin_transaction::ctx().await;
        ctx.expect()
            .once()
            .withf(|modifications| *modifications == 1)
            .returning(|_| Ok(()));

        let (ctx, _add_modifications_to_queue_lock) = mock_add_modifications_to_queue::ctx().await;
        ctx.expect()
            .once()
            .withf(|modifications| modifications.len() == 1)
            .returning(|_| Ok(()));

        let (ctx, _proceed_transaction_lock) = mock_increase_queue_items_count::ctx().await;
        ctx.expect()
            .withf(|modifications| *modifications == 1)
            .once()
            .returning(|_| Ok(()));

        let repo = LoadContentRepository {
            get_full_content: mock_get_full_content::call,
            add_modifications_to_queue: mock_add_modifications_to_queue::call,
            is_transaction_in_progress: mock_is_transaction_in_progress::call,
            begin_transaction: mock_begin_transaction::call,
            increase_queue_items_count: mock_increase_queue_items_count::call,
        };

        let result = load_tasks(vec![ExpectedTechnologyData::default()], repo).await;

        assert!(result.is_ok());
    }
}
