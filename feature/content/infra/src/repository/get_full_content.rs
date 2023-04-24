use common_domain::error::Result;
use content_domain::model::full_content::FullContent;

use super::{
    get_sections::get_sections_for_multiple_technologies,
    get_tasks::get_tasks_for_multiple_technologies, get_technologies::get_all_technologies,
};

pub async fn get_full_content() -> Result<FullContent> {
    let technologies = get_all_technologies().await?;
    let technologies_ids = technologies.iter().map(|t| t.id.clone()).collect();

    let sections = get_sections_for_multiple_technologies(technologies_ids).await?;
    let sections_ids = sections.iter().map(|s| s.id.clone()).collect();

    let tasks = get_tasks_for_multiple_technologies(sections_ids).await?;

    Ok(FullContent {
        technologies,
        sections,
        tasks,
    })
}
