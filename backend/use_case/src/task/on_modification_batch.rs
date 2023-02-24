use common_domain::{
    define_repo,
    error::{Error, Result},
};
use task_domain::model::modification::Modification;

define_repo! {
    pub struct OnModificationBatchRepository<A, B, C, D> {
        pub is_transaction_in_progress: Fn() -> Result<bool> as A,
        pub write_modifications: FnOnce(modifications: Vec<Modification>) -> Result<()> as B,
        pub increment_transaction_counter: Fn(count: u64) -> Result<()> as C,
        pub finish_transaction_if_ready: Fn() -> Result<()> as D,
    }
}

pub async fn on_modification_batch<A, B, C, D>(
    modificaitons: Vec<Modification>,
    repo: OnModificationBatchRepository<A, B, C, D>,
) -> Result<()>
where
    A: IsTransactionInProgressType,
    B: WriteModificationsType,
    C: IncrementTransactionCounterType,
    D: FinishTransactionIfReadyType,
{
    let is_transaction_in_progress = (repo.is_transaction_in_progress)().await?;
    if !is_transaction_in_progress {
        return Err(transaction_not_in_progress_error());
    }

    let count = modificaitons.len() as u64;
    (repo.write_modifications)(modificaitons).await?;
    (repo.increment_transaction_counter)(count).await?;

    (repo.finish_transaction_if_ready)().await
}

fn transaction_not_in_progress_error() -> Error {
    Error::unknown("Transaction not in progress")
}
