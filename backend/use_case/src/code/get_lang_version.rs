use common_domain::{define_repo, error::Result};

define_repo! {
    pub struct GetLangVersionRepository<A> {
        pub get_version: Fn() -> Result<String> as A,
    }
}

pub async fn get_lang_version<A>(repo: GetLangVersionRepository<A>) -> Result<String>
where
    A: GetVersionType,
{
    (repo.get_version)().await
}
