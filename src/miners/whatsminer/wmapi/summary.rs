use serde::Deserialize;

use super::Status;

#[derive(Deserialize, Debug)]
pub struct Summary {
    #[serde(rename = "Elapsed")]
    pub elapsed: usize,
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
    #[serde(rename = "HS RT")]
    pub hs_rt: Option<f64>,
    #[serde(rename = "Found Blocks")]
    pub found_blocks: Option<usize>,
    #[serde(rename = "Getworks")]
    pub getworks: Option<usize>,
    #[serde(rename = "Accepted")]
    pub accepted: usize,
    #[serde(rename = "Rejected")]
    pub rejected: usize,
    #[serde(rename = "Hardware Errors")]
    pub hardware_errors: Option<usize>,
    #[serde(rename = "Utility")]
    pub utility: Option<f64>,
    #[serde(rename = "Discarded")]
    pub discarded: Option<usize>,
    #[serde(rename = "Stale")]
    pub stale: Option<usize>,
    #[serde(rename = "Get Failures")]
    pub get_failures: Option<usize>,
    #[serde(rename = "Local Work")]
    pub local_work: Option<usize>,
    #[serde(rename = "Remote Failures")]
    pub remote_failures: Option<usize>,
    #[serde(rename = "Network Blocks")]
    pub network_blocks: Option<usize>,
    #[serde(rename = "Total MH")]
    pub total_mh: f64,
    #[serde(rename = "Work Utility")]
    pub work_utility: Option<f64>,
    #[serde(rename = "Difficulty Accepted")]
    pub difficulty_accepted: Option<f64>,
    #[serde(rename = "Difficulty Rejected")]
    pub difficulty_rejected: Option<f64>,
    #[serde(rename = "Difficulty Stale")]
    pub difficulty_stale: Option<f64>,
    #[serde(rename = "Best Share")]
    pub best_share: Option<usize>,
    #[serde(rename = "Temperature")]
    pub temperature: f64,
    pub freq_avg: usize,
    #[serde(rename = "Fan Speed In")]
    pub fan_speed_in: u32,
    #[serde(rename = "Fan Speed Out")]
    pub fan_speed_out: u32,
    #[serde(rename = "Voltage")]
    pub voltage: Option<usize>,
    #[serde(rename = "Power")]
    pub power: usize,
    //#[serde(rename = "Power Rate")]
    //pub power_rate: Option<f64>,
    #[serde(rename = "Power_RT")]
    pub power_rt: Option<usize>,
    #[serde(rename = "Device Hardware%")]
    pub device_hardware_per: Option<f64>,
    #[serde(rename = "Device Rejected%")]
    pub device_rejected_per: Option<f64>,
    #[serde(rename = "Pool Rejected%")]
    pub pool_rejected_per: f64,
    #[serde(rename = "Pool Stale%")]
    pub pool_stale_per: f64,
    #[serde(rename = "Last getwork")]
    pub last_getwork: Option<usize>,
    #[serde(rename = "Uptime")]
    pub uptime: usize,
    // #[serde(rename = "Power Current")]
    // pub power_current: Option<f64>,
    #[serde(rename = "Power Fanspeed")]
    pub power_fanspeed: Option<f64>,
    //TODO: Error codes are reported like this
    #[serde(rename = "Error Code 0")]
    pub error_code_0: Option<usize>,
    #[serde(rename = "Error 0 Time")]
    pub error_0_time: Option<String>,
    #[serde(rename = "Error Code Count")]
    pub error_code_count: Option<usize>,
    #[serde(rename = "Factory Error Code Count")]
    pub factory_error_code_count: Option<usize>,
    #[serde(rename = "Security Mode")]
    pub security_mode: Option<usize>,
    #[serde(rename = "Liquid Cooling")]
    pub liquid_cooling: Option<bool>,
    // #[serde(rename = "Hash Stable")]
    // pub hash_stable: bool,
    // #[serde(rename = "Hash Stable Cost Seconds")]
    // pub hash_stable_cost_seconds: Option<usize>,
    //#[serde(rename = "Hash Deviation%")]
    //pub hash_deviation_per: Option<f64>,
    #[serde(rename = "Target Freq")]
    pub target_freq: usize,
    #[serde(rename = "Target MHS")]
    pub target_mhs: f64,
    #[serde(rename = "Env Temp")]
    pub env_temp: Option<f64>,
    #[serde(rename = "Power Mode")]
    pub power_mode: String,
    #[serde(rename = "Firmware Version")]
    pub firmware_version: Option<String>,
    #[serde(rename = "MAC")]
    pub mac: Option<String>,
    #[serde(rename = "Factory GHS")]
    pub factory_ghs: usize,
    #[serde(rename = "Power Limit")]
    pub power_limit: usize,
    #[serde(rename = "Chip Temp Min")]
    pub chip_temp_min: f64,
    #[serde(rename = "Chip Temp Max")]
    pub chip_temp_max: f64,
    #[serde(rename = "Chip Temp Avg")]
    pub chip_temp_avg: f64,
    #[serde(rename = "Debug")]
    pub debug: Option<String>,
    #[serde(rename = "Btminer Fast Boot")]
    pub fast_boot: Option<String>,
}

impl Summary {
    pub fn hashrate_ths(&self) -> f64 {
        match self.hs_rt {
            Some(hs_rt) => hs_rt / 1000000.0,
            None => self.mhs_5s / 1000000.0,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct SummaryResp {
    #[serde(rename = "STATUS")]
    pub status: [Status; 1],
    #[serde(rename = "SUMMARY")]
    pub summary: Vec<Summary>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_deserializes() {
        let input = r#"{"STATUS":[{"STATUS":"S","Msg":"Summary"}],"SUMMARY":[{"Elapsed":10256,"MHS av":86344408.19,"MHS 5s":104558122.51,"MHS 1m":87932837.07,"MHS 5m":86351357.73,"MHS 15m":86295510.58,"HS RT":86351357.73,"Accepted":786,"Rejected":2,"Total MH":885555462530.0000,"Temperature":77.75,"freq_avg":650,"Fan Speed In":2880,"Fan Speed Out":2850,"Power":3431,"Power Rate":39.73,"Pool Rejected%":0.2611,"Pool Stale%":0.0000,"Uptime":10974,"Security Mode":0,"Hash Stable":true,"Hash Stable Cost Seconds":426,"Hash Deviation%":0.0559,"Target Freq":637,"Target MHS":85788612,"Env Temp":13.25,"Power Mode":"Normal","Factory GHS":86022,"Power Limit":3600,"Chip Temp Min":69.19,"Chip Temp Max":97.58,"Chip Temp Avg":86.19,"Debug":"","Btminer Fast Boot":"disable"}],"id":1}"#;
        let _: SummaryResp = serde_json::from_str(input).unwrap();
        let inpu2 = r#"{"STATUS":[{"STATUS":"S","Msg":"Summary"}],"SUMMARY":[{"Elapsed":23397,"MHS av":91598055.38,"MHS 5s":107889994.42,"MHS 1m":91464807.14,"MHS 5m":91611411.38,"MHS 15m":91577620.93,"HS RT":91611411.38,"Accepted":1713,"Rejected":3,"Total MH":2143119015066.00,"Temperature":80.00,"freq_avg":521,"Fan Speed In":3510,"Fan Speed Out":3540,"Power":3583,"Power Rate":39.12,"Pool Rejected%":0.1604,"Pool Stale%":0.0000,"Uptime":26782,"Hash Stable":true,"Hash Stable Cost Seconds":1293,"Hash Deviation%":0.1023,"Target Freq":478,"Target MHS":91370226.00,"Env Temp":13.50,"Power Mode":"Normal","Factory GHS":90859,"Power Limit":3600,"Chip Temp Min":59.46,"Chip Temp Max":106.14,"Chip Temp Avg":78.58,"Debug":"","Btminer Fast Boot":"disable"}],"id":1}"#;
        let _: SummaryResp = serde_json::from_str(inpu2).unwrap();
    }
}
