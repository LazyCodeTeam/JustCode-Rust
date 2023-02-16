use common_domain::{define_repo, error::Result};

define_repo! {
    pub struct LoadTasksRepository {}
}

pub async fn load_tasks(_repo: LoadTasksRepository) -> Result<()> {
    Ok(())
}
