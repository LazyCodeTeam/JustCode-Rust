use common_domain::{
    define_repo,
    error::{Error, Result, ResultLogExt},
};
use content_domain::model::modification::Modification;
use snafu::{ResultExt, Snafu};

define_repo! {
    pub struct OnModificationBatchRepository<A, B, C, D> {
        pub is_transaction_in_progress: Fn() -> Result<bool> as A,
        pub write_modifications: FnOnce(modifications: Vec<Modification>) -> Result<()> as B,
        pub increment_transaction_counter: Fn(count: u64) -> Result<()> as C,
        pub finish_transaction_if_ready: Fn() -> Result<()> as D,
    }
}

#[derive(Debug, Snafu)]
pub enum OnModificationBatchError {
    #[snafu(display("Transaction is not in progress"))]
    TransactionNotInProgress,
    Infra {
        source: Error,
    },
}

pub async fn on_modification_batch<A, B, C, D>(
    modificaitons: Vec<Modification>,
    repo: OnModificationBatchRepository<A, B, C, D>,
) -> std::result::Result<(), OnModificationBatchError>
where
    A: IsTransactionInProgressType,
    B: WriteModificationsType,
    C: IncrementTransactionCounterType,
    D: FinishTransactionIfReadyType,
{
    let is_transaction_in_progress = (repo.is_transaction_in_progress)()
        .await
        .context(InfraSnafu)?;
    if !is_transaction_in_progress {
        return Err(OnModificationBatchError::TransactionNotInProgress).with_warn_log();
    }

    let count = modificaitons.len() as u64;
    (repo.write_modifications)(modificaitons)
        .await
        .context(InfraSnafu)?;
    (repo.increment_transaction_counter)(count)
        .await
        .context(InfraSnafu)?;

    (repo.finish_transaction_if_ready)()
        .await
        .context(InfraSnafu)
}
