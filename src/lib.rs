#[macro_use]
extern crate exonum;
#[macro_use]
extern crate failure;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate exonum_testkit;

use service::SERVICE_NAME;
use exonum::{helpers::fabric::Context, blockchain::Service, helpers::fabric};

pub mod errors;
pub mod service;
pub mod schema;
pub mod api;
pub mod transactions;

#[derive(Debug)]
pub struct ServiceFactory;

impl fabric::ServiceFactory for ServiceFactory {
    fn service_name(&self) -> &str {
        SERVICE_NAME
    }

    fn make_service(&mut self, _: &Context) -> Box<dyn Service> {
        Box::new(service::VotingService)
    }
}