use serde::{Deserialize, de::Deserializer};

use super::{CoolingSettings, StatusCode, System};

#[derive(Deserialize)]
pub struct MinerSummaryStatus {
    pub miner_state: StatusCode,
    pub miner_state_time: u64,
}

#[derive(Deserialize)]
pub struct TempMinMax {
    pub min: f32,
    pub max: f32,
}

#[derive(PartialEq, Debug)]
pub enum PoolStatus {
    Working,
    Active,
    Offline,
    Disabled,
    Rejecting,
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
            "rejecting" => PoolStatus::Rejecting,
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
    pub ls_diff: f32,
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
    // pub settings: CoolingSettings,
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
    pub temp: f32,
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
    pub frequency: f32,
    pub hashrate_ideal: f64,
    pub hashrate_rt: f32,
    pub pcb_temp_sens: Option<Vec<TempSensor>>,
    pub chip_temp_sens: Option<Vec<TempSensor>>,
    pub chip_temp: Option<TempMinMax>,
    pub pcb_temp: Option<TempMinMax>,
    pub hw_errors: i32,
    pub voltage: i64,
    pub chip_statuses: ChipStatus,
    pub status: ChainStatus,
}

#[derive(Deserialize)]
pub struct MinerSummary {
    pub miner_status: MinerSummaryStatus,
    pub miner_type: String,
    // pub hardware_version: String,
    // pub cgminer_version: String,
    // pub compile_time: String,
    pub average_hashrate: f64,
    pub instant_hashrate: f64,
    pub pcb_temp: TempMinMax,
    pub chip_temp: TempMinMax,
    pub power_usage: f64,
    pub power_efficiency: f64,
    pub hw_errors_percent: f32,
    pub hw_errors: i64,
    pub devfee_percent: f32,
    pub devfee: f32,
    pub pools: Vec<PoolStats>,
    pub cooling: CoolingStats,
    pub chains: Vec<Chain>,
    pub found_blocks: f32,
    pub best_share: i64
}

#[derive(Deserialize)]
pub struct Summary {
    pub miner: Option<MinerSummary>,
    // pub system: System,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn test_de() {
        let s = r#"{"system":{"os":"GNU/Linux","miner_name":"Antminer","file_system_version":"","mem_total":233712,"mem_free":195048,"mem_free_percent":83,"mem_buf":19668,"mem_buf_percent":8,"network_status":{"mac":"4E:9F:85:7B:57:7C","dhcp":true,"ip":"10.138.11.63","netmask":"255.255.254.0","gateway":"10.138.11.254","dns":["208.67.220.220","208.67.222.222"],"hostname":"Antminer"},"uptime":"9 days,  3:58"},"miner":{"miner_status":{"miner_state":"mining","miner_state_time":6412},"miner_type":"Antminer S19 (Vnish 1.2.0-beta10)","hardware_version":"49.0.1.3","cgminer_version":"4.11.1","compile_time":"Mon Apr 17 08:08:58 UTC 2023","average_hashrate":67.4193,"instant_hashrate":66.62962,"pcb_temp":{"min":13,"max":39},"chip_temp":{"min":23,"max":51},"power_usage":3733.0,"power_efficiency":55.369904,"hw_errors_percent":0.0,"hw_errors":0,"devfee_percent":0.0,"devfee":0.0,"pools":[{"id":0,"url":"btc.foundryusapool.com:3333","pool_type":"UserPool","user":"s19s.11x63","status":"active","asic_boost":true,"diff":"262K","accepted":153,"rejected":148,"stale":0,"ls_diff":262144.0,"ls_time":"0:02:40","diffa":31850496.0},{"id":1,"url":"btc.foundryusapool.com:443","pool_type":"UserPool","user":"s19s.11x63","status":"working","asic_boost":true,"diff":"65.5K","accepted":0,"rejected":0,"stale":0,"ls_diff":0.0,"ls_time":"0","diffa":0.0},{"id":2,"url":"btc.foundryusapool.com:25","pool_type":"UserPool","user":"s19s.11x63","status":"working","asic_boost":true,"diff":"65.5K","accepted":0,"rejected":0,"stale":0,"ls_diff":0.0,"ls_time":"0","diffa":0.0},{"id":3,"url":"DevFee","pool_type":"DevFee","user":"DevFee","status":"unknown","asic_boost":false,"diff":"","accepted":0,"rejected":0,"stale":0,"ls_diff":0.0,"ls_time":"0","diffa":0.0}],"cooling":{"fan_num":4,"fans":[{"id":0,"rpm":6360},{"id":1,"rpm":5040},{"id":2,"rpm":6120},{"id":3,"rpm":5040}],"settings":{"mode":{"name":"manual","param":100}},"fan_duty":100},"chains":[{"id":1,"frequency":680.0,"voltage":14000,"power_usage":1242,"hashrate_ideal":32196.64,"hashrate_rt":31563.244,"hashrate_percentage":99.53,"hw_errors":0,"pcb_temp_sens":[{"status":"measure","temp":19},{"status":"error","temp":13},{"status":"measure","temp":36},{"status":"measure","temp":37}],"chip_temp_sens":[{"status":"measure","temp":29},{"status":"error","temp":23},{"status":"measure","temp":46},{"status":"measure","temp":47}],"chip_temp":{"min":23,"max":47},"chip_statuses":{"red":0,"orange":0,"grey":76},"status":{"state":"mining","description":""}},{"id":2,"frequency":680.0,"voltage":14000,"power_usage":1248,"hashrate_ideal":32196.64,"hashrate_rt":31518.17,"hashrate_percentage":99.05,"hw_errors":0,"pcb_temp_sens":[{"status":"measure","temp":20},{"status":"measure","temp":24},{"status":"measure","temp":38},{"status":"measure","temp":40}],"chip_temp_sens":[{"status":"measure","temp":30},{"status":"measure","temp":34},{"status":"measure","temp":48},{"status":"measure","temp":50}],"chip_temp":{"min":30,"max":50},"chip_statuses":{"red":0,"orange":0,"grey":76},"status":{"state":"mining","description":""}},{"id":3,"frequency":680.0,"voltage":14000,"power_usage":1243,"hashrate_ideal":32196.64,"hashrate_rt":32103.768,"hashrate_percentage":99.77,"hw_errors":0,"pcb_temp_sens":[{"status":"measure","temp":23},{"status":"measure","temp":17},{"status":"measure","temp":41},{"status":"measure","temp":39}],"chip_temp_sens":[{"status":"measure","temp":33},{"status":"measure","temp":27},{"status":"measure","temp":51},{"status":"measure","temp":49}],"chip_temp":{"min":27,"max":51},"chip_statuses":{"red":0,"orange":0,"grey":76},"status":{"state":"mining","description":""}}],"found_blocks":0,"best_share":101399818}}"#;
        let _: Summary = from_str(&s).unwrap();
    }
}