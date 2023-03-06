use common_api::lambda::register_handler::register_handler;
use lambda_http::Error;

mod handler;

#[tokio::main]
async fn main() -> Result<(), Error> {
    register_handler(handler::handle_request).await
}
