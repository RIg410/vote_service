use exonum::{
    api::{self, ServiceApiBuilder, ServiceApiState},
    blockchain::{BlockProof, Transaction, Schema, TransactionSet, Schema as GeneralSchema},
    crypto::{Hash, PublicKey},
    node::TransactionSend,
    storage::{MapProof, ListProof, Snapshot},
    helpers::Height,
    explorer::{BlockchainExplorer, TransactionInfo}
};

use service::VOTING_SERVICE;
use transactions::VotingTransactions;
use schema::{Candidate, Elector, VotingSchema};

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionResponse {
    pub tx_hash: Hash,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CandidateQuery {
    pub pub_key: PublicKey,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ElectorQuery {
    pub pub_key: PublicKey,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VotingHistory {
    pub transactions: Vec<VotingTransactions>,
    pub history_proof: ListProof<Hash>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CandidateInfo {
    pub candidate: Candidate,
    pub vote_percent: f32,
    pub proof: MapProof<PublicKey, Candidate>,
    pub history: VotingHistory,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VotingResults {
    pub candidates: Vec<CandidateInfo>,
    pub block_proof: BlockProof,
    pub to_table: MapProof<Hash, Hash>,
}

#[derive(Debug, Clone, Copy)]
pub struct PublicApi;

impl PublicApi {
    pub fn results(state: &ServiceApiState, _query: ()) -> api::Result<VotingResults> {
        let snapshot = state.snapshot();
        let general_schema = GeneralSchema::new(&snapshot);

        let max_height = general_schema.block_hashes_by_height().len() - 1;

        let schema = VotingSchema::new(state.snapshot());
        let idx = schema.candidates();

        let total_votes_number: u64 = idx.iter()
            .map(|c| c.1.voices()).sum();

        let candidates: Vec<CandidateInfo> = idx.iter()
            .map(|c| Self::get_candidate_info(&schema, &general_schema, c.1, total_votes_number))
            .collect();

        let block_proof = general_schema
            .block_and_precommits(Height(max_height))
            .unwrap();

        let to_table: MapProof<Hash, Hash> =
            general_schema.get_proof_to_service_table(VOTING_SERVICE, 0);

        Ok(VotingResults {
            candidates,
            block_proof,
            to_table,
        })
    }

    fn get_candidate_info(
        schema: &VotingSchema<Box<Snapshot>>,
        general_schema: &GeneralSchema<&Box<Snapshot>>,
        candidate: Candidate,
        total_votes_number: u64,
    ) -> CandidateInfo {
        let history = schema.vote_history(&candidate.pub_key());
        let history_proof = history.get_range_proof(0, history.len());

        let transactions: Vec<VotingTransactions> = history
            .iter()
            .map(|record| general_schema.transactions().get(&record).unwrap())
            .map(|raw| VotingTransactions::tx_from_raw(raw).unwrap())
            .collect::<Vec<_>>();

        let percent = candidate.voices() as f64 / total_votes_number as f64 * 100.0;

        let to_candidate_proof = schema.candidates().get_proof(*candidate.pub_key());

        CandidateInfo {
            candidate,
            history: VotingHistory {
                transactions,
                history_proof,
            },
            vote_percent: percent as f32,
            proof: to_candidate_proof,
        }
    }

    pub fn post_candidate(state: &ServiceApiState, query: VotingTransactions) -> api::Result<TransactionResponse> {
        let transaction: Box<dyn Transaction> = query.into();
        let tx_hash = transaction.hash();
        state.sender().send(transaction)?;
        Ok(TransactionResponse { tx_hash })
    }

    pub fn get_elector(state: &ServiceApiState, query: ElectorQuery) -> api::Result<Elector> {
        let schema = VotingSchema::new(state.snapshot());
        schema.elector(&query.pub_key)
            .ok_or_else(|| api::Error::NotFound("Elector not found".to_owned()))
    }

    pub fn get_candidate(state: &ServiceApiState, query: CandidateQuery) -> api::Result<Candidate> {
        let schema = VotingSchema::new(state.snapshot());
        schema.candidate(&query.pub_key)
            .ok_or_else(|| api::Error::NotFound("Candidate not found".to_owned()))
    }

    pub fn get_block_number(state: &ServiceApiState, query: ElectorQuery) -> api::Result<Height> {
        let core_schema = Schema::new(state.snapshot());
        let tx_list = core_schema.transactions();

        let tx = tx_list.iter()
            .map(|row| (row.0, VotingTransactions::tx_from_raw(row.1)))
            .find(|tx| {
                if let Ok(tx) = &tx.1 {
                    match &tx {
                        VotingTransactions::Vote(vote) => &query.pub_key == vote.elector(),
                        _ => false,
                    }
                } else {
                    false
                }
            }).map(|tx| tx.0)
            .and_then(|hash| BlockchainExplorer::new(state.blockchain()).transaction(&hash))
            .ok_or(api::Error::NotFound("Transaction not found".to_owned()))?;

        if let TransactionInfo::Committed(tx) = tx {
            Ok(tx.location().block_height())
        } else {
            Err(api::Error::NotFound("Block not found".to_owned()))
        }
    }

    pub fn get_candidates(state: &ServiceApiState, _query: ()) -> api::Result<Vec<Candidate>> {
        let snapshot = state.snapshot();
        let schema = VotingSchema::new(snapshot);
        let idx = schema.candidates();
        let candidates = idx.values().collect();
        Ok(candidates)
    }

    pub fn wire(builder: &mut ServiceApiBuilder) {
        builder.public_scope()
            .endpoint("v1/results", Self::results)
            .endpoint("v1/elector", Self::get_elector)
            .endpoint("v1/candidate", Self::get_candidate)
            .endpoint("v1/candidates", Self::get_candidates)
            .endpoint("v1/vote/block", Self::get_block_number)
            .endpoint_mut("v1/candidate", Self::post_candidate)
            .endpoint_mut("v1/elector", Self::post_candidate)
            .endpoint_mut("v1/vote", Self::post_candidate);
    }
}