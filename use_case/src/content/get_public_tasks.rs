use common_domain::{
    define_repo,
    error::{Error, Result, ResultLogExt},
};
use content_domain::model::task::Task;
use snafu::{ResultExt, Snafu};

define_repo! {
    pub struct GetPublicTasksRepo<A> {
        pub get_tasks: Fn(section_id: String) -> Result<Vec<content_domain::model::task::Task>> as A,
    }
}

#[derive(Debug, Snafu)]
pub enum GetPublicTasksError {
    #[snafu(display("Tasks not found for section: {section_id}"))]
    NotFound {
        section_id: String,
    },
    Infra {
        source: Error,
    },
}

pub async fn get_public_tasks<A>(
    section_id: String,
    repo: GetPublicTasksRepo<A>,
) -> std::result::Result<Vec<Task>, GetPublicTasksError>
where
    A: GetTasksType,
{
    let tasks = (repo.get_tasks)(section_id.clone())
        .await
        .context(InfraSnafu)?;

    if tasks.is_empty() {
        return Err(GetPublicTasksError::NotFound { section_id }).with_debug_log();
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
            .return_once(move |_| Ok(vec![]));

        let repo = GetPublicTasksRepo {
            get_tasks: mock_get_tasks::call,
        };

        let result = get_public_tasks("section_id".to_string(), repo).await;

        assert!(result.is_err());
        assert!(
            matches!(result.unwrap_err(), GetPublicTasksError::NotFound { section_id } if section_id == "section_id")
        );
    }
}
