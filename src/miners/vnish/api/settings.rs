use serde::{Deserialize, de::Deserializer, Serialize, ser::Serializer};

use crate::Pool;

#[derive(Deserialize, Serialize)]
struct ICoolingMode {
    name: String,
    param: Option<u8>,
}

pub enum CoolingMode {
    Auto(u8),
    Manual(u8),
    Immersion,
}

impl<'de> Deserialize<'de> for CoolingMode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let cm = ICoolingMode::deserialize(deserializer)?;
        match cm.name.as_str() {
            "auto" => Ok(CoolingMode::Auto(cm.param.ok_or(serde::de::Error::custom("Missing cooling mode parameter"))?)),
            "manual" => Ok(CoolingMode::Manual(cm.param.ok_or(serde::de::Error::custom("Missing cooling mode parameter"))?)),
            "immers" => Ok(CoolingMode::Immersion),
            _ => Err(serde::de::Error::custom(format!("Unknown cooling mode: {}", cm.name))),
        }
    }
}

impl Serialize for CoolingMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let ser = match self {
            CoolingMode::Auto(param) => ICoolingMode {
                name: "auto".to_string(),
                param: Some(*param),
            },
            CoolingMode::Manual(param) => ICoolingMode {
                name: "manual".to_string(),
                param: Some(*param),
            },
            CoolingMode::Immersion => ICoolingMode {
                name: "immers".to_string(),
                param: None,
            },
        };
        ser.serialize(serializer)
    }
}

#[derive(Deserialize, Serialize)]
pub struct CoolingSettings {
    pub mode: CoolingMode,
}

#[derive(Deserialize, Serialize)]
pub struct DevFee {
    pub region: String,
}

#[derive(Deserialize, Serialize)]
pub struct MiscSettings {
    pub asic_boost: bool,
    pub restart_hashrate: i64,
    pub restart_temperature: i64,
    pub disable_restart_unbalanced: bool,
    pub disable_chain_break_protection: bool,
    pub max_restart_attempts: usize,
    pub bitmain_disable_volt_comp: bool,
    pub quick_start: bool,
    pub higher_volt_offset: usize,
    pub tuner_bad_chip_hr_threshold: usize,
}

#[derive(Deserialize, Serialize)]
pub struct GlobalOverclockSettings {
    pub freq: usize,
    pub volt: usize,
}

#[derive(Deserialize, Serialize)]
pub struct ChainSettings {
    pub freq: usize,
    pub chips: Vec<usize>,
}

#[derive(Deserialize, Serialize)]
pub struct OverclockSettings {
    pub preset: String,
    pub globals: GlobalOverclockSettings,
    pub chains: Vec<ChainSettings>,
}

#[derive(Deserialize, Serialize)]
pub struct HotelPool {
    pub url: String,
    pub worker: String,
    pub percent: u8,
}

#[derive(Deserialize, Serialize)]
pub struct HotelFee {
    pub enabled: bool,
    pub pool: Pool,
}

#[derive(Deserialize, Serialize)]
pub struct MinerSettings {
    pub cooling: CoolingSettings,
    pub devfee: DevFee,
    pub misc: MiscSettings,
    pub overclock: OverclockSettings,
    pub pools: Vec<Pool>,
    pub hotel_fee: HotelFee,
}

#[derive(Deserialize, Serialize)]
pub struct TzSettings {
    pub current: String,
}

#[derive(Deserialize, Serialize)]
pub struct RegionalSettings {
    pub timezone: TzSettings,
}

#[derive(Deserialize, Serialize)]
pub struct NetworkSettings {
    pub mac: String,
    pub ip: String,
    pub netmask: String,
    pub gateway: String,
    pub dns: Vec<String>,
    pub hostname: String,
    pub dhcp: bool,
}

#[derive(Deserialize, Serialize)]
pub struct SshSettings {
    pub port: u16,
    pub enabled: bool,
}

#[derive(Deserialize, Serialize)]
pub struct PasswordSettings {
    pub current: String,
    pub pw: String,
}

#[derive(Deserialize, Serialize)]
pub struct Settings {
    pub miner: MinerSettings,
    // ui - UI shit we don't care about
    pub regional: RegionalSettings,
    pub ssh: SshSettings,
    pub password: PasswordSettings,
}