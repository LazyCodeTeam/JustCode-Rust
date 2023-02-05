use common_domain::error::Result;
use git_domain::model::git_hook_event::GitHookEvent;

pub struct LoadTasksRepository {}

pub async fn load_tasks(_event: GitHookEvent, _repo: LoadTasksRepository) -> Result<()> {
    Ok(())
}
