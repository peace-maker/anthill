use std::time::Duration;
use regex::Regex;
use std::path::Path;
use crate::team::Team;
use crate::exploit::Policy;

#[derive(Debug)]
struct Settings<'a> {
    /// Regex used to extract flags from the exploit output.
    flag_regex: Regex,
    /// Round/Tick time for drawing pretty plots.
    tick_length: Duration,
    /// Default exploit timeout prefilled when creating a new exploit.
    exploit_timeout: Duration,
    /// Default current working directory (CWD) prefilled when creating a new exploit.
    exploit_working_dir: &'a Path,
    /// Default policy which is pre-selected when creating a new exploit.
    default_policy: Policy,
    /// Our own team in the CTF.
    own_team: &'a Team,
    /// NOP team by event organizers. Possibly unpatched or worth no points.
    nop_team: &'a Team,
    /// Do we get points for exploiting the NOP team?
    nop_team_grants_points: bool,
    /// Number of flags we're allowed to submit at once.
    flag_submission_batch_size: u64,
    /// Number of concurrently running exploits to tune to the hardware.
    number_of_parallel_exploit_runs: u64,
}