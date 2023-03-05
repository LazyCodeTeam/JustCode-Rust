use common_domain::{
    define_repo,
    error::{Error, Result},
};
use content_domain::model::task::Task;

define_repo! {
    pub struct GetPublicTasksRepo<A> {
        pub get_tasks: Fn<'a>(section_id: &'a str) -> Result<Vec<content_domain::model::task::Task>> as A,
    }
}

pub async fn get_public_tasks<A>(
    section_id: String,
    repo: GetPublicTasksRepo<A>,
) -> Result<Vec<Task>>
where
    A: GetTasksType,
{
    let tasks = (repo.get_tasks)(&section_id).await?;

    if tasks.is_empty() {
        return Err(Error::not_found());
    }

    Ok(tasks)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn get_tasks_found() {
        let tasks = vec![Task::default()];

        let (ctx, _get_tasks_lock) = mock_get_tasks::ctx().await;
        let out = tasks.clone();
        ctx.expect()
            .once()
            .withf(|id| "section_id" == id)
            .return_once(move |_| Ok(out));

        let repo = GetPublicTasksRepo {
            get_tasks: mock_get_tasks::call,
        };

        let result = get_public_tasks("section_id".to_string(), repo).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), tasks);
    }

    #[tokio::test]
    async fn get_tasks_not_found() {
        let (ctx, _get_tasks_lock) = mock_get_tasks::ctx().await;
        ctx.expect()
            .once()
            .withf(|id| "section_id" == id)
            .return_once(move |_| Err(Error::not_found()));

        let repo = GetPublicTasksRepo {
            get_tasks: mock_get_tasks::call,
        };

        let result = get_public_tasks("section_id".to_string(), repo).await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), Error::not_found());
    }
}
