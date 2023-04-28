use serde::Deserialize;

#[derive(Deserialize)]
pub struct Network {
    pub mac: String,
    pub ip: String,
    pub netmask: String,
    pub gateway: String,
    pub dns: Vec<String>,
    pub hostname: String,
}

#[derive(Deserialize)]
pub struct System {
    pub os: String,
    pub file_system_version: String,
    pub mem_total: usize,
    pub mem_free: usize,
    pub mem_free_percent: u8,
    pub mem_buf: usize,
    pub mem_buf_percent: u8,
    pub network_status: Network,
    pub uptime: String,
}

#[derive(Deserialize)]
pub struct Info {
    pub miner: String,
    pub model: String,
    pub fw_name: String,
    pub fw_version: String,
    pub platform: String,
    pub install_type: String,
    pub build_time: String,
    pub system: System,
}