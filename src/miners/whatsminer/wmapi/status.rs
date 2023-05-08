use serde::{Deserialize, de};

pub use crate::miners::common::StatusCode;

#[derive(Deserialize, Debug)]
pub struct Status {
    #[serde(rename = "STATUS")]
    pub status: StatusCode,
    #[serde(rename = "When")]
    pub when: Option<usize>,
    #[serde(rename = "Code")]
    pub code: Option<usize>,
    #[serde(rename = "Msg")]
    pub msg: String,
    #[serde(rename = "Description")]
    pub description: Option<String>,
}

fn deserialize_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(de::Error::custom(format!("invalid bool string: {}", s))),
    }
}

#[derive(Deserialize, Debug)]
pub struct BtStatus1 {
    #[serde(deserialize_with = "deserialize_bool")]
    pub btmineroff: bool,
    #[serde(rename = "Firmware Version", alias = "FirmwareVersion")]
    pub firmware_version: String,
}

#[derive(Deserialize, Debug)]
pub struct BtStatus2 {
    #[serde(deserialize_with = "deserialize_bool")]
    pub mineroff: bool,
    #[serde(rename = "FirmwareVersion")]
    pub firmware_version: String,
    pub power_mode: String,
    pub hash_percent: String,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum BtStatus {
    V1(BtStatus1),
    V2(BtStatus2),
}

impl BtStatus {
    pub fn firmware_version(&self) -> &str {
        match self {
            BtStatus::V1(status) => &status.firmware_version,
            BtStatus::V2(status) => &status.firmware_version,
        }
    }

    pub fn mineroff(&self) -> bool {
        match self {
            BtStatus::V1(status) => status.btmineroff,
            BtStatus::V2(status) => status.mineroff,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct BtStatusResp {
    #[serde(rename = "STATUS")]
    pub status: StatusCode,
    #[serde(rename = "When")]
    pub when: Option<usize>,
    #[serde(rename = "Code")]
    pub code: Option<usize>,
    #[serde(rename = "Msg")]
    pub msg: BtStatus,
    #[serde(rename = "Description")]
    pub description: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bt_status() {
        let json = r#"{"btmineroff":"true","Firmware Version":"1.0.0"}"#;
        let status: BtStatus = serde_json::from_str(json).unwrap();
        assert!(matches!(status, BtStatus::V1(_)));
        assert_eq!(status.mineroff(), true);
        assert_eq!(status.firmware_version(), "1.0.0");
    }

    #[test]
    fn test_bt_statusv2() {
        let json = r#"{"mineroff":"true","FirmwareVersion":"1.0.0","power_mode":"","hash_percent":""}"#;
        let status: BtStatus = serde_json::from_str(json).unwrap();
        assert!(matches!(status, BtStatus::V2(_)));
        assert_eq!(status.mineroff(), true);
        assert_eq!(status.firmware_version(), "1.0.0");
    }
}
