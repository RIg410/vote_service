use exonum::blockchain::ExecutionError;

#[derive(Debug, Fail)]
#[repr(u8)]
pub enum Error {
    #[fail(display = "Candidate already exists")]
    CandidateAlreadyExists = 0,

    #[fail(display = "Elector already exists")]
    ElectorAlreadyExists = 1,

    #[fail(display = "Elector doesn't exist")]
    ElectorNotFound = 2,

    #[fail(display = "Candidate doesn't exist")]
    CandidateNotFound = 3,

    #[fail(display = "The voter has already voted.")]
    AlreadyVoted = 4,
}

impl From<Error> for ExecutionError {
    fn from(value: Error) -> ExecutionError {
        let description = format!("{}", value);
        ExecutionError::with_description(value as u8, description)
    }
}