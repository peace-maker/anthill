use std::time::{Instant};
use crate::exploit::ExploitRun;

#[derive(PartialEq, Debug)]
enum FlagSubmissionResult {
    /// A valid flag that gained points.
    Valid,
    /// This flag was submitted previously.
    AlreadySubmitted,
    /// This is not a valid flag.
    Invalid,
    /// This flag is too old, no points anymore.
    Expired,
    /// This flag belongs to our own team. No points.
    Own,
    /// This flag belongs to the NOP team. Possibly no points.
    NOPTeam,
    /// Submission server failed to process our request.
    Error,
    /// The flag wasn't sent to the submission server yet.
    Pending,
}

#[derive(Debug)]
struct Flag {
    /// Plain flag value that was seen.
    flag: String,
    /// Submission time of when it was handed to the flag submission endpoint.
    submission_time: Instant,
    /// Mapped answer of the submission endpoint if the flag was valid or not.
    submission_result: FlagSubmissionResult,
}

impl Flag {
    fn create(flag: String) -> Flag {
        Flag {
            flag,
            submission_time: Instant::now(),
            submission_result: FlagSubmissionResult::Pending,
        }
    }

    fn is_valid(&self) -> bool {
        self.submission_result == FlagSubmissionResult::Valid
    }

    fn is_pending(&self) -> bool {
        match self.submission_result {
            FlagSubmissionResult::Error => true,
            FlagSubmissionResult::Pending => true,
            _ => false
        }
    }
}

#[derive(Debug)]
pub struct FlagOccurence<'c> {
    /// The flag that was seen (again).
    flag: &'c Flag,
    /// Time of when the flag was stolen.
    collection_time: Instant,
    /// The exploit run which got this flag.
    exploit_run: &'c ExploitRun<'c>,
}

#[derive(Debug)]
pub struct UnknownFlagResponse<'c> {
    /// The flag that was submitted.
    flag: &'c Flag,
    /// Whatever we got from the submission server.
    raw_submission_result: [u8],
}
