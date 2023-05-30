use lazy_regex::regex;

use crate::miner::{MinerError, ErrorType};

pub static ANTMINER_ERRORS: [MinerError; 11] = [
    // Unsure
    MinerError {
        re: regex!(r".+load chain ([0-9]).+\n.+(EEPROM error|bad_asic_crc)"),
        msg: "Chain {} EEPROM CRC error",
        error_type: ErrorType::HashBoard,
    },
    MinerError {
        re: regex!(r"Data load fail for chain ([0-9])"),
        msg: "Chain {} load EEPROM fail",
        error_type: ErrorType::HashBoard,
    },
    MinerError {
        re: regex!(r".+ERROR_POWER_LOST"),
        msg: "Power lost",
        error_type: ErrorType::Power,
    },
    MinerError {
        re: regex!(r".+ERROR_FAN_LOST"),
        msg: "Fan lost",
        error_type: ErrorType::Fan,
    },
    MinerError {
        re: regex!(r".+ERROR_TEMP_TOO_HIGH"),
        msg: "Temperature too high",
        error_type: ErrorType::Temperature,
    },
    MinerError {
        re: regex!(r".+_read_an6_voltage"),
        msg: "Read voltage failed",
        error_type: ErrorType::Power,
    },
    MinerError {
        re: regex!(r".+Chain ([0-9]) only find ([0-9]+) asic"),
        msg: "Chain {} only find {} asic",
        error_type: ErrorType::HashBoard,
    },
    MinerError {
        re: regex!(r".+i2c: timeout waiting for bus ready"),
        msg: "I2C timeout",
        error_type: ErrorType::ControlBoard,
    },
    MinerError {
        re: regex!(r".+fail to read pic temp for chain ([0-9])"),
        msg: "Chain {} read pic temp fail",
        error_type: ErrorType::HashBoard,
    },
    MinerError {
        re: regex!(r".+FW type (.+), (?:.+) shows (.+)"),
        msg: "Incorrect firmware (should be {}, found {})",
        error_type: ErrorType::Config,
    },
    MinerError {
        re: regex!(r".+read temp sensor failed: chain = ([0-9])"),
        msg: "Chain {} read temp sensor failed",
        error_type: ErrorType::HashBoard,
    }
];
