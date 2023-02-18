use common_domain::{define_repo, error::Result};
use task_domain::model::expected_technology_data::ExpectedTechnologyData;

define_repo! {
    pub struct LoadTasksRepository {}
}

pub async fn load_tasks(
    _content: Vec<ExpectedTechnologyData>,
    _repo: LoadTasksRepository,
) -> Result<()> {
    Ok(())
}
