/*
 * just-code-dev
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2023-03-10 06:01:08UTC
 *
 * Generated by: https://openapi-generator.tech
 */

use std::borrow::Borrow;
#[allow(unused_imports)]
use std::option::Option;
use std::pin::Pin;
use std::rc::Rc;

use futures::Future;
use hyper;

use super::request as __internal_request;
use super::{configuration, Error};

pub struct ProfileApiClient<C: hyper::client::connect::Connect>
where
    C: Clone + std::marker::Send + Sync + 'static,
{
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::connect::Connect> ProfileApiClient<C>
where
    C: Clone + std::marker::Send + Sync,
{
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> ProfileApiClient<C> {
        ProfileApiClient { configuration }
    }
}

pub trait ProfileApi {
    fn v1_profile_current_avatar_upload_url_get(
        &self,
    ) -> Pin<Box<dyn Future<Output = Result<crate::models::PresignedUrlDto, Error>>>>;
    fn v1_profile_current_get(
        &self,
    ) -> Pin<Box<dyn Future<Output = Result<crate::models::ProfileDto, Error>>>>;
    fn v1_profile_current_push_delete(&self) -> Pin<Box<dyn Future<Output = Result<(), Error>>>>;
    fn v1_profile_current_push_put(
        &self,
        push_data_dto: crate::models::PushDataDto,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>>>>;
    fn v1_profile_current_put(
        &self,
        update_profile_dto: crate::models::UpdateProfileDto,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>>>>;
}

impl<C: hyper::client::connect::Connect> ProfileApi for ProfileApiClient<C>
where
    C: Clone + std::marker::Send + Sync,
{
    #[allow(unused_mut)]
    fn v1_profile_current_avatar_upload_url_get(
        &self,
    ) -> Pin<Box<dyn Future<Output = Result<crate::models::PresignedUrlDto, Error>>>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::GET,
            "/v1/profile/current/avatar/upload-url".to_string(),
        )
        .with_auth(__internal_request::Auth::Oauth);

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn v1_profile_current_get(
        &self,
    ) -> Pin<Box<dyn Future<Output = Result<crate::models::ProfileDto, Error>>>> {
        let mut req =
            __internal_request::Request::new(hyper::Method::GET, "/v1/profile/current".to_string())
                .with_auth(__internal_request::Auth::Oauth);

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn v1_profile_current_push_delete(&self) -> Pin<Box<dyn Future<Output = Result<(), Error>>>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::DELETE,
            "/v1/profile/current/push".to_string(),
        )
        .with_auth(__internal_request::Auth::Oauth);
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn v1_profile_current_push_put(
        &self,
        push_data_dto: crate::models::PushDataDto,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>>>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::PUT,
            "/v1/profile/current/push".to_string(),
        )
        .with_auth(__internal_request::Auth::Oauth);
        req = req.with_body_param(push_data_dto);
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn v1_profile_current_put(
        &self,
        update_profile_dto: crate::models::UpdateProfileDto,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>>>> {
        let mut req =
            __internal_request::Request::new(hyper::Method::PUT, "/v1/profile/current".to_string())
                .with_auth(__internal_request::Auth::Oauth);
        req = req.with_body_param(update_profile_dto);
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }
}
