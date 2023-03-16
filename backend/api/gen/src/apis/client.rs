use std::rc::Rc;

use super::configuration::Configuration;
use hyper;

pub struct APIClient {
    content_api: Box<dyn crate::apis::ContentApi>,
    profile_api: Box<dyn crate::apis::ProfileApi>,
}

impl APIClient {
    pub fn new<C: hyper::client::connect::Connect>(configuration: Configuration<C>) -> APIClient
    where
        C: Clone + std::marker::Send + Sync + 'static,
    {
        let rc = Rc::new(configuration);

        APIClient {
            content_api: Box::new(crate::apis::ContentApiClient::new(rc.clone())),
            profile_api: Box::new(crate::apis::ProfileApiClient::new(rc)),
        }
    }

    pub fn content_api(&self) -> &dyn crate::apis::ContentApi {
        self.content_api.as_ref()
    }

    pub fn profile_api(&self) -> &dyn crate::apis::ProfileApi {
        self.profile_api.as_ref()
    }
}
