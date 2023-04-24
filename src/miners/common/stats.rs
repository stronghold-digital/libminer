use serde::Deserialize;
use crate::miners::common::*;

#[derive(Debug, Deserialize)]
pub struct StatsShared {
    #[serde(rename = "STATS")]
    pub stats: usize,
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Elapsed")]
    pub elapsed: usize,
    #[serde(rename = "Calls")]
    pub calls: usize,
    #[serde(rename = "Wait")]
    pub wait: f64,
    #[serde(rename = "Max")]
    pub max: f64,
    #[serde(rename = "Min")]
    pub min: f64,
}

#[derive(Deserialize, Debug)]
pub struct DevStats {
    #[serde(flatten)]
    pub shared: StatsShared,
    #[serde(rename = "Type")]
    pub type_: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct PoolStats {
    #[serde(flatten)]
    pub shared: StatsShared,
    #[serde(rename = "Type")]
    pub type_: String,
    #[serde(rename = "Pool Calls")]
    pub pool_calls: usize,
    #[serde(rename = "Pool Attempts")]
    pub pool_attempts: usize,
    #[serde(rename = "Pool Wait")]
    pub pool_wait: f64,
    #[serde(rename = "Pool Max")]
    pub pool_max: f64,
    #[serde(rename = "Pool Min")]
    pub pool_min: f64,
    #[serde(rename = "Pool Av")]
    pub pool_av: f64,
    #[serde(rename = "Work Had Roll Time")]
    pub work_had_roll_time: bool,
    #[serde(rename = "Work Can Roll")]
    pub work_can_roll: bool,
    #[serde(rename = "Work Had Expire")]
    pub work_had_expire: bool,
    #[serde(rename = "Work Roll Time")]
    pub work_roll_time: usize,
    #[serde(rename = "Work Diff")]
    pub work_diff: f64,
    #[serde(rename = "Min Diff")]
    pub min_diff: f64,
    #[serde(rename = "Max Diff")]
    pub max_diff: f64,
    #[serde(rename = "Min Diff Count")]
    pub min_diff_count: usize,
    #[serde(rename = "Max Diff Count")]
    pub max_diff_count: usize,
    #[serde(rename = "Times Sent")]
    pub times_sent: usize,
    #[serde(rename = "Bytes Sent")]
    pub bytes_sent: usize,
    #[serde(rename = "Times Recv")]
    pub times_recv: usize,
    #[serde(rename = "Bytes Recv")]
    pub bytes_recv: usize,
    #[serde(rename = "Net Bytes Sent")]
    pub net_bytes_sent: usize,
    #[serde(rename = "Net Bytes Recv")]
    pub net_bytes_recv: usize,
}

/// Antminer stats section including model and version
#[derive(Deserialize, Debug)]
pub struct AmVersion {
    #[serde(rename = "BMMiner")]
    pub bmminer: String,
    #[serde(rename = "Miner")]
    pub miner: String,
    #[serde(rename = "CompileTime")]
    pub compile_time: String,
    #[serde(rename = "Type")]
    pub type_: String,
}

/// Antminer stats section including current device stats
#[derive(Deserialize, Debug)]
pub struct AmStats {
    #[serde(flatten)]
    pub shared: StatsShared,
    #[serde(rename = "GHS 5s")]
    pub ghs_5s: f64,
    #[serde(rename = "GHS av")]
    pub ghs_av: f64,
    #[serde(rename = "rate_30m")]
    pub rate_30m: f64,
    #[serde(rename = "Mode")]
    pub mode: usize,
    pub miner_count: usize,
    pub frequency: usize,
    pub fan_num: usize,
    pub fan1: usize,
    pub fan2: usize,
    pub fan3: usize,
    pub fan4: usize,
    pub temp_num: usize,
    pub temp1: usize,
    pub temp2: usize,
    pub temp2_1: usize,
    pub temp2_2: usize,
    pub temp2_3: usize,
    pub temp3: usize,
    pub temp_pcb1: String,
    pub temp_pcb2: String,
    pub temp_pcb3: String,
    pub temp_pcb4: String,
    pub temp_chip1: String,
    pub temp_chip2: String,
    pub temp_chip3: String,
    pub temp_chip4: String,
    pub temp_pic1: String,
    pub temp_pic2: String,
    pub temp_pic3: String,
    pub temp_pic4: String,
    pub total_rateideal: f64,
    pub rate_unit: String,
    pub total_freqavg: usize,
    pub total_acn: usize,
    #[serde(rename = "total rate")]
    pub total_rate: f64,
    pub temp_max: usize,
    pub no_matching_work: usize,
    pub chain_acn1: usize,
    pub chain_acn2: usize,
    pub chain_acn3: usize,
    pub chain_acn4: usize,
    pub chain_acs1: Option<String>,
    pub chain_acs2: Option<String>,
    pub chain_acs3: Option<String>,
    pub chain_acs4: Option<String>,
    pub chain_hw1: usize,
    pub chain_hw2: usize,
    pub chain_hw3: usize,
    pub chain_hw4: usize,
    pub chain_rate1: String,
    pub chain_rate2: String,
    pub chain_rate3: String,
    pub chain_rate4: String,
    pub freq1: usize,
    pub freq2: usize,
    pub freq3: usize,
    pub freq4: usize,
    pub miner_version: String,
    pub miner_id: String,
}

/// Avalon stats section
/// wtf Avalon?
#[derive(Deserialize, Debug)]
pub struct AvaStats {
    #[serde(flatten)]
    pub shared: StatsShared,
    #[serde(rename = "MM ID0")]
    pub mm_id0: String,
}

/// MinerVa Status section
#[derive(Deserialize, Debug)]
pub struct MvStats {
    #[serde(flatten)]
    pub shared: StatsShared,
    #[serde(rename = "Type")]
    pub type_: String,
    #[serde(rename = "Chain ID")]
    pub chain_id: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Enabled")]
    pub enabled: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "MHS av")]
    pub mhs_av: f64,
    #[serde(rename = "MHS 5s")]
    pub mhs_5s: f64,
    #[serde(rename = "MHS 1m")]
    pub mhs_1m: f64,
    #[serde(rename = "MHS 5m")]
    pub mhs_5m: f64,
    #[serde(rename = "MHS 15m")]
    pub mhs_15m: f64,
    #[serde(rename = "Accepted")]
    pub accepted: usize,
    #[serde(rename = "Rejected")]
    pub rejected: usize,
    #[serde(rename = "Hardware Errors")]
    pub hw_errors: usize,
    #[serde(rename = "Diff1 Work")]
    pub diff1_work: usize,
    #[serde(rename = "Difficulty Accepted")]
    pub diff_accepted: f64,
    #[serde(rename = "Difficulty Rejected")]
    pub diff_rejected: f64,
    #[serde(rename = "Last Share Difficulty")]
    pub last_share_diff: f64,
    #[serde(rename = "Last Valid Work")]
    pub last_valid_work: usize,
    #[serde(rename = "Device Hardware%")]
    pub dev_hw: f64,
    #[serde(rename = "Device Rejected%")]
    pub dev_rejected: f64,
    #[serde(rename = "Device Elapsed")]
    pub dev_elapsed: usize,
    #[serde(rename = "Chain Enabled")]
    pub chain_enabled: String,
    #[serde(rename = "Chain BIN")]
    pub chain_bin: String,
    #[serde(rename = "Chip Count")]
    pub chip_count: usize,
    #[serde(rename = "Device Diff")]
    pub dev_diff: usize,
    #[serde(rename = "Device Freq")]
    pub dev_freq: usize,
    #[serde(rename = "Temp Avg")]
    pub temp_avg: f64,
    #[serde(rename = "Voltage Avg")]
    pub voltage_avg: f64,
    #[serde(rename = "Voltage")]
    pub voltage: f64,
    #[serde(rename = "Power Consumption")]
    pub power_consumption: f64,
    #[serde(rename = "Fan Duty")]
    pub fan_duty: f64,
    #[serde(rename = "Fan0 Speed")]
    pub fan0_speed: u32,
}

/// Enum of a variety of stat sections that can be returned
/// from {"command": "stats"}
#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum Stats {
    Pool(PoolStats), // Ensure PoolStats is attempted first
    MvStats(MvStats),
    AvaStats(AvaStats),
    Dev(DevStats),
    AmVersion(AmVersion),
    AmStats(AmStats),
}

#[derive(Deserialize, Debug)]
pub struct StatsResp {
    #[serde(rename = "STATUS")]
    pub status: [Status; 1],
    #[serde(rename = "STATS")]
    pub stats: Option<Vec<Stats>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn test_mvstats() {
        let s = r#"{"STATUS":[{"STATUS":"S","When":1502358955,"Code":70,"Msg":"CGMiner stats","Description":"cgminer 4.10.0"}],"STATS":[{"STATS":0,"ID":"C30120","Elapsed":521572,"Calls":0,"Wait":0.000000,"Max":0.000000,"Min":99999999.000000,"Type":"Minerva","Chain ID":"1","Name":"C3012","Enabled":"Y","Status":"Alive","MHS av":29317556.42,"MHS 5s":27223474.34,"MHS 1m":29507931.81,"MHS 5m":29726644.35,"MHS 15m":29202155.72,"Accepted":13625,"Rejected":35,"Hardware Errors":108092,"Diff1 Work":3476823,"Difficulty Accepted":3543769088.00000000,"Difficulty Rejected":8945664.00000000,"Last Share Difficulty":262144.00000000,"Last Valid Work":1502358954,"Device Hardware%":3.0152,"Device Rejected%":257.2942,"Device Elapsed":521572,"Chain Enabled":"Y","Chain BIN":"20","Chip Count":120,"Device Diff":1024,"Device Freq":520,"Temp Avg":80.37,"Voltage Avg":316.319,"Voltage":12800.000,"Power Consumption":2045.568,"Fan Duty":52.34,"Fan0 Speed":3840},{"STATS":1,"ID":"C30120","Elapsed":521572,"Calls":0,"Wait":0.000000,"Max":0.000000,"Min":99999999.000000,"Type":"Minerva","Chain ID":"2","Name":"C3012","Enabled":"Y","Status":"Alive","MHS av":29140571.21,"MHS 5s":36755341.94,"MHS 1m":30322549.44,"MHS 5m":29411440.77,"MHS 15m":29278686.58,"Accepted":13693,"Rejected":31,"Hardware Errors":149229,"Diff1 Work":3455834,"Difficulty Accepted":3560816640.00000000,"Difficulty Rejected":8126464.00000000,"Last Share Difficulty":262144.00000000,"Last Valid Work":1502358955,"Device Hardware%":4.1394,"Device Rejected%":235.1520,"Device Elapsed":521572,"Chain Enabled":"Y","Chain BIN":"20","Chip Count":120,"Device Diff":1024,"Device Freq":520,"Temp Avg":87.21,"Voltage Avg":315.697,"Voltage":12800.000,"Power Consumption":2045.568,"Fan Duty":52.34,"Fan0 Speed":3840},{"STATS":2,"ID":"POOL0","Elapsed":521572,"Calls":0,"Wait":0.000000,"Max":0.000000,"Min":99999999.000000,"Type":"Minerva","Pool Calls":0,"Pool Attempts":0,"Pool Wait":0.000000,"Pool Max":0.000000,"Pool Min":99999999.000000,"Pool Av":0.000000,"Work Had Roll Time":false,"Work Can Roll":false,"Work Had Expire":false,"Work Roll Time":0,"Work Diff":262144.00000000,"Min Diff":4096.00000000,"Max Diff":262144.00000000,"Min Diff Count":1969,"Max Diff Count":34602385,"Times Sent":27397,"Bytes Sent":3702515,"Times Recv":45054,"Bytes Recv":23563421,"Net Bytes Sent":3702515,"Net Bytes Recv":23563421},{"STATS":3,"ID":"POOL1","Elapsed":521572,"Calls":0,"Wait":0.000000,"Max":0.000000,"Min":99999999.000000,"Type":"Minerva","Pool Calls":0,"Pool Attempts":0,"Pool Wait":0.000000,"Pool Max":0.000000,"Pool Min":99999999.000000,"Pool Av":0.000000,"Work Had Roll Time":false,"Work Can Roll":false,"Work Had Expire":false,"Work Roll Time":0,"Work Diff":65536.00000000,"Min Diff":65536.00000000,"Max Diff":65536.00000000,"Min Diff Count":5,"Max Diff Count":5,"Times Sent":3,"Bytes Sent":294,"Times Recv":10,"Bytes Recv":5631,"Net Bytes Sent":294,"Net Bytes Recv":5631},{"STATS":4,"ID":"POOL2","Elapsed":521572,"Calls":0,"Wait":0.000000,"Max":0.000000,"Min":99999999.000000,"Type":"Minerva","Pool Calls":0,"Pool Attempts":0,"Pool Wait":0.000000,"Pool Max":0.000000,"Pool Min":99999999.000000,"Pool Av":0.000000,"Work Had Roll Time":false,"Work Can Roll":false,"Work Had Expire":false,"Work Roll Time":0,"Work Diff":65536.00000000,"Min Diff":65536.00000000,"Max Diff":65536.00000000,"Min Diff Count":1,"Max Diff Count":1,"Times Sent":3,"Bytes Sent":294,"Times Recv":6,"Bytes Recv":1667,"Net Bytes Sent":294,"Net Bytes Recv":1667}],"id":1}"#;
        let stat: StatsResp = from_str(s).unwrap();
        assert!(stat.stats.is_some());
        let stats = stat.stats.unwrap();
        assert_eq!(stats.len(), 5);
        assert!(matches!(stats[0], Stats::MvStats(_)));
    }
}