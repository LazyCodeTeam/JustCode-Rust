use common_domain::error::Result;
use task_domain::model::raw_tasks_tree::RawTasksTree;

use super::{
    get_all_technologies, get_sections::get_sections_for_multiple_technologies,
    get_tasks::get_tasks_for_multiple_technologies,
};

pub async fn get_raw_tasks_tree() -> Result<RawTasksTree> {
    let technologies = get_all_technologies().await?;
    let technologies_ids = technologies.iter().map(|t| t.id.clone()).collect();

    let sections = get_sections_for_multiple_technologies(technologies_ids).await?;
    let sections_ids = sections.iter().map(|s| s.id.clone()).collect();

    let tasks = get_tasks_for_multiple_technologies(sections_ids).await?;

    Ok(RawTasksTree {
        technologies,
        sections,
        tasks,
    })
}
