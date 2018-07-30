extern crate exonum;
extern crate voting_service as voting;
extern crate exonum_testkit;

use exonum::crypto::{self, PublicKey, SecretKey};
use exonum_testkit::{TestKit, TestKitBuilder};

use voting::{
    transactions::{CreateCandidate, CreateElector, Vote},
    schema::{VotingSchema, Candidate, Elector},
};

#[test]
fn test_create_candidate() {
    let mut testkit = init_testkit();
    let (john, _) = create_candidate(&mut testkit, "John");

    let candidate = get_candidate(&testkit, john.pub_key());
    assert_eq!(candidate.pub_key(), john.pub_key());
    assert_eq!(candidate.name(), "John");
    assert_eq!(candidate.voices(), 0);
}

#[test]
fn test_candidate_identity() {
    let mut testkit = init_testkit();
    let (john, sec) = create_candidate(&mut testkit, "John");

    let john_1 = CreateCandidate::new(john.pub_key(), "John_1", &sec);
    let block = testkit.create_block_with_transaction(john_1.clone());
    let tx_status = block.transactions[0].status().err().expect("Expect error.");
    assert_eq!(tx_status.description(), Some("Candidate already exists"));

    let candidate = get_candidate(&testkit, john.pub_key());
    assert_eq!(candidate.pub_key(), john.pub_key());
    assert_eq!(candidate.name(), "John");
    assert_eq!(candidate.voices(), 0);
}

#[test]
fn test_create_elector() {
    let mut testkit = init_testkit();
    let (den, _) = create_elector(&mut testkit, "Den");

    let candidate = get_elector(&testkit, den.pub_key());
    assert_eq!(candidate.pub_key(), den.pub_key());
    assert_eq!(candidate.name(), "Den");
    assert_eq!(candidate.has_vote(), true);
}

#[test]
fn test_elector_identity() {
    let mut testkit = init_testkit();
    let (den, sec) = create_elector(&mut testkit, "Den");

    let den_1 = CreateElector::new(den.pub_key(), "Den_1", &sec);

    let block = testkit.create_block_with_transaction(den_1.clone());
    let tx_status = block.transactions[0].status().err().expect("Expect error.");
    assert_eq!(tx_status.description(), Some("Elector already exists"));

    let candidate = get_elector(&testkit, den.pub_key());
    assert_eq!(candidate.pub_key(), den.pub_key());
    assert_eq!(candidate.name(), "Den");
    assert_eq!(candidate.has_vote(), true);
}

#[test]
fn test_vote() {
    let mut testkit = init_testkit();
    let (john, _) = create_candidate(&mut testkit, "John");
    let (den, sec) = create_elector(&mut testkit, "Den");
    create_vote_tx(&mut testkit, john.pub_key(), den.pub_key(), &sec);

    let elector = get_elector(&testkit, den.pub_key());
    assert_eq!(elector.has_vote(), false);

    let candidate = get_candidate(&testkit, john.pub_key());
    assert_eq!(candidate.voices(), 1);
}

#[test]
fn double_vote_test() {
    let mut testkit = init_testkit();
    let (john, _) = create_candidate(&mut testkit, "John");
    let (john_1, _) = create_candidate(&mut testkit, "John_1");
    let (den, sec) = create_elector(&mut testkit, "Den");

    create_vote_tx(&mut testkit, john.pub_key(), den.pub_key(), &sec);
    create_vote_tx(&mut testkit, john_1.pub_key(), den.pub_key(), &sec);

    let elector = get_elector(&testkit, den.pub_key());
    assert_eq!(elector.has_vote(), false);

    let john = get_candidate(&testkit, john.pub_key());
    assert_eq!(john.voices(), 1);

    let john_1 = get_candidate(&testkit, john_1.pub_key());
    assert_eq!(john_1.voices(), 0);
}

fn create_vote_tx(testkit: &mut TestKit, candidate_pub_key: &PublicKey, elector_key: &PublicKey, elector_seq_key: &SecretKey) -> Vote {
   let tx = Vote::new(elector_key, candidate_pub_key, elector_seq_key);
    testkit.create_block_with_transaction(tx.clone());
    tx
}

fn create_candidate(testkit: &mut TestKit, name: &str) -> (CreateCandidate, SecretKey) {
    let (pubkey, key) = crypto::gen_keypair();
    let tx = CreateCandidate::new(&pubkey, name, &key);
    testkit.create_block_with_transaction(tx.clone());
    (tx, key)
}

fn create_elector(testkit: &mut TestKit, name: &str) -> (CreateElector, SecretKey) {
    let (pubkey, key) = crypto::gen_keypair();
    let tx = CreateElector::new(&pubkey, name, &key);
    testkit.create_block_with_transaction(tx.clone());
    (tx, key)
}

fn try_get_candidate(testkit: &TestKit, pubkey: &PublicKey) -> Option<Candidate> {
    let snapshot = testkit.snapshot();
    VotingSchema::new(&snapshot).candidate(pubkey)
}

fn get_candidate(testkit: &TestKit, pubkey: &PublicKey) -> Candidate {
    try_get_candidate(testkit, pubkey).expect("No candidate persisted.")
}

fn try_get_elector(testkit: &TestKit, pubkey: &PublicKey) -> Option<Elector> {
    let snapshot = testkit.snapshot();
    VotingSchema::new(&snapshot).elector(pubkey)
}

fn get_elector(testkit: &TestKit, pubkey: &PublicKey) -> Elector {
    try_get_elector(testkit, pubkey).expect("No elector persisted.")
}

fn init_testkit() -> TestKit {
    TestKitBuilder::validator()
        .with_service(voting::service::VotingService)
        .create()
}