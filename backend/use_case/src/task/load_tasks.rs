use common_domain::{define_repo, error::Result};
use task_domain::model::expected_technology_data::ExpectedTechnologyData;

define_repo! {
    pub struct LoadTasksRepository<A> {
        pub add_actions_to_queue: Fn() -> Result<()> as A,
    }
}

pub async fn load_tasks<A>(
    _content: Vec<ExpectedTechnologyData>,
    repo: LoadTasksRepository<A>,
) -> Result<()>
where
    A: AddActionsToQueueType,
{
    (repo.add_actions_to_queue)().await.unwrap();
    Ok(())
}
