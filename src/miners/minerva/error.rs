use lazy_regex::regex;

use crate::miner::{IntMinerError, ErrorType};

pub(crate) static MINERA_ERRORS: [IntMinerError; 4] = [
    IntMinerError {
        re: regex!(r"power up to.+failed read_bak"),
        msg: "PSU failed to power up",
        error_type: ErrorType::Power,
    },
    IntMinerError {
        re: regex!(r"ACK not found"),
        msg: "SPI ACK not found",
        error_type: ErrorType::HashBoard,
    },
    IntMinerError {
        re: regex!("low fan speed of fan ([0-9])"),
        msg: "Fan {} speed too low",
        error_type: ErrorType::Fan,
    },
    IntMinerError {
        re: regex!("C3012 ([0-9]) failure disabling!"),
        msg: "Chain {} failure",
        error_type: ErrorType::HashBoard,
    },
];

pub(crate) static MINERVA_ERRORS: [IntMinerError; 10] = [
    IntMinerError {
        re: regex!(r".+Error: fan ([0-9]) failed"),
        msg: "Fan {} failed",
        error_type: ErrorType::Fan,
    },
    IntMinerError {
        re: regex!(r".+booting board ([0-9]).+\n.+ACK not found"),
        msg: "Board {} ACK not found",
        error_type: ErrorType::HashBoard,
    },
    IntMinerError {
        re: regex!(r".+(voltage not up to standard|电源故障，电压不达标)"),
        msg: "Voltage not up to standard",
        error_type: ErrorType::Power,
    },
    IntMinerError {
        re: regex!(r".+Error: init power supply"),
        msg: "Unable to init power supply",
        error_type: ErrorType::Power,
    },
    IntMinerError {
        re: regex!(r".+(?:init chip|启动芯片|初始化芯片)([0-9])/([0-9])"),
        msg: "Failed to init board {} chip {}",
        error_type: ErrorType::HashBoard,
    },
    IntMinerError {
        re: regex!(r".+mv64xxx_i2c_fsm: Ctlr Error"),
        msg: "I2C controller error",
        error_type: ErrorType::ControlBoard,
    },
    IntMinerError {
        re: regex!(r".+Stratum connection to pool [0-9] interrupted.+\n.+flushing server.+\n.+flush failed"),
        msg: "Connection interrupted, failed to flush server",
        error_type: ErrorType::Network,
    },
    IntMinerError {
        re: regex!(r".+read eeprom failed:I2C(Nix(ENXIO))"),
        msg: "Failed to read EEPROM device did not respond",
        error_type: ErrorType::HashBoard,
    },
    IntMinerError {
        re: regex!(r".+waiting for fan spinning up: rpm: 0"),
        msg: "Fan not spinning up or sense fail",
        error_type: ErrorType::Fan,
    },
    IntMinerError {
        re: regex!(r".+board temp: ([\d\.]+) C, sleep for protect"),
        msg: "Board temperature {}C failed to cool below 40C",
        error_type: ErrorType::Temperature,
    }
];
