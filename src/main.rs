extern crate exonum;
extern crate vote_service;
extern crate exonum_configuration;

use exonum::{
    helpers::fabric::NodeBuilder,
};
use exonum_configuration::ServiceFactory as ConfigurationService;
use vote_service as vote;

fn main() {
    exonum::helpers::init_logger().unwrap();
    exonum::crypto::init();

    let node = NodeBuilder::new()
        .with_service(Box::new(ConfigurationService))
        .with_service(Box::new(vote::ServiceFactory));
    node.run();
}
