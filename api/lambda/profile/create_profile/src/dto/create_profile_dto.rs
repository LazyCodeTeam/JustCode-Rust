use aws_lambda_events::cognito::CognitoEventUserPoolsPostConfirmation;
use common_domain::error::Error;
use profile_domain::model::create_profile_params::CreateProfileParams;
use snafu::OptionExt;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CreateProfileDto {
    pub name: String,
    pub email: String,
    pub id: String,
}

impl TryFrom<CognitoEventUserPoolsPostConfirmation> for CreateProfileDto {
    type Error = Error;

    fn try_from(value: CognitoEventUserPoolsPostConfirmation) -> Result<Self, Self::Error> {
        Ok(CreateProfileDto {
            name: value
                .cognito_event_user_pools_header
                .user_name
                .whatever_context("user_name not found")?,
            email: value
                .request
                .user_attributes
                .get("email")
                .whatever_context("user_attributes.email is not found")?
                .to_owned(),
            id: value
                .request
                .user_attributes
                .get("sub")
                .whatever_context("user_attributes.sub is not found")?
                .replace('-', ""),
        })
    }
}

impl From<CreateProfileDto> for CreateProfileParams {
    fn from(dto: CreateProfileDto) -> Self {
        CreateProfileParams {
            id: dto.id,
            name: dto.name,
            email: dto.email,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use aws_lambda_events::cognito::{
        CognitoEventUserPoolsHeader, CognitoEventUserPoolsPostConfirmationRequest,
        CognitoEventUserPoolsPostConfirmationResponse,
    };

    use super::*;

    #[test]
    fn test_from_to_model() {
        let dto = CreateProfileDto {
            name: "name".to_owned(),
            email: "email".to_owned(),
            id: "id".to_owned(),
        };
        let params = CreateProfileParams::from(dto);
        assert_eq!(
            params,
            CreateProfileParams {
                id: "id".to_owned(),
                name: "name".to_owned(),
                email: "email".to_owned(),
            }
        );
    }

    #[test]
    fn test_try_from_event_to_dto() {
        let event = CognitoEventUserPoolsPostConfirmation {
            response: CognitoEventUserPoolsPostConfirmationResponse {},
            request: CognitoEventUserPoolsPostConfirmationRequest {
                user_attributes: HashMap::from([
                    ("email".to_owned(), "email".to_owned()),
                    ("sub".to_owned(), "id".to_owned()),
                ]),
                ..Default::default()
            },
            cognito_event_user_pools_header: CognitoEventUserPoolsHeader {
                user_name: Some("name".to_owned()),
                ..Default::default()
            },
        };

        let dto = CreateProfileDto::try_from(event).unwrap();

        assert_eq!(
            dto,
            CreateProfileDto {
                name: "name".to_owned(),
                email: "email".to_owned(),
                id: "id".to_owned(),
            }
        );
    }

    #[test]
    fn test_try_from_event_to_dto_no_user_name() {
        let event = CognitoEventUserPoolsPostConfirmation {
            response: CognitoEventUserPoolsPostConfirmationResponse {},
            request: CognitoEventUserPoolsPostConfirmationRequest {
                user_attributes: HashMap::from([
                    ("email".to_owned(), "email".to_owned()),
                    ("sub".to_owned(), "id".to_owned()),
                ]),
                ..Default::default()
            },
            cognito_event_user_pools_header: CognitoEventUserPoolsHeader {
                user_name: None,
                ..Default::default()
            },
        };

        let dto = CreateProfileDto::try_from(event);

        assert!(dto.is_err());
    }

    #[test]
    fn test_try_from_event_to_dto_no_email() {
        let event = CognitoEventUserPoolsPostConfirmation {
            response: CognitoEventUserPoolsPostConfirmationResponse {},
            request: CognitoEventUserPoolsPostConfirmationRequest {
                user_attributes: HashMap::from([("sub".to_owned(), "id".to_owned())]),
                ..Default::default()
            },
            cognito_event_user_pools_header: CognitoEventUserPoolsHeader {
                user_name: Some("name".to_owned()),
                ..Default::default()
            },
        };

        let dto = CreateProfileDto::try_from(event);

        assert!(dto.is_err());
    }

    #[test]
    fn test_try_from_event_to_dto_no_id() {
        let event = CognitoEventUserPoolsPostConfirmation {
            response: CognitoEventUserPoolsPostConfirmationResponse {},
            request: CognitoEventUserPoolsPostConfirmationRequest {
                user_attributes: HashMap::from([("email".to_owned(), "email".to_owned())]),
                ..Default::default()
            },
            cognito_event_user_pools_header: CognitoEventUserPoolsHeader {
                user_name: Some("name".to_owned()),
                ..Default::default()
            },
        };

        let dto = CreateProfileDto::try_from(event);

        assert!(dto.is_err());
    }
}
