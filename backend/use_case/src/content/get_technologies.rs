use common_domain::{define_repo, error::Result};
use content_domain::model::technology::Technology;

define_repo! {
    pub struct GetTechnologiesRepo<A> {
        pub get_all_technologies: Fn() -> Result<Vec<Technology>> as A,
    }
}

pub async fn get_technologies<A>(repo: GetTechnologiesRepo<A>) -> Result<Vec<Technology>>
where
    A: GetAllTechnologiesType,
{
    (repo.get_all_technologies)().await
}
