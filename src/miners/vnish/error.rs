use lazy_regex::regex;

use crate::miner::{IntMinerError, ErrorType};

pub(crate) static VNISH_ERRORS: [IntMinerError; 11] = [
    IntMinerError {
        re: regex!(r#"chain#(\d) - [Ff]ailed to init pic controller"#),
        msg: "Chain {} - Failed to init pic controller",
        error_type: ErrorType::HashBoard,
    },
    IntMinerError {
        re: regex!(r#"chain#(\d) - (\d+) of (\d+) chips detected, attempt 3"#),
        msg: "Chain {} - {} of {} chips detected",
        error_type: ErrorType::HashBoard,
    },
    IntMinerError {
        re: regex!(r#"[Ff]ailed to set voltage to (\d+) mV"#),
        msg: "Failed to set voltage to {} mV",
        error_type: ErrorType::Power,
    },
    IntMinerError {
        re: regex!(r#"chain#(\d) - [Cc]hain break detected"#),
        msg: "Chain {} - Chain break detected",
        error_type: ErrorType::HashBoard,
    },
    IntMinerError {
        re: regex!(r#"chain#(\d) - [Oo]verheated, pcb temp=(\d+)"#),
        msg: "Chain {} - Overheated PCB {} C",
        error_type: ErrorType::Temperature,
    },
    IntMinerError {
        re: regex!(r#"chain#(\d) - [Oo]verheated, chip temp=(\d+)"#),
        msg: "Chain {} - Overheated Chip {} C",
        error_type: ErrorType::Temperature,
    },
    IntMinerError {
        re: regex!(r#"fan#(\d) - (?:LOST|lost)"#),
        msg: "Lost Fan {}",
        error_type: ErrorType::Fan,
    },
    IntMinerError {
        re: regex!(r#"chain#(\d) - [Ff]ailed to init board temp sensors"#),
        msg: "Chain {} - Failed to init board temp sensors",
        error_type: ErrorType::HashBoard,
    },
    IntMinerError {
        re: regex!(r#"ERROR: chain#(\d) - [Ff]ailed to power on the chain"#),
        msg: "Chain {} - Failed to power on the chain",
        error_type: ErrorType::HashBoard,
    },
    IntMinerError {
        re: regex!(r#"ERROR: chain#(\d) sen#(\d) - dead, temperature doesn't change"#),
        msg: "Chain {} - Sensor {} dead, temperature doesn't change",
        error_type: ErrorType::HashBoard,
    },
    IntMinerError {
        re: regex!(r#"chain#(\d) - [Ff]ailed to load eeprom data /chain-info.c:73/"#),
        msg: "Chain {} - Failed to load eeprom data",
        error_type: ErrorType::HashBoard,
    },
];