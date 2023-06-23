use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use lazy_regex::{Regex, Lazy};
use crate::error::Error;
use crate::{Client, Cache};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Pool {
    pub url: String,
    #[serde(rename = "user")]
    pub username: String,
    #[serde(rename = "pass")]
    pub password: Option<String>,
}

impl Default for Pool {
    fn default() -> Self {
        Self {
            url: String::new(),
            username: String::new(),
            password: None,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
pub enum ErrorType {
    ControlBoard,
    HashBoard,
    Fan,
    Temperature,
    Power,
    Network,
    Config,
    Other,
}

#[derive(Debug)]
pub(crate) struct IntMinerError {
    pub re: &'static Lazy<Regex>,
    pub msg: &'static str,
    pub error_type: ErrorType,
}

impl IntMinerError {
    pub fn get_msg(&self, line: &str) -> Option<String> {
        if let Some(caps) = self.re.captures(line) {
            let caps = caps.iter().skip(1);
            let mut msg = self.msg.to_string();
            for cap in caps {
                if let Some(cap) = cap {
                    msg = msg.replacen("{}", cap.as_str(), 1);
                }
            }
            Some(msg)
        } else {
            None
        }
    }

    pub fn get_err(&self, line: &str) -> Option<MinerError> {
        if let Some(msg) = self.get_msg(line) {
            Some(MinerError {
                msg,
                error_type: self.error_type,
            })
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct MinerError {
    pub msg: String,
    pub error_type: ErrorType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Profile {
    Default,
    LowPower,
    Preset { name: String, power: f64, ths: f64 },
    Manual { volt: u32, freq: u32, min_freq: u32, max_freq: u32, min_volt: u32, max_volt: u32, def_volt: u32, def_freq: u32 },
}

#[async_trait]
pub trait Miner {
    fn new(client: Client, ip: String, port: u16) -> Self
        where Self: Sized;
    
    #[allow(unused_mut)]
    fn with_cache(mut self, _cache: Option<Cache>) -> Self
        where Self: Sized {
            self
        }

    fn get_type(&self) -> &'static str;

    async fn get_model(&self) -> Result<String, Error>;

    async fn auth(&mut self, username: &str, password: &str) -> Result<(), Error>;

    async fn reboot(&mut self) -> Result<(), Error>;

    async fn get_hashrate(&self) -> Result<f64, Error>;

    async fn get_power(&self) -> Result<f64, Error>;

    async fn get_nameplate_power(&self) -> Result<f64, Error>;

    async fn get_efficiency(&self) -> Result<f64, Error>;

    async fn get_nameplate_rate(&self) -> Result<f64, Error>;

    async fn get_temperature(&self) -> Result<f64, Error>;

    async fn get_fan_speed(&self) -> Result<Vec<u32>, Error>;

    async fn get_fan_pwm(&self) -> Result<f64, Error>;

    async fn get_pools(&self) -> Result<Vec<Pool>, Error>;

    async fn set_pools(&mut self, pools: Vec<Pool>) -> Result<(), Error>;

    async fn get_sleep(&self) -> Result<bool, Error>;

    async fn set_sleep(&mut self, sleep: bool) -> Result<(), Error>;

    async fn get_blink(&self) -> Result<bool, Error>;

    async fn set_blink(&mut self, blink: bool) -> Result<(), Error>;

    async fn get_logs(&mut self) -> Result<Vec<String>, Error>;

    async fn get_mac(&self) -> Result<String, Error>;

    async fn get_errors(&mut self) -> Result<Vec<MinerError>, Error>;

    async fn get_dns(&self) -> Result<String, Error>;

    async fn get_profile(&self) -> Result<Profile, Error>;

    async fn get_profiles(&self) -> Result<Vec<Profile>, Error>;

    async fn set_profile(&mut self, profile: Profile) -> Result<(), Error>;

    async fn get_hashboard(&mut self) -> Result<String, Error>;
}

pub struct LockMiner {
    _permit: tokio::sync::OwnedSemaphorePermit,
    miner: Box<dyn Miner + Send + Sync>,
}

impl LockMiner {
    pub fn new_locked(miner: Box<dyn Miner + Send + Sync>, permit: tokio::sync::OwnedSemaphorePermit) -> LockMiner {
        LockMiner {
            _permit: permit,
            miner,
        }
    }
}

#[async_trait]
impl Miner for LockMiner {
    fn new(_client: Client, _ip: String, _port: u16) -> Self
        where Self: Sized {
            unimplemented!();
        }

    fn get_type(&self) -> &'static str {
        self.miner.get_type()
    }

    async fn get_model(&self) -> Result<String, Error> {
        self.miner.get_model().await
    }

    async fn auth(&mut self, username: &str, password: &str) -> Result<(), Error> {
        self.miner.auth(username, password).await
    }

    async fn reboot(&mut self) -> Result<(), Error> {
        self.miner.reboot().await
    }

    async fn get_hashrate(&self) -> Result<f64, Error> {
        self.miner.get_hashrate().await
    }

    async fn get_power(&self) -> Result<f64, Error> {
        self.miner.get_power().await
    }

    async fn get_nameplate_power(&self) -> Result<f64, Error> {
        self.miner.get_nameplate_power().await
    }

    async fn get_efficiency(&self) -> Result<f64, Error> {
        self.miner.get_efficiency().await
    }

    async fn get_nameplate_rate(&self) -> Result<f64, Error> {
        self.miner.get_nameplate_rate().await
    }

    async fn get_temperature(&self) -> Result<f64, Error> {
        self.miner.get_temperature().await
    }

    async fn get_fan_speed(&self) -> Result<Vec<u32>, Error> {
        self.miner.get_fan_speed().await
    }

    async fn get_fan_pwm(&self) -> Result<f64, Error> {
        self.miner.get_fan_pwm().await
    }

    async fn get_pools(&self) -> Result<Vec<Pool>, Error> {
        self.miner.get_pools().await
    }

    async fn set_pools(&mut self, pools: Vec<Pool>) -> Result<(), Error> {
        self.miner.set_pools(pools).await
    }

    async fn get_sleep(&self) -> Result<bool, Error> {
        self.miner.get_sleep().await
    }

    async fn set_sleep(&mut self, sleep: bool) -> Result<(), Error> {
        self.miner.set_sleep(sleep).await
    }

    async fn get_blink(&self) -> Result<bool, Error> {
        self.miner.get_blink().await
    }

    async fn set_blink(&mut self, blink: bool) -> Result<(), Error> {
        self.miner.set_blink(blink).await
    }

    async fn get_logs(&mut self) -> Result<Vec<String>, Error> {
        self.miner.get_logs().await
    }

    async fn get_mac(&self) -> Result<String, Error> {
        self.miner.get_mac().await
    }

    async fn get_errors(&mut self) -> Result<Vec<MinerError>, Error> {
        self.miner.get_errors().await
    }

    async fn get_dns(&self) -> Result<String, Error> {
        self.miner.get_dns().await
    }

    async fn get_profile(&self) -> Result<Profile, Error> {
        self.miner.get_profile().await
    }

    async fn get_profiles(&self) -> Result<Vec<Profile>, Error> {
        self.miner.get_profiles().await
    }

    async fn set_profile(&mut self, profile: Profile) -> Result<(), Error> {
        self.miner.set_profile(profile).await
    }

    async fn get_hashboard(&mut self) -> Result<String, Error> {
        self.miner.get_hashboard().await
    }
}
