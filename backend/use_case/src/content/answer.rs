use std::collections::HashMap;

use common_domain::{
    define_repo,
    error::{Error, ErrorOutput, ErrorType, Result},
};
use content_domain::model::{
    answer::Answer,
    answer_to_save::AnswerToSave,
    answer_validation_result::AnswerValidationResult,
    historical_answer::{HistoricalAnswer, VecHistoricalAnswerExt},
    task::Task,
};
use tokio::join;

define_repo! {
    pub struct AnswerRepository<A, B, C> {
        pub get_task: Fn(task_id: String) -> Result<Option<Task>> as A,
        pub get_previous_answers: Fn(user_id: String, task_id: String) -> Result<Vec<HistoricalAnswer>> as B,
        pub save_answer: Fn(answer: AnswerToSave) -> Result<()> as C,
    }
}

pub async fn answer<A, B, C>(
    user_id: String,
    answer: Answer,
    repo: AnswerRepository<A, B, C>,
) -> Result<AnswerValidationResult>
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
    let Some(task) = task? else {
        return Err(task_not_found_error());
    };
    let previous_answers = previous_answers?;
    let is_valid = answer.is_valid_for(&task)?;
    let had_valid_answer_before = previous_answers.had_valid_answer();

    (repo.save_answer)(AnswerToSave {
        user_id,
        is_valid,
        had_valid_answer_before,
        answer,
    })
    .await?;

    Ok(AnswerValidationResult {
        is_valid,
        had_valid_answer_before,
    })
}

fn task_not_found_error() -> Error {
    Error {
        debug_message: "Task not found".to_string(),
        error_type: ErrorType::NotFound,
        output: Box::new(ErrorOutput {
            message: "Task not found".to_string(),
            code: "not_found".to_owned(),
            args: HashMap::new(),
        }),
    }
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

        assert_eq!(result, Err(task_not_found_error()));
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
                had_valid_answer_before: false,
                is_valid: true,
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

        assert_eq!(
            result,
            Ok(AnswerValidationResult {
                is_valid: true,
                had_valid_answer_before: false,
            })
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
                    was_valid: true,
                    ..Default::default()
                }])
            })
            .once();
        let (ctx, _save_answer_lock) = mock_save_answer::ctx().await;
        ctx.expect()
            .with(predicate::eq(AnswerToSave {
                user_id: "user_id".to_owned(),
                had_valid_answer_before: true,
                is_valid: true,
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

        assert_eq!(
            result,
            Ok(AnswerValidationResult {
                is_valid: true,
                had_valid_answer_before: true,
            })
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
                had_valid_answer_before: false,
                is_valid: false,
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

        assert_eq!(
            result,
            Ok(AnswerValidationResult {
                is_valid: false,
                had_valid_answer_before: false,
            })
        );
    }
}
