use common_domain::{
    define_repo,
    error::{Error, Result, ResultLogExt},
};
use content_domain::{
    model::{
        historical_answer::{HistoricalAnswer, VecHistoricalAnswerExt},
        personalized_section::PersonalizedSection,
        section::Section,
    },
    Personalize,
};
use snafu::{ResultExt, Snafu};
use tokio::join;

define_repo! {
    pub struct GetSectionsRepo<A, B> {
        pub get_sections: Fn(technology: String) -> Result<Vec<Section>> as A,
        pub get_valid_historical_answers: Fn(user_id: String) -> Result<Vec<HistoricalAnswer>> as B,
    }
}

#[derive(Debug, Snafu)]
pub enum GetSectionsError {
    #[snafu(display("Not found"))]
    NotFound,
    Infra {
        source: Error,
    },
}

pub async fn get_sections<A, B>(
    technology_id: String,
    user_id: String,
    repo: GetSectionsRepo<A, B>,
) -> std::result::Result<Vec<PersonalizedSection>, GetSectionsError>
where
    A: GetSectionsType,
    B: GetValidHistoricalAnswersType,
{
    let (sections, valid_historical_answers) = join!(
        (repo.get_sections)(technology_id),
        (repo.get_valid_historical_answers)(user_id)
    );
    let sections = sections.context(InfraSnafu)?;
    let answered_tasks = valid_historical_answers
        .context(InfraSnafu)?
        .into_answer_per_task_id();

    if sections.is_empty() {
        return Err(GetSectionsError::NotFound).with_debug_log();
    }

    Ok(sections.personalize(&answered_tasks))
}
