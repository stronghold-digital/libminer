use serde::{Deserialize, de::Deserializer, Serialize, ser::Serializer};

use crate::Pool;

#[derive(Serialize)]
pub struct VPool {
    pub url: String,
    pub user: String,
    pub pass: String,
    pub order: usize,
}

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
    pub restart_temp: i64,
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
    pub percent: f64,
}

#[derive(Deserialize, Serialize)]
pub struct HotelFee {
    pub enable: bool,
    pub pool: HotelPool,
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
    pub password: Option<PasswordSettings>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn test() {
        let s = r#"{"miner":{"cooling":{"mode":{"name":"auto","param":60}},"devfee":{"region":"auto"},"misc":{"asic_boost":false,"restart_hashrate":0,"restart_temp":85,"disable_restart_unbalanced":false,"disable_chain_break_protection":false,"max_restart_attempts":0,"bitmain_disable_volt_comp":false,"quick_start":false,"higher_volt_offset":100,"tuner_bad_chip_hr_threshold":50},"overclock":{"modded_psu":false,"preset":"3486","globals":{"volt":1400,"freq":610},"chains":[{"freq":0,"chips":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],"disabled":false},{"freq":0,"chips":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],"disabled":false},{"freq":0,"chips":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],"disabled":false}]},"pools":[{"url":"btc.foundryusapool.com:3333","user":"pct19.47.4x243","pass":""},{"url":"btc.foundryusapool.com:443","user":"pct19.47.4x243","pass":""},{"url":"btc.foundryusapool.com:25","user":"pct19.47.4x243","pass":""}],"hotel_fee":{"enable":false,"pool":{"url":"stratum.slushpool.com:3333","worker":"ahx.hotelfee","percent":1.0}}},"ui":{"theme":"auto","dark_side_pane":false,"disable_animation":false,"locale":"en","timezone":"GMT","consts":{"cooling":{"min_fan_pwm":10,"min_target_temp":20,"max_target_temp":100},"overclock":{"max_voltage":1535,"min_voltage":1200,"default_voltage":1340,"max_freq":1000,"min_freq":50,"default_freq":600,"warn_freq":750,"max_voltage_stock_psu":1500},"timezones":[["GMT-11","GMT-11"],["GMT-10","GMT-10"],["GMT-9","GMT-09"],["GMT-8","GMT-08"],["GMT-7","GMT-07"],["GMT-6","GMT-06"],["GMT-5","GMT-05"],["GMT-4","GMT-04"],["GMT-3","GMT-03"],["GMT-2","GMT-02"],["GMT-1","GMT-01"],["GMT","GMT"],["GMT+1","GMT+01"],["GMT+2","GMT+02"],["GMT+3","GMT+03"],["GMT+4","GMT+04"],["GMT+5","GMT+05"],["GMT+6","GMT+06"],["GMT+7","GMT+07"],["GMT+8","GMT+08"],["GMT+9","GMT+09"],["GMT+10","GMT+10"],["GMT+11","GMT+11"],["GMT+12","GMT+12"]]}},"regional":{"timezone":{"current":"GMT"}},"network":{"hostname":"Antminer","dhcp":true,"ipaddress":"192.168.15.44","netmask":"255.255.255.0","gateway":"192.168.15.1","dnsservers":["192.168.15.1","1.1.1.1"]},"ssh":{"enabled":true,"port":22},"password":null,"layout":null,"boot":null}"#;
        let settings: Settings = from_str(s).unwrap();
        assert_eq!(settings.miner.pools.len(), 3);
        assert_eq!(settings.miner.pools[0].url, "btc.foundryusapool.com:3333");
        assert_eq!(settings.miner.pools[0].username, "pct19.47.4x243");
    }
}