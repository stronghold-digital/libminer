use lazy_regex::regex;

use crate::miner::{MinerError, ErrorType};

pub static WHATSMINER_ERRORS: [MinerError; 67] = [
    MinerError {
        re: regex!(r"1[0-3](0|1)"),
        msg: "Fan {} speed error",
        error_type: ErrorType::Fan,
    },
    MinerError {
        re: regex!(r"140"),
        msg: "Fan speed too high",
        error_type: ErrorType::Fan,
    },

    MinerError {
        re: regex!(r"200"),
        msg: "No power found",
        error_type: ErrorType::Power,
    },
    MinerError {
        re: regex!(r"201"),
        msg: "Power configuration mismatch",
        error_type: ErrorType::Power,
    },
    MinerError {
        re: regex!(r"202"),
        msg: "Power output voltage error",
        error_type: ErrorType::Power,
    },
    MinerError {
        re: regex!(r"20[3,4]"),
        msg: "Power protection triggered",
        error_type: ErrorType::Power,
    },
    MinerError {
        re: regex!(r"205"),
        msg: "Power current error",
        error_type: ErrorType::Power,
    },
    MinerError {
        re: regex!(r"206"),
        msg: "Low input voltage",
        error_type: ErrorType::Power,
    },
    MinerError {
        re: regex!(r"207"),
        msg: "Input current protection",
        error_type: ErrorType::Power,
    },
    MinerError {
        re: regex!(r"210"),
        msg: "Power error status",
        error_type: ErrorType::Power,
    },
    MinerError {
        re: regex!(r"213"),
        msg: "Input voltage and current do not match",
        error_type: ErrorType::Power,
    },
    MinerError {
        re: regex!(r"216"),
        msg: "Power remained unchanged",
        error_type: ErrorType::Power,
    },
    MinerError {
        re: regex!(r"217"),
        msg: "Power enable error",
        error_type: ErrorType::Power,
    },
    MinerError {
        re: regex!(r"218"),
        msg: "Input voltage below 230V in high-perf mode",
        error_type: ErrorType::Power,
    },
    MinerError {
        re: regex!(r"23[3-5]"),
        msg: "Power output over-temperature",
        error_type: ErrorType::Power,
    },
    MinerError {
        re: regex!(r"23[6-8]|268"),
        msg: "Power output overcurrent",
        error_type: ErrorType::Power,
    },
    MinerError {
        re: regex!(r"239"),
        msg: "Power output over voltage",
        error_type: ErrorType::Power,
    },
    MinerError {
        re: regex!(r"240"),
        msg: "Power output under voltage",
        error_type: ErrorType::Power,
    },
    MinerError {
        re: regex!(r"241"),
        msg: "Power output current imbalance",
        error_type: ErrorType::Power,
    },
    MinerError {
        re: regex!(r"24[3-5]"),
        msg: "Power input over-temperature",
        error_type: ErrorType::Power,
    },
    MinerError {
        re: regex!(r"24[6,7]|269"),
        msg: "Power input overcurrent",
        error_type: ErrorType::Power,
    },
    MinerError {
        re: regex!(r"24[8,9]|270"),
        msg: "Power input over voltage",
        error_type: ErrorType::Power,
    },
    MinerError {
        re: regex!(r"25[0,1]|271"),
        msg: "Power input under voltage",
        error_type: ErrorType::Power,
    },
    MinerError {
        re: regex!(r"25[3,4]"),
        msg: "PSU fan error",
        error_type: ErrorType::Power,
    },
    MinerError {
        re: regex!(r"25[5,6]"),
        msg: "Power output over power",
        error_type: ErrorType::Power,
    },
    MinerError {
        re: regex!(r"257"),
        msg: "Input overcurrent protection on primary",
        error_type: ErrorType::Power,
    },
    MinerError {
        re: regex!(r"263"),
        msg: "Power communication warning",
        error_type: ErrorType::Power,
    },
    MinerError {
        re: regex!(r"264"),
        msg: "Power communication error",
        error_type: ErrorType::Power,
    },
    MinerError {
        re: regex!(r"267"),
        msg: "Power watchdog error",
        error_type: ErrorType::Power,
    },
    MinerError {
        re: regex!(r"272"),
        msg: "Excessive power output warning",
        error_type: ErrorType::Power,
    },
    MinerError {
        re: regex!(r"273"),
        msg: "Power input power too high",
        error_type: ErrorType::Power,
    },
    MinerError {
        re: regex!(r"274"),
        msg: "PSU fan warning",
        error_type: ErrorType::Power,
    },
    MinerError {
        re: regex!(r"275"),
        msg: "PSU over-temperature warning",
        error_type: ErrorType::Power,
    },

    MinerError {
        re: regex!(r"30([0-2])"),
        msg: "Board {} temperature sensor error",
        error_type: ErrorType::HashBoard,
    },
    MinerError {
        re: regex!(r"32([0-2])"),
        msg: "Board {} temperature reading error",
        error_type: ErrorType::HashBoard,
    },
    MinerError {
        re: regex!(r"329"),
        msg: "Control board temperature sensor error",
        error_type: ErrorType::ControlBoard,
    },
    MinerError {
        re: regex!(r"35([0-2])"),
        msg: "Board {} overheating",
        error_type: ErrorType::Temperature,
    },
    MinerError {
        re: regex!(r"360"),
        msg: "Board overheating",
        error_type: ErrorType::Temperature,
    },

    MinerError {
        re: regex!(r"41([0-2])"),
        msg: "Board {} EEPROM detect error",
        error_type: ErrorType::HashBoard,
    },
    MinerError {
        re: regex!(r"42([0-2])"),
        msg: "Board {} EEPROM parse error",
        error_type: ErrorType::HashBoard,
    },
    MinerError {
        re: regex!(r"43([0-2])"),
        msg: "Board {} EEPROM chip bin type error",
        error_type: ErrorType::HashBoard,
    },
    MinerError {
        re: regex!(r"44([0-2])"),
        msg: "Board {} EEPROM chip number error",
        error_type: ErrorType::HashBoard,
    },
    MinerError {
        re: regex!(r"45([0-2])"),
        msg: "Board {} EEPROM transfer error",
        error_type: ErrorType::HashBoard,
    },
    MinerError {
        re: regex!(r"51([0-2])"),
        msg: "Board {} type error",
        error_type: ErrorType::HashBoard,
    },
    MinerError {
        re: regex!(r"52([0-2])"),
        msg: "Board {} bin type error",
        error_type: ErrorType::HashBoard,
    },
    MinerError {
        re: regex!(r"53([0-2])"),
        msg: "Board {} not found",
        error_type: ErrorType::HashBoard,
    },
    MinerError {
        re: regex!(r"54([0-2])"),
        msg: "Board {} read chip id error",
        error_type: ErrorType::HashBoard,
    },
    MinerError {
        re: regex!(r"55([0-2])"),
        msg: "Board {} bad chip",
        error_type: ErrorType::HashBoard,
    },
    MinerError {
        re: regex!(r"56([0-2])"),
        msg: "Board {} loss balance",
        error_type: ErrorType::HashBoard,
    },
    MinerError {
        re: regex!(r"511([0-2])"),
        msg: "Board {} frequency up timeout",
        error_type: ErrorType::HashBoard,
    },
    MinerError {
        re: regex!(r"507([0-2])"),
        msg: "Board {} water velocity abnormal",
        error_type: ErrorType::HashBoard,
    },

    MinerError {
        re: regex!(r"600"),
        msg: "Overheating",
        error_type: ErrorType::Temperature,
    },
    MinerError {
        re: regex!(r"610"),
        msg: "Temperature too high in high-perf mode",
        error_type: ErrorType::Temperature,
    },

    MinerError {
        re: regex!(r"701"),
        msg: "Control board no support chip",
        error_type: ErrorType::ControlBoard,
    },
    MinerError {
        re: regex!(r"710|712"),
        msg: "Control board rebooted as exception",
        error_type: ErrorType::ControlBoard,
    },

    MinerError {
        re: regex!(r"800"),
        msg: "Cgminer checksum error",
        error_type: ErrorType::ControlBoard,
    },
    MinerError {
        re: regex!(r"801"),
        msg: "System-monitor checksum error",
        error_type: ErrorType::ControlBoard,
    },
    MinerError {
        re: regex!(r"802"),
        msg: "Remote-daemon checksum error",
        error_type: ErrorType::ControlBoard,
    },

    MinerError {
        re: regex!(r"2000"),
        msg: "No pools configured",
        error_type: ErrorType::Config,
    },
    MinerError {
        re: regex!(r"2010"),
        msg: "All pools disabled",
        error_type: ErrorType::Config,
    },
    MinerError {
        re: regex!(r"202[0-2]"),
        msg: "Pool {} connect failure",
        error_type: ErrorType::Network,
    },
    MinerError {
        re: regex!(r"2030"),
        msg: "High pool reject rate",
        error_type: ErrorType::Network,
    },
    MinerError {
        re: regex!(r"2040"),
        msg: "Pool does not support asicboost",
        error_type: ErrorType::Config,
    },
    MinerError {
        re: regex!(r"23[1,2]0"),
        msg: "Hashrate too low",
        error_type: ErrorType::Other,
    },
    MinerError {
        re: regex!(r"24[1,2]0"),
        msg: "Hashrate loss is too high",
        error_type: ErrorType::Other,
    },

    MinerError {
        re: regex!(r"8410"),
        msg: "Incorrect firmware version",
        error_type: ErrorType::Config,
    },
    MinerError {
        re: regex!(r"10000[0-3]"),
        msg: "Corrupted firmware signature",
        error_type: ErrorType::Config,
    },
];
