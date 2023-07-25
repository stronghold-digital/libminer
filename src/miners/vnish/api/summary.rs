use serde::{Deserialize, de::Deserializer};

use super::{CoolingSettings, StatusCode};

#[derive(Deserialize)]
pub struct MinerSummaryStatus {
    pub miner_state: StatusCode,
    pub miner_state_time: usize,
}

#[derive(Deserialize)]
pub struct TempMinMax {
    pub min: i32,
    pub max: i32,
}

#[derive(PartialEq, Debug)]
pub enum PoolStatus {
    Working,
    Active,
    Offline,
    Disabled,
    Unknown,
}

impl<'de> Deserialize<'de> for PoolStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(match String::deserialize(deserializer)?.as_str() {
            "working" => PoolStatus::Working,
            "active" => PoolStatus::Active,
            "offline" => PoolStatus::Offline,
            "disabled" => PoolStatus::Disabled,
            "unknown" => PoolStatus::Unknown,
            _ => return Err(serde::de::Error::custom("Unknown pool status")),
        })
    }
}

#[derive(Deserialize)]
pub struct PoolStats {
    pub id: u32,
    pub url: String,
    pub user: String,
    pub accepted: u32,
    pub rejected: u32,
    pub status: PoolStatus,
    pub asic_boost: bool,
    pub ls_time: String,
    pub ls_diff: f64,
    pub stale: u32,
    pub diff: String,
}

#[derive(Deserialize)]
pub struct Fan {
    pub id: u32,
    pub rpm: u32,
}

#[derive(Deserialize)]
pub struct CoolingStats {
    pub fan_num: u32,
    pub fans: Vec<Fan>,
    //pub settings: CoolingSettings,
    pub fan_duty: u32,
}

#[derive(PartialEq, Debug)]
pub enum TempSensorStatus {
    Init,
    Measure,
    Error,
    Unknown,
}

impl<'de> Deserialize<'de> for TempSensorStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(match String::deserialize(deserializer)?.as_str() {
            "init" => TempSensorStatus::Init,
            "measure" => TempSensorStatus::Measure,
            "error" => TempSensorStatus::Error,
            "unknown" => TempSensorStatus::Unknown,
            _ => return Err(serde::de::Error::custom("Unknown temp sensor status")),
        })
    }
}

#[derive(Deserialize)]
pub struct TempSensor {
    pub status: TempSensorStatus,
    pub temp: i32,
}

#[derive(Deserialize)]
pub struct ChipStatus {
    pub red: i32,
    pub orange: i32,
    pub grey: i32,
}

#[derive(PartialEq, Debug)]
pub enum ChainState {
    Initializing,
    Mining,
    Stopped,
    Failure,
    Disconnected,
    Disabled,
    Unknown,
}

impl<'de> Deserialize<'de> for ChainState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(match String::deserialize(deserializer)?.as_str() {
            "initializing" => ChainState::Initializing,
            "mining" => ChainState::Mining,
            "stopped" => ChainState::Stopped,
            "failure" => ChainState::Failure,
            "disconnected" => ChainState::Disconnected,
            "disabled" => ChainState::Disabled,
            "unknown" => ChainState::Unknown,
            _ => return Err(serde::de::Error::custom("Unknown chain status")),
        })
    }
}

#[derive(Deserialize)]
pub struct ChainStatus {
    pub state: ChainState,
}

#[derive(Deserialize)]
pub struct Chain {
    pub id: u32,
    pub frequency: f64,
    pub hashrate_ideal: f64,
    pub hashrate_rt: f64,
    pub pcb_temp_sens: Vec<TempSensor>,
    pub chip_temp_sens: Vec<TempSensor>,
    pub hw_errors: i32,
    pub voltage: i64,
    pub chip_statuses: ChipStatus,
    pub status: ChainStatus,
}

#[derive(Deserialize)]
pub struct MinerSummary {
    pub miner_status: MinerSummaryStatus,
    pub miner_type: String,
    pub hardware_version: String,
    pub cgminer_version: String,
    pub compile_time: String,
    pub average_hashrate: f64,
    pub instant_hashrate: f64,
    pub pcb_temp: TempMinMax,
    pub chip_temp: TempMinMax,
    pub power_usage: f64,
    pub power_efficiency: f64,
    pub hw_errors_percent: f64,
    pub hw_errors: i64,
    pub devfee_percent: f64,
    pub devfee: f64,
    pub pools: Vec<PoolStats>,
    pub cooling: CoolingStats,
    pub chains: Vec<Chain>,
    pub found_blocks: f64,
    pub best_share: i64
}

#[derive(Deserialize)]
pub struct Summary {
    pub miner: Option<MinerSummary>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn test_de() {
        let s = r#"{"system": {"os": "GNU/Linux","miner_name": "19.36x55","file_system_version": "","mem_total": 252120,"mem_free": 214968,"mem_free_percent": 85,"mem_buf": 19612,"mem_buf_percent": 7,"network_status": {"mac": "B4:10:7B:A1:B1:E6","dhcp": true,"ip": "10.24.36.55","netmask": "255.255.254.0","gateway": "10.24.37.254","dns": ["208.67.220.220","208.67.222.222"],"hostname": "Antminer"},"uptime": " 1:00"},"miner": {"miner_status": {"miner_state": "mining","miner_state_time": 3564},"miner_type": "Antminer S19j Pro (Vnish 1.2.0-rc3)","hardware_version": "uart_trans.1.3","cgminer_version": "4.11.1","compile_time": "Sat Jul 22 16:41:12 UTC 2023","average_hashrate": 54.50821,"instant_hashrate": 54.478542,"hr_realtime": 54478.543,"hr_average": 54508.21,"pcb_temp": {"min": 36,"max": 50},"chip_temp": {"min": 51,"max": 65},"power_consumption": 2631,"power_usage": 2631,"power_efficiency": 0.048267957,"hw_errors_percent": 0.0,"hr_error": 0.0,"hw_errors": 0,"devfee_percent": 1.842,"devfee": 1003.58887,"pools": [{"id": 0,"url": "btc.foundryusapool.com:3333","pool_type": "UserPool","user": "pcs19jpro.19.36x55","status": "active","asic_boost": true,"diff": "131K","accepted": 258,"rejected": 0,"stale": 0,"ls_diff": 131072.0,"ls_time": "0:00:11","diffa": 23461888.0,"ping": 19},{"id": 1,"url": "btc.foundryusapool.com:443","pool_type": "UserPool","user": "pcs19jpro.19.36x55","status": "working","asic_boost": true,"diff": "65.5K","accepted": 0,"rejected": 0,"stale": 0,"ls_diff": 0.0,"ls_time": "0","diffa": 0.0,"ping": 0},{"id": 2,"url": "btc.foundryusapool.com:25","pool_type": "UserPool","user": "pcs19jpro.19.36x55","status": "working","asic_boost": true,"diff": "65.5K","accepted": 0,"rejected": 0,"stale": 0,"ls_diff": 0.0,"ls_time": "0","diffa": 0.0,"ping": 0},{"id": 3,"url": "DevFee","pool_type": "DevFee","user": "DevFee","status": "offline","asic_boost": false,"diff": "2.05K","accepted": 215,"rejected": 0,"stale": 0,"ls_diff": 2048.0,"ls_time": "0:00:39","diffa": 440320.0,"ping": 0}],"cooling": {"fan_num": 4,"fans": [{"id": 0,"rpm": 5820},{"id": 1,"rpm": 5880},{"id": 2,"rpm": 5850},{"id": 3,"rpm": 5790}],"settings": {"mode": {"name": "manual"}},"fan_duty": 100},"chains": [{"id": 1,"frequency": 400.0,"voltage": 13600,"power_consumption": 878,"hashrate_ideal": 25905.6,"hashrate_rt": 25275.506,"hashrate_percentage": 98.88,"hr_error": 0.0,"hw_errors": 0,"pcb_temp_sens": [{"status": "measure","temp": 37},{"status": "measure","temp": 36},{"status": "measure","temp": 48},{"status": "measure","temp": 50}],"chip_temp_sens": [{"status": "measure","temp": 52},{"status": "measure","temp": 51},{"status": "measure","temp": 63},{"status": "measure","temp": 65}],"pcb_temp": {"min": 36,"max": 50},"chip_temp": {"min": 51,"max": 65},"chip_statuses": {"red": 0,"orange": 0,"grey": 126},"status": {"state": "mining"}},{"id": 2,"frequency": 400.0,"voltage": 13600,"power_consumption": 877,"hashrate_ideal": 25905.6,"hashrate_rt": 26131.725,"hashrate_percentage": 100.37,"hr_error": 0.0,"hw_errors": 0,"pcb_temp_sens": [{"status": "measure","temp": 38},{"status": "measure","temp": 37},{"status": "measure","temp": 48},{"status": "measure","temp": 49}],"chip_temp_sens": [{"status": "measure","temp": 53},{"status": "measure","temp": 52},{"status": "measure","temp": 63},{"status": "measure","temp": 64}],"pcb_temp": {"min": 37,"max": 49},"chip_temp": {"min": 52,"max": 64},"chip_statuses": {"red": 0,"orange": 1,"grey": 125},"status": {"state": "mining"}},{"id": 3,"frequency": 400.0,"voltage": 13600,"power_consumption": 876,"hashrate_ideal": 25905.6,"hashrate_rt": 26419.26,"hashrate_percentage": 101.32,"hr_error": 0.0,"hw_errors": 0,"pcb_temp_sens": [{"status": "measure","temp": 36},{"status": "measure","temp": 36},{"status": "measure","temp": 49},{"status": "measure","temp": 49}],"chip_temp_sens": [{"status": "measure","temp": 51},{"status": "measure","temp": 51},{"status": "measure","temp": 64},{"status": "measure","temp": 64}],"pcb_temp": {"min": 36,"max": 49},"chip_temp": {"min": 51,"max": 64},"chip_statuses": {"red": 0,"orange": 0,"grey": 126},"status": {"state": "mining"}}],"found_blocks": 0,"best_share": 198441277}}"#;
        let _: Summary = from_str(&s).unwrap();
    }
}