use common_domain::{define_repo, error::Result};
use content_domain::model::technology::Technology;

define_repo! {
    pub struct GetPublicTechnologiesRepo<A> {
        pub get_technologies: Fn() -> Result<Vec<Technology>> as A,
    }
}

pub async fn get_public_technologies<A>(
    repo: GetPublicTechnologiesRepo<A>,
) -> Result<Vec<Technology>>
where
    A: GetTechnologiesType,
{
    (repo.get_technologies)().await
}
