use common_domain::{
    define_repo,
    error::{Error, Result},
};
use content_domain::model::technology::Technology;
use snafu::{ResultExt, Snafu};

define_repo! {
    pub struct GetPublicTechnologiesRepo<A> {
        pub get_technologies: Fn() -> Result<Vec<Technology>> as A,
    }
}

#[derive(Debug, Snafu)]
pub enum GetPublicTechnologiesError {
    Infra { source: Error },
}

pub async fn get_public_technologies<A>(
    repo: GetPublicTechnologiesRepo<A>,
) -> std::result::Result<Vec<Technology>, GetPublicTechnologiesError>
where
    A: GetTechnologiesType,
{
    (repo.get_technologies)().await.context(InfraSnafu)
}
