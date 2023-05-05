use lazy_regex::regex;

use crate::miner::MinerError;

pub static VNISH_ERRORS: [MinerError; 4] = [
    MinerError {
        re: regex!(r#"chain#(\d) - Failed to init pic controller"#),
        msg: "Chain {} - Failed to init pic controller",
    },
    MinerError {
        re: regex!(r#"chain#(\d) - (\d+) of (\d+) chips detected, attempt 3"#),
        msg: "Chain {} - {} of {} chips detected",
    },
    MinerError {
        re: regex!(r#"Failed to set voltage to (\d+) mV"#),
        msg: "Failed to set voltage to {} mV",
    },
    MinerError {
        re: regex!(r#"chain#(\d) - Chain break detected"#),
        msg: "Chain {} - Chain break detected",
    },
];