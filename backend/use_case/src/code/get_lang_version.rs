use code_domain::port::GetVersion;
use common_domain::error::Result;

pub async fn get_lang_version<A>(get_version: A) -> Result<String>
where
    A: GetVersion,
{
    get_version().await
}
