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

pub struct ContentApiClient<C: hyper::client::connect::Connect>
where
    C: Clone + std::marker::Send + Sync + 'static,
{
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::connect::Connect> ContentApiClient<C>
where
    C: Clone + std::marker::Send + Sync,
{
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> ContentApiClient<C> {
        ContentApiClient { configuration }
    }
}

pub trait ContentApi {
    fn v1_content_load_put(
        &self,
        expected_technology_dto: Vec<crate::models::ExpectedTechnologyDto>,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>>>>;
    fn v1_content_public_section_section_id_tasks_get(
        &self,
        section_id: &str,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<crate::models::PublicTaskDto>, Error>>>>;
    fn v1_content_public_technologies_get(
        &self,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<crate::models::TechnologyDto>, Error>>>>;
    fn v1_content_public_technology_technology_id_sections_get(
        &self,
        technology_id: &str,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<crate::models::SectionDto>, Error>>>>;
}

impl<C: hyper::client::connect::Connect> ContentApi for ContentApiClient<C>
where
    C: Clone + std::marker::Send + Sync,
{
    #[allow(unused_mut)]
    fn v1_content_load_put(
        &self,
        expected_technology_dto: Vec<crate::models::ExpectedTechnologyDto>,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>>>> {
        let mut req =
            __internal_request::Request::new(hyper::Method::PUT, "/v1/content/load".to_string())
                .with_auth(__internal_request::Auth::ApiKey(
                    __internal_request::ApiKey {
                        in_header: true,
                        in_query: false,
                        param_name: "X-Api-Key".to_owned(),
                    },
                ));
        req = req.with_body_param(expected_technology_dto);
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn v1_content_public_section_section_id_tasks_get(
        &self,
        section_id: &str,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<crate::models::PublicTaskDto>, Error>>>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::GET,
            "/v1/content/public/section/{section_id}/tasks".to_string(),
        )
        .with_auth(__internal_request::Auth::ApiKey(
            __internal_request::ApiKey {
                in_header: true,
                in_query: false,
                param_name: "X-Api-Key".to_owned(),
            },
        ));
        req = req.with_path_param("section_id".to_string(), section_id.to_string());

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn v1_content_public_technologies_get(
        &self,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<crate::models::TechnologyDto>, Error>>>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::GET,
            "/v1/content/public/technologies".to_string(),
        )
        .with_auth(__internal_request::Auth::ApiKey(
            __internal_request::ApiKey {
                in_header: true,
                in_query: false,
                param_name: "X-Api-Key".to_owned(),
            },
        ));

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn v1_content_public_technology_technology_id_sections_get(
        &self,
        technology_id: &str,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<crate::models::SectionDto>, Error>>>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::GET,
            "/v1/content/public/technology/{technology_id}/sections".to_string(),
        )
        .with_auth(__internal_request::Auth::ApiKey(
            __internal_request::ApiKey {
                in_header: true,
                in_query: false,
                param_name: "X-Api-Key".to_owned(),
            },
        ));
        req = req.with_path_param("technology_id".to_string(), technology_id.to_string());

        req.execute(self.configuration.borrow())
    }
}
