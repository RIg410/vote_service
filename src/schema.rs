use exonum::{
    crypto::{PublicKey, Hash},
    storage::{Fork, Snapshot, ProofMapIndex, ProofListIndex},
};

const CANDIDATE_INDEX: &str = "vote.candidate";
const ELECTORATE_INDEX: &str = "vote.electorate";
const VOTE_HISTORY: &str = "vote.history";

encoding_struct! {
    struct Candidate {
        pub_key: &PublicKey,
        name: &str,
        history_hash: &Hash,
        voices: u64,
    }
}

impl Candidate {
    pub fn add_voice(self, history_hash: &Hash) -> Self {
        Candidate::new(self.pub_key(), self.name(), history_hash, self.voices() + 1)
    }
}

encoding_struct! {
    struct Elector {
        pub_key: &PublicKey,
        name: &str,
        has_vote: bool,
    }
}

impl Elector {
    pub fn vote(self) -> Self {
        Elector::new(self.pub_key(), self.name(), false)
    }
}

pub struct VoteSchema<T> {
    view: T,
}

impl<T> VoteSchema<T> {
    pub fn new(snapshot: T) -> Self {
        VoteSchema { view: snapshot }
    }
}

impl<T: AsRef<dyn Snapshot>> VoteSchema<T> {
    pub fn state_hash(&self) -> Vec<Hash> {
        vec![
            self.candidates().merkle_root(),
            self.electorate().merkle_root()
        ]
    }

    pub fn candidates(&self) -> ProofMapIndex<&dyn Snapshot, PublicKey, Candidate> {
        ProofMapIndex::new(CANDIDATE_INDEX, self.view.as_ref())
    }

    pub fn candidate(&self, pub_key: &PublicKey) -> Option<Candidate> {
        self.candidates().get(pub_key)
    }

    pub fn electorate(&self) -> ProofMapIndex<&dyn Snapshot, PublicKey, Elector> {
        ProofMapIndex::new(ELECTORATE_INDEX, self.view.as_ref())
    }

    pub fn elector(&self, pub_key: &PublicKey) -> Option<Elector> {
        self.electorate().get(pub_key)
    }

    pub fn vote_history(&self, public_key: &PublicKey) -> ProofListIndex<&T, Hash> {
        ProofListIndex::new_in_family(VOTE_HISTORY, public_key, &self.view)
    }
}

impl<'a> VoteSchema<&'a mut Fork> {
    pub fn candidate_mut(&mut self) -> ProofMapIndex<&mut Fork, PublicKey, Candidate> {
        ProofMapIndex::new(CANDIDATE_INDEX, &mut self.view)
    }

    pub fn electorate_mut(&mut self) -> ProofMapIndex<&mut Fork, PublicKey, Elector> {
        ProofMapIndex::new(ELECTORATE_INDEX, &mut self.view)
    }

    pub fn vote_history_mut(&mut self, public_key: &PublicKey) -> ProofListIndex<&mut Fork, Hash> {
        ProofListIndex::new_in_family(VOTE_HISTORY, public_key, &mut self.view)
    }
}
