#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TasksTransactionState {
    PopulatingQueue,
    QueuePopulated,
    ProcessingQueue,
    Invalid,
}
