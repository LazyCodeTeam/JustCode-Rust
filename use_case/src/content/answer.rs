use common_domain::{
    define_repo,
    error::{Error, Result, ResultLogExt},
};
use content_domain::model::{
    answer::Answer,
    answer_result::AnswerResult,
    answer_to_save::AnswerToSave,
    answer_validation_result::AnswerValidationResult,
    historical_answer::{HistoricalAnswer, VecHistoricalAnswerExt},
    task::Task,
};
use snafu::{ResultExt, Snafu};
use tokio::join;

define_repo! {
    pub struct AnswerRepository<A, B, C> {
        pub get_task: Fn(task_id: String) -> Result<Option<Task>> as A,
        pub get_previous_answers: Fn(user_id: String, task_id: String) -> Result<Vec<HistoricalAnswer>> as B,
        pub save_answer: Fn(answer: AnswerToSave) -> Result<()> as C,
    }
}

#[derive(Debug, Snafu)]
pub enum AnswerError {
    #[snafu(display("Task not found: {task_id}"))]
    TaskNotFound {
        task_id: String,
    },
    InvalidAnswerType {
        source: Error,
    },
    Infra {
        source: Error,
    },
}

pub async fn answer<A, B, C>(
    user_id: String,
    answer: Answer,
    repo: AnswerRepository<A, B, C>,
) -> std::result::Result<AnswerValidationResult, AnswerError>
where
    A: GetTaskType,
    B: GetPreviousAnswersType,
    C: SaveAnswerType,
{
    // TODO: validate if user can answer
    let (task, previous_answers) = join!(
        (repo.get_task)(answer.task_id.clone()),
        (repo.get_previous_answers)(user_id.clone(), answer.task_id.clone())
    );
    let Some(task) = task.context(InfraSnafu)? else {
        return Err(AnswerError::TaskNotFound {
            task_id: answer.task_id,
        }).with_debug_log();
    };
    let previous_answers = previous_answers.context(InfraSnafu)?;
    let is_valid = answer.is_valid_for(&task).context(InvalidAnswerTypeSnafu)?;
    let had_valid_answer_before = previous_answers.had_valid_answer();
    let result = AnswerResult::new(is_valid, had_valid_answer_before);

    (repo.save_answer)(AnswerToSave {
        user_id,
        result,
        answer,
    })
    .await
    .context(InfraSnafu)?;

    Ok(AnswerValidationResult { result })
}

#[cfg(test)]
mod tests {
    use content_domain::model::{answer_content::AnswerContent, task_content::TaskContent};
    use mockall::predicate;

    use super::*;

    #[tokio::test]
    async fn fail_task_not_found() {
        let (ctx, _get_task_lock) = mock_get_task::ctx().await;
        ctx.expect()
            .with(predicate::eq("task_id".to_owned()))
            .returning(|_| Ok(None))
            .once();

        let (ctx, _get_previous_answers_lock) = mock_get_previous_answers::ctx().await;
        ctx.expect()
            .with(
                predicate::eq("user_id".to_owned()),
                predicate::eq("task_id".to_owned()),
            )
            .returning(|_, _| Ok(Vec::new()))
            .once();
        let (ctx, _save_answer_lock) = mock_save_answer::ctx().await;
        ctx.expect().returning(|_| Ok(())).times(0);

        let repo = AnswerRepository {
            get_task: mock_get_task::call,
            get_previous_answers: mock_get_previous_answers::call,
            save_answer: mock_save_answer::call,
        };

        let result = answer(
            "user_id".to_owned(),
            Answer {
                task_id: "task_id".to_owned(),
                ..Default::default()
            },
            repo,
        )
        .await;

        assert!(result.is_err());
        assert!(match result.unwrap_err() {
            AnswerError::TaskNotFound { task_id } => task_id == "task_id",
            _ => false,
        })
    }

    #[tokio::test]
    async fn success_first_valid_answer() {
        let (ctx, _get_task_lock) = mock_get_task::ctx().await;
        ctx.expect()
            .with(predicate::eq("task_id".to_owned()))
            .returning(|_| {
                Ok(Some(Task {
                    content: TaskContent::Lesson {
                        content: "".to_owned(),
                    },
                    ..Default::default()
                }))
            })
            .once();

        let (ctx, _get_previous_answers_lock) = mock_get_previous_answers::ctx().await;
        ctx.expect()
            .with(
                predicate::eq("user_id".to_owned()),
                predicate::eq("task_id".to_owned()),
            )
            .returning(|_, _| Ok(Vec::new()))
            .once();
        let (ctx, _save_answer_lock) = mock_save_answer::ctx().await;
        ctx.expect()
            .with(predicate::eq(AnswerToSave {
                user_id: "user_id".to_owned(),
                result: AnswerResult::new(true, false),
                answer: Answer {
                    task_id: "task_id".to_owned(),
                    ..Default::default()
                },
            }))
            .returning(|_| Ok(()))
            .once();

        let repo = AnswerRepository {
            get_task: mock_get_task::call,
            get_previous_answers: mock_get_previous_answers::call,
            save_answer: mock_save_answer::call,
        };

        let result = answer(
            "user_id".to_owned(),
            Answer {
                task_id: "task_id".to_owned(),
                ..Default::default()
            },
            repo,
        )
        .await;

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            AnswerValidationResult {
                result: AnswerResult::Valid,
            }
        );
    }

    #[tokio::test]
    async fn success_not_first_valid_answer() {
        let (ctx, _get_task_lock) = mock_get_task::ctx().await;
        ctx.expect()
            .with(predicate::eq("task_id".to_owned()))
            .returning(|_| {
                Ok(Some(Task {
                    content: TaskContent::Lesson {
                        content: "".to_owned(),
                    },
                    ..Default::default()
                }))
            })
            .once();

        let (ctx, _get_previous_answers_lock) = mock_get_previous_answers::ctx().await;
        ctx.expect()
            .with(
                predicate::eq("user_id".to_owned()),
                predicate::eq("task_id".to_owned()),
            )
            .returning(|_, _| {
                Ok(vec![HistoricalAnswer {
                    result: AnswerResult::Valid,
                    ..Default::default()
                }])
            })
            .once();
        let (ctx, _save_answer_lock) = mock_save_answer::ctx().await;
        ctx.expect()
            .with(predicate::eq(AnswerToSave {
                user_id: "user_id".to_owned(),
                result: AnswerResult::new(true, true),
                answer: Answer {
                    task_id: "task_id".to_owned(),
                    ..Default::default()
                },
            }))
            .returning(|_| Ok(()))
            .once();

        let repo = AnswerRepository {
            get_task: mock_get_task::call,
            get_previous_answers: mock_get_previous_answers::call,
            save_answer: mock_save_answer::call,
        };

        let result = answer(
            "user_id".to_owned(),
            Answer {
                task_id: "task_id".to_owned(),
                ..Default::default()
            },
            repo,
        )
        .await;

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            AnswerValidationResult {
                result: AnswerResult::AgainValid,
            }
        );
    }

    #[tokio::test]
    async fn success_first_invalid_answer() {
        let (ctx, _get_task_lock) = mock_get_task::ctx().await;
        ctx.expect()
            .with(predicate::eq("task_id".to_owned()))
            .returning(|_| {
                Ok(Some(Task {
                    content: TaskContent::SingleSelection {
                        content: "".to_owned(),
                        options: vec![],
                        correct_option: 1,
                        hints: vec![],
                    },
                    ..Default::default()
                }))
            })
            .once();

        let (ctx, _get_previous_answers_lock) = mock_get_previous_answers::ctx().await;
        ctx.expect()
            .with(
                predicate::eq("user_id".to_owned()),
                predicate::eq("task_id".to_owned()),
            )
            .returning(|_, _| Ok(Vec::new()))
            .once();
        let (ctx, _save_answer_lock) = mock_save_answer::ctx().await;
        ctx.expect()
            .with(predicate::eq(AnswerToSave {
                user_id: "user_id".to_owned(),
                result: AnswerResult::new(false, false),
                answer: Answer {
                    task_id: "task_id".to_owned(),
                    content: AnswerContent::SingleAnswer { answer: 2 },
                },
            }))
            .returning(|_| Ok(()))
            .once();

        let repo = AnswerRepository {
            get_task: mock_get_task::call,
            get_previous_answers: mock_get_previous_answers::call,
            save_answer: mock_save_answer::call,
        };

        let result = answer(
            "user_id".to_owned(),
            Answer {
                task_id: "task_id".to_owned(),
                content: AnswerContent::SingleAnswer { answer: 2 },
            },
            repo,
        )
        .await;

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            AnswerValidationResult {
                result: AnswerResult::Invalid,
            }
        );
    }
}
