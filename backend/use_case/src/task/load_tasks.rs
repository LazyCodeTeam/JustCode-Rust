use common_domain::{define_repo, error::Result};
use task_domain::model::expected_technology_data::ExpectedTechnologyData;
use task_domain::model::section::Section;
use task_domain::model::task::Task;
use task_domain::model::technology::Technology;

define_repo! {
    pub struct LoadTasksRepository<A, B> {
        pub get_tasks_tree: Fn() -> Result<(Vec<Technology>, Vec<Section>, Vec<Task>)> as A,
        pub add_actions_to_queue: Fn() -> Result<()> as B,
    }
}

pub async fn load_tasks<A, B>(
    _content: Vec<ExpectedTechnologyData>,
    repo: LoadTasksRepository<A, B>,
) -> Result<()>
where
    A: GetTasksTreeType,
    B: AddActionsToQueueType,
{
    let (_technologies, _sections, _tasks) = (repo.get_tasks_tree)().await?;
    Ok(())
}
