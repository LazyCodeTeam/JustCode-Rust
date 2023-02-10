use std::collections::HashMap;

use common_domain::error::{Error, ErrorOutput, ErrorType, Result};

pub trait Validate<T>
where
    Self: Sized,
{
    fn validate(item: Self) -> Result<Self>;
}

pub trait Validated<T> {
    fn validated(self) -> Result<T>;
}

impl<T> Validate<T> for T
where
    T: validator::Validate,
{
    fn validate(item: T) -> Result<T> {
        item.validate().map_err(|e| {
            let args: HashMap<String, String> = e
                .errors()
                .iter()
                .flat_map(|(key, value)| {
                    serde_json::to_string(&value).map(|v| ((*key).to_owned(), v))
                })
                .collect();

            Error {
                debug_message: format!("Validation error: {e:?}"),
                error_type: ErrorType::InvalidInput,
                output: Box::new(ErrorOutput {
                    message: "Validation failure".to_owned(),
                    code: "validation_failure".to_owned(),
                    args,
                }),
            }
        })?;

        Ok(item)
    }
}

impl<T> Validate<T> for Vec<T>
where
    T: Validate<T>,
{
    fn validate(items: Vec<T>) -> Result<Vec<T>> {
        items.into_iter().map(|item| T::validate(item)).collect()
    }
}

impl<T> Validated<T> for T
where
    T: Validate<T>,
{
    fn validated(self) -> Result<Self> {
        T::validate(self)
    }
}
