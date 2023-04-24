use common_domain::{
    define_repo,
    error::{Error, Result},
};
use content_domain::model::{
    historical_answer::{HistoricalAnswer, VecHistoricalAnswerExt},
    personalized_task::PersonalizedTask,
    task::{Task, VecTaskExt},
};
use tokio::join;

define_repo! {
    pub struct GetTasksRepo<A, B> {
        pub get_tasks: Fn(section_id: String) -> Result<Vec<Task>> as A,
        pub get_valid_historical_answers: Fn(task_id: String) -> Result<Vec<HistoricalAnswer>> as B,
    }
}

pub async fn get_tasks<A, B>(
    section_id: String,
    user_id: String,
    repo: GetTasksRepo<A, B>,
) -> Result<Vec<PersonalizedTask>>
where
    A: GetTasksType,
    B: GetValidHistoricalAnswersType,
{
    let (tasks, valid_historical_answers) = join!(
        (repo.get_tasks)(section_id),
        (repo.get_valid_historical_answers)(user_id)
    );
    let tasks = tasks?;
    let answered_tasks = valid_historical_answers?.into_answer_per_task_id();

    if tasks.is_empty() {
        return Err(Error::not_found());
    }

    Ok(tasks.personalize(answered_tasks))
}

#[cfg(test)]
mod tests {
    use mockall::predicate::*;

    use super::*;

    #[tokio::test]
    async fn get_tasks_test() {
        let now = chrono::Utc::now();
        let (ctx, _get_tasks_lock) = mock_get_tasks::ctx().await;
        ctx.expect()
            .with(eq("section_id".to_owned()))
            .returning(|_| {
                Ok(vec![
                    Task {
                        id: "id".to_owned(),
                        ..Default::default()
                    },
                    Task {
                        id: "id_2".to_owned(),
                        ..Default::default()
                    },
                ])
            })
            .once();
        let (ctx, _get_valid_historical_answers_lock) =
            mock_get_valid_historical_answers::ctx().await;
        ctx.expect()
            .with(eq("user_id".to_owned()))
            .returning(move |_| {
                Ok(vec![HistoricalAnswer {
                    task_id: "id".to_owned(),
                    created_at: now,
                    ..Default::default()
                }])
            })
            .once();

        let repo = GetTasksRepo {
            get_tasks: mock_get_tasks::call,
            get_valid_historical_answers: mock_get_valid_historical_answers::call,
        };

        let result = get_tasks("section_id".to_owned(), "user_id".to_owned(), repo).await;

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            vec![
                PersonalizedTask {
                    id: "id".to_owned(),
                    done_at: Some(now),
                    ..Default::default()
                },
                PersonalizedTask {
                    id: "id_2".to_owned(),
                    ..Default::default()
                },
            ]
        );
    }
}
