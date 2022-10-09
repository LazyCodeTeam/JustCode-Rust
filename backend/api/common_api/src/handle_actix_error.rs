#[macro_export]
macro_rules! handle_error {
    ($e:expr) => {{
        use actix_web::ResponseError;

        match $e {
            Ok(a) => a,
            Err(e) => {
                return common_api::dto::actix_error_dto::ErrorResponseDto::from(e).error_response()
            }
        }
    }};
}
