use crate::config::CONFIG;
use common_domain::error::Result;
use common_infra::cognito_client::get_cognito_client;
use snafu::ResultExt;

pub async fn delete_user_by_username(username: impl Into<String>) -> Result<()> {
    get_cognito_client()
        .await
        .admin_delete_user()
        .username(username)
        .user_pool_id(&CONFIG.user_pool_id)
        .send()
        .await
        .map(|_| ())
        .whatever_context("Failed to delete user by username")
}
