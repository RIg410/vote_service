use exonum::{
    blockchain::{ExecutionError, Transaction}, crypto::{PublicKey, CryptoHash},
    messages::Message, storage::Fork,
};
use service;
use schema::{VotingSchema, Candidate, Elector};
use errors::Error;

transactions! {
    pub VotingTransactions {
        const SERVICE_ID = service::VOTING_SERVICE;

        struct CreateCandidate {
            pub_key: &PublicKey,
            name: &str,
        }

        struct CreateElector {
            pub_key: &PublicKey,
            name: &str,
        }

        struct Vote {
            elector: &PublicKey,
            candidate: &PublicKey,
        }
    }
}

impl Transaction for CreateCandidate {
    fn verify(&self) -> bool {
        self.verify_signature(self.pub_key())
    }

    fn execute(&self, fork: &mut Fork) -> Result<(), ExecutionError> {
        let mut schema = VotingSchema::new(fork);

        if schema.candidate(self.pub_key()).is_none() {
            let history_hash = {
                let mut history = schema.vote_history_mut(self.pub_key());
                history.push(self.hash());
                history.merkle_root()
            };

            let candidate = Candidate::new(self.pub_key(), self.name(), &history_hash, 0);

            println!("Create the candidate: {:?}", candidate);
            schema.candidate_mut().put(self.pub_key(), candidate);
            Ok(())
        } else {
            Err(Error::CandidateAlreadyExists)?
        }
    }
}

impl Transaction for CreateElector {
    fn verify(&self) -> bool {
        self.verify_signature(self.pub_key())
    }

    fn execute(&self, fork: &mut Fork) -> Result<(), ExecutionError> {
        let mut schema = VotingSchema::new(fork);

        if schema.elector(self.pub_key()).is_none() {
            let elector = Elector::new(self.pub_key(), self.name(), true);
            println!("Create the elector: {:?}", elector);
            schema.electorate_mut().put(self.pub_key(), elector);
            Ok(())
        } else {
            Err(Error::ElectorAlreadyExists)?
        }
    }
}

impl Transaction for Vote {
    fn verify(&self) -> bool {
        self.verify_signature(self.elector())
    }

    fn execute(&self, fork: &mut Fork) -> Result<(), ExecutionError> {
        let mut schema = VotingSchema::new(fork);
        let elector = match schema.elector(self.elector()) {
            Some(val) => val,
            None => Err(Error::ElectorNotFound)?,
        };

        let candidate = match schema.candidate(self.candidate()) {
            Some(val) => val,
            None => Err(Error::CandidateNotFound)?,
        };

        if elector.has_vote() {
            let history_hash = {
                let mut history = schema.vote_history_mut(candidate.pub_key());
                history.push(self.hash());
                history.merkle_root()
            };

            let elector = elector.vote();
            let candidate = candidate.add_voice(&history_hash);

            println!("{:?} voted in favor of {:?}", elector, candidate);
            schema.electorate_mut().put(self.elector(), elector);
            schema.candidate_mut().put(self.candidate(), candidate);
            Ok(())
        } else {
            Err(Error::AlreadyVoted)?
        }
    }
}

