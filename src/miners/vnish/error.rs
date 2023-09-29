use lazy_regex::regex;

use crate::miner::{IntMinerError, ErrorType};

pub(crate) static VNISH_ERRORS: [IntMinerError; 9] = [
    IntMinerError {
        re: regex!(r#"chain#(\d) - Failed to init pic controller"#),
        msg: "Chain {} - Failed to init pic controller",
        error_type: ErrorType::HashBoard,
    },
    IntMinerError {
        re: regex!(r#"chain#(\d) - (\d+) of (\d+) chips detected, attempt 3"#),
        msg: "Chain {} - {} of {} chips detected",
        error_type: ErrorType::HashBoard,
    },
    IntMinerError {
        re: regex!(r#"Failed to set voltage to (\d+) mV"#),
        msg: "Failed to set voltage to {} mV",
        error_type: ErrorType::Power,
    },
    IntMinerError {
        re: regex!(r#"chain#(\d) - Chain break detected"#),
        msg: "Chain {} - Chain break detected",
        error_type: ErrorType::HashBoard,
    },
    IntMinerError {
        re: regex!(r#"chain#(\d) - Overheated, pcb temp=(\d+)"#),
        msg: "Chain {} - Overheated PCB {} C",
        error_type: ErrorType::Temperature,
    },
    IntMinerError {
        re: regex!(r#"chain#(\d) - Overheated, chip temp=(\d+)"#),
        msg: "Chain {} - Overheated Chip {} C",
        error_type: ErrorType::Temperature,
    },
    IntMinerError {
        re: regex!(r#"fan#(\d) - LOST"#),
        msg: "Lost Fan {}",
        error_type: ErrorType::Fan,
    },
    IntMinerError {
        re: regex!(r#"chain#(\d) - Failed to init board temp sensors"#),
        msg: "Chain {} - Failed to init board temp sensors",
        error_type: ErrorType::HashBoard,
    },
    IntMinerError {
        re: regex!(r#"ERROR: chain#(\d) - Failed to power on the chain"#),
        msg: "Chain {} - Failed to power on the chain",
        error_type: ErrorType::HashBoard,
    }
];