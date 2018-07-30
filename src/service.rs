use api::PublicApi;
use schema::VotingSchema;
use transactions::VotingTransactions;

use exonum::{
    api::ServiceApiBuilder,
    blockchain::{Transaction, TransactionSet, Service},
    crypto::Hash, encoding::Error as StreamStructError,
    messages::RawTransaction, storage::Snapshot,
};

pub const VOTING_SERVICE: u16 = 13;
pub const SERVICE_NAME: &str = "voting";

pub struct VotingService;

impl Service for VotingService {
    fn service_id(&self) -> u16 {
        VOTING_SERVICE
    }

    fn service_name(&self) -> &str {
        SERVICE_NAME
    }

    fn state_hash(&self, snapshot: &Snapshot) -> Vec<Hash> {
        let schema = VotingSchema::new(snapshot);
        schema.state_hash()
    }

    fn tx_from_raw(&self, raw: RawTransaction) -> Result<Box<Transaction>, StreamStructError> {
        let tx = VotingTransactions::tx_from_raw(raw)?;
        Ok(tx.into())
    }

    fn wire_api(&self, builder: &mut ServiceApiBuilder) {
        PublicApi::wire(builder);
    }
}