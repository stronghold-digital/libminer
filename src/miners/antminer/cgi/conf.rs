use serde::{Deserialize, Serialize};
use crate::Pool;

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum StringOrInt {
    String(String),
    Int(u8),
    BigInt(u64),
}

impl StringOrInt {
    pub fn as_int(&self) -> u8 {
        match self {
            StringOrInt::String(s) => s.parse().unwrap_or(0),
            StringOrInt::Int(i) => *i,
            StringOrInt::BigInt(i) => (*i % 256) as u8, // Ensure it fits in u8
        }
    }
}

#[derive(Serialize, Clone, Deserialize, Debug)]
#[serde(untagged)]
pub enum BoolIntStr {
    Bool(bool),
    Int(u8),
    String(String),
}

#[derive(Serialize, Clone, Deserialize, Debug)]
pub struct GetConfResponse {
    // #[serde(rename = "api-allow")]
    // pub api_allow: String,
    // #[serde(rename = "api-groups")]
    // pub api_groups: String,
    // #[serde(rename = "api-listen")]
    // pub api_listen: bool,
    // #[serde(rename = "api-network")]
    // pub api_network: bool,
    // #[serde(rename = "bitmain-ccdelay")]
    // pub bitmain_ccdelay: String,
    #[serde(rename = "bitmain-fan-ctrl")]
    pub bitmain_fan_ctrl: BoolIntStr,
    #[serde(rename = "bitmain-fan-pwm")]
    pub bitmain_fan_pwm: StringOrInt,
    #[serde(rename = "bitmain-freq", default, skip_serializing_if = "Option::is_none")]
    pub bitmain_freq: Option<StringOrInt>,
    #[serde(rename = "bitmain-freq-level", default, skip_serializing_if = "Option::is_none")]
    pub bitmain_freq_level: Option<StringOrInt>,
    #[serde(rename = "bitmain-user-ip-cat")]
    pub bitmain_user_ip_cat: BoolIntStr,
    // #[serde(rename = "bitmain-pwth")]
    // pub bitmain_pwth: String,
    // #[serde(rename = "bitmain-use-vil")]
    // pub bitmain_use_vil: bool,
    #[serde(rename = "bitmain-voltage", default, skip_serializing_if = "Option::is_none")]
    pub bitmain_voltage: Option<f32>,
    /// "0" is normal, "1" is sleep
    #[serde(rename = "bitmain-work-mode")]
    pub bitmain_work_mode: StringOrInt,
    // #[serde(rename = "bitmain-hashrate-percent")]
    // pub bitmain_hashrate_percent: Option<String>,
    pub pools: Vec<Pool>,
}

#[derive(Serialize, Debug)]
pub struct SetConf {
    // #[serde(rename = "bitmain-fan-ctrl")]
    // pub bitmain_fan_ctrl: bool,
    // #[serde(rename = "bitmain-fan-pwm")]
    // pub bitmain_fan_pwm: String,
    // #[serde(rename = "freq-level")]
    // pub freq_level: String,
    /// 0 is normal, 1 is sleep
    #[serde(rename = "miner-mode")]
    pub miner_mode: u8,
    pub pools: Vec<Pool>,
}

impl From<&GetConfResponse> for SetConf {
    fn from(conf: &GetConfResponse) -> Self {
        SetConf {
            // bitmain_fan_ctrl: conf.bitmain_fan_ctrl,
            // bitmain_fan_pwm: conf.bitmain_fan_pwm.clone(),
            // freq_level: conf.bitmain_freq_level.clone(),
            // Antminers sometimes have this empty, default to 0 (normal)
            miner_mode: conf.bitmain_work_mode.as_int(),
            pools: conf.pools.clone(),
        }
    }
}
