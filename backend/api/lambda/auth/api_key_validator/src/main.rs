mod handler;

use common_api::lambda::register_internal_handler::register_internal_handler;
use lambda_runtime::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    register_internal_handler(handler::handle_request).await
}
