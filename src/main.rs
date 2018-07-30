extern crate exonum;
extern crate voting_service;
extern crate exonum_configuration;

use exonum::{
    helpers::fabric::NodeBuilder,
};
use exonum_configuration::ServiceFactory as ConfigurationService;
use voting_service as voting;

fn main() {
    exonum::helpers::init_logger().unwrap();
    println!("Creating in-memory database...");
    exonum::crypto::init();

    let node = NodeBuilder::new()
        .with_service(Box::new(ConfigurationService))
        .with_service(Box::new(voting::ServiceFactory));
    node.run();
}
