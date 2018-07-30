#[macro_use]
extern crate serde_json;

extern crate exonum_testkit;
extern crate exonum;
extern crate voting_service as voting;

use exonum::{
    api::node::public::explorer::TransactionQuery,
    crypto::{self, CryptoHash, Hash, PublicKey, SecretKey},
};

use exonum_testkit::{ApiKind, TestKit, TestKitApi, TestKitBuilder};
use voting::{
    api::{CandidateQuery, ElectorQuery, VotingResults},
    service::{VotingService, SERVICE_NAME},
    transactions::{CreateCandidate, CreateElector, Vote},
    schema::{Candidate, Elector},
};

#[test]
fn create_candidate() {
    let (mut testkit, api) = create_testkit();
    let (john_tx, _) = api.create_candidate("John");
    let (john_1_tx, _) = api.create_candidate("John_1");
    testkit.create_block();

    api.assert_tx_success(john_tx.hash());
    api.assert_tx_success(john_1_tx.hash());

    let john = api.get_candidate(*john_tx.pub_key()).unwrap();
    assert_eq!(john.pub_key(), john_tx.pub_key());
    assert_eq!(john.name(), john_tx.name());
    assert_eq!(john.voices(), 0);

    let john_1 = api.get_candidate(*john_1_tx.pub_key()).unwrap();
    assert_eq!(john_1.pub_key(), john_1_tx.pub_key());
    assert_eq!(john_1.name(), john_1_tx.name());
    assert_eq!(john_1.voices(), 0);

    let candidates = api.get_candidates();
    assert_eq!(2, candidates.len());
    assert_eq!(true, candidates.iter()
        .any(|c| c.pub_key() == john.pub_key() && c.name() == john.name() && john.voices() == 0));
    assert_eq!(true, candidates.iter()
        .any(|c| c.pub_key() == john_1.pub_key() && c.name() == john_1.name() && john_1.voices() == 0));
}

#[test]
fn create_elector() {
    let (mut testkit, api) = create_testkit();
    let (den_tx, _) = api.create_elector("Den");
    testkit.create_block();

    api.assert_tx_success(den_tx.hash());
    let den = api.get_elector(*den_tx.pub_key()).unwrap();

    assert_eq!(den.pub_key(), den_tx.pub_key());
    assert_eq!(den.name(), den_tx.name());
    assert_eq!(den.has_vote(), true);
}

#[test]
fn vote() {
    let (mut testkit, api) = create_testkit();
    let (den_1_tx, den_1_seq) = api.create_elector("Den lee");
    let (den_2_tx, den_2_seq) = api.create_elector("Den_2");
    let (john_1_tx, _) = api.create_candidate("John Forbes Nash");
    let (john_2_tx, _) = api.create_candidate("John_2");
    testkit.create_block();

    api.assert_tx_success(den_1_tx.hash());
    api.assert_tx_success(john_2_tx.hash());

    let vote_1 = api.vote(john_1_tx.pub_key(), den_1_tx.pub_key(), &den_1_seq);
    let vote_2 = api.vote(john_2_tx.pub_key(), den_2_tx.pub_key(), &den_2_seq);
    testkit.create_block();
    api.assert_tx_success(vote_1.hash());
    api.assert_tx_success(vote_2.hash());

    let vote_3 = api.vote(john_1_tx.pub_key(), den_2_tx.pub_key(), &den_2_seq);
    let vote_4 = api.vote(john_2_tx.pub_key(), den_1_tx.pub_key(), &den_1_seq);
    testkit.create_block();

    api.assert_tx_status(vote_3.hash(), &json!({ "type": "error", "code" : 4,  "description": "The voter has already voted."}));
    api.assert_tx_status(vote_4.hash(), &json!({ "type": "error", "code" : 4,  "description": "The voter has already voted."}));

    let john_1 = api.get_candidate(*john_1_tx.pub_key()).unwrap();
    let john_2 = api.get_candidate(*john_2_tx.pub_key()).unwrap();
    assert_eq!(john_1.voices(), 1);
    assert_eq!(john_2.voices(), 1);

    let den_1 = api.get_elector(*den_1_tx.pub_key()).unwrap();
    let den_2 = api.get_elector(*den_2_tx.pub_key()).unwrap();
    assert_eq!(den_1.has_vote(), false);
    assert_eq!(den_2.has_vote(), false);
}

#[test]
fn get_block_number() {
    let (mut testkit, api) = create_testkit();
    let (den_1_tx, den_1_seq) = api.create_elector("Den_1");
    testkit.create_block();
    let (john_1_tx, _) = api.create_candidate("John_1");
    testkit.create_block();
    api.vote(john_1_tx.pub_key(), den_1_tx.pub_key(), &den_1_seq);
    testkit.create_block();

    assert_eq!(Some(3), api.get_block_number(*den_1_tx.pub_key()));
}

#[test]
fn results() {
    let (mut testkit, api) = create_testkit();
    let (den_1_tx, den_1_seq) = api.create_elector("Den_1");
    let (den_2_tx, den_2_seq) = api.create_elector("Den_2");
    let (den_3_tx, den_3_seq) = api.create_elector("Den_3");

    let (john_1_tx, _) = api.create_candidate("John_1");
    let (john_2_tx, _) = api.create_candidate("John_2");
    testkit.create_block();
    api.vote(john_1_tx.pub_key(), den_1_tx.pub_key(), &den_1_seq);
    api.vote(john_2_tx.pub_key(), den_2_tx.pub_key(), &den_2_seq);
    api.vote(john_1_tx.pub_key(), den_3_tx.pub_key(), &den_3_seq);
    testkit.create_block();

    let res = api.get_results();
    assert_eq!(2, res.candidates.len());

    assert_eq!(true, res.candidates.iter()
        .any(|c| c.candidate.name() == "John_1" && c.candidate.voices() == 2));
    assert_eq!(true, res.candidates.iter()
        .any(|c| c.candidate.name() == "John_2" && c.candidate.voices() == 1));
}

struct Api {
    pub inner: TestKitApi,
}

impl Api {
    fn create_candidate(&self, name: &str) -> (CreateCandidate, SecretKey) {
        let (pubkey, key) = crypto::gen_keypair();
        let tx = CreateCandidate::new(&pubkey, name, &key);
        println!("create candidate: {}", serde_json::to_string_pretty(&tx).unwrap());
        let tx_info: serde_json::Value = self.inner
            .public(ApiKind::Service(SERVICE_NAME))
            .query(&tx)
            .post("v1/candidate")
            .unwrap();
        assert_eq!(tx_info, json!({ "tx_hash": tx.hash() }));
        (tx, key)
    }

    fn create_elector(&self, name: &str) -> (CreateElector, SecretKey) {
        let (pubkey, key) = crypto::gen_keypair();
        let tx = CreateElector::new(&pubkey, name, &key);
        println!("create elector: {}", serde_json::to_string_pretty(&tx).unwrap());
        let tx_info: serde_json::Value = self.inner
            .public(ApiKind::Service(SERVICE_NAME))
            .query(&tx)
            .post("v1/elector")
            .unwrap();
        assert_eq!(tx_info, json!({ "tx_hash": tx.hash() }));
        (tx, key)
    }

    fn vote(&self, candidate_pub_key: &PublicKey, elector_pub_key: &PublicKey, elector_sec_key: &SecretKey) -> Vote {
        let tx = Vote::new(&elector_pub_key, candidate_pub_key, elector_sec_key);
        println!("vote elector: {}", serde_json::to_string_pretty(&tx).unwrap());

        let tx_info: serde_json::Value = self.inner
            .public(ApiKind::Service(SERVICE_NAME))
            .query(&tx)
            .post("v1/vote")
            .unwrap();
        assert_eq!(tx_info, json!({ "tx_hash": tx.hash() }));
        tx
    }

    fn assert_tx_status(&self, tx_hash: Hash, expected_status: &serde_json::Value) {
        let info: serde_json::Value = self.inner
            .public(ApiKind::Explorer)
            .query(&TransactionQuery::new(tx_hash))
            .get("v1/transactions")
            .unwrap();

        if let serde_json::Value::Object(mut info) = info {
            let tx_status = info.remove("status").unwrap();
            assert_eq!(tx_status, *expected_status);
        } else {
            panic!("Invalid transaction info format, object expected");
        }
    }

    fn assert_tx_success(&self, tx_hash: Hash) {
        self.assert_tx_status(tx_hash, &json!({ "type": "success" }));
    }

    fn get_elector(&self, pub_key: PublicKey) -> Option<Elector> {
        self.inner
            .public(ApiKind::Service(SERVICE_NAME))
            .query(&ElectorQuery { pub_key })
            .get::<Elector>("v1/elector")
            .ok()
    }

    fn get_block_number(&self, pub_key: PublicKey) -> Option<i32> {
        self.inner
            .public(ApiKind::Service(SERVICE_NAME))
            .query(&ElectorQuery { pub_key })
            .get::<i32>("v1/vote/block")
            .ok()
    }

    fn get_candidate(&self, pub_key: PublicKey) -> Option<Candidate> {
        self.inner
            .public(ApiKind::Service(SERVICE_NAME))
            .query(&CandidateQuery { pub_key })
            .get::<Candidate>("v1/candidate")
            .ok()
    }

    fn get_candidates(&self) -> Vec<Candidate> {
        self.inner
            .public(ApiKind::Service(SERVICE_NAME))
            .get::<Vec<Candidate>>("v1/candidates")
            .unwrap()
    }

    fn get_results(&self) -> VotingResults {
        self.inner
            .public(ApiKind::Service(SERVICE_NAME))
            .get::<VotingResults>("v1/results")
            .unwrap()
    }
}

fn create_testkit() -> (TestKit, Api) {
    let testkit = TestKitBuilder::validator()
        .with_service(VotingService)
        .create();
    let api = Api {
        inner: testkit.api(),
    };
    (testkit, api)
}