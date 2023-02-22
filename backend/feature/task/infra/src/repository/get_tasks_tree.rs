use common_domain::error::Result;
use task_domain::model::{section::Section, task::Task, technology::Technology};

use super::get_all_technologies;

pub async fn get_tasks_tree() -> Result<(Vec<Technology>, Vec<Section>, Vec<Task>)> {
    Ok((get_all_technologies().await?, vec![], vec![]))
}
