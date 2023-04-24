use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Network {
    pub dhcp4: bool,
    pub dns: String,
    #[serde(rename = "dnsBak")]
    pub dns_bak: String,
    pub gateway: String,
    #[serde(rename = "hardwareAddress")]
    pub hardware_address: String,
    #[serde(rename = "interfaceName")]
    pub interface_name: String,
    pub ip: String,
    pub netmask: String,
}

#[derive(Deserialize, Debug)]
pub struct NetworkResponse {
    pub code: usize,
    pub data: Network,
    pub message: String,
}