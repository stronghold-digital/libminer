use async_trait::async_trait;
use serde::Serialize;

use crate::error::Error;
use crate::Client;

// We can implement serialize directly on here for antminer
#[derive(Debug, Serialize)]
pub struct Pool {
    pub url: String,
    #[serde(rename = "user")]
    pub username: String,
    #[serde(rename = "pass")]
    pub password: Option<String>,
}

#[async_trait]
pub trait Miner {
    fn new(client: Client, ip: String, port: u16) -> Self
        where Self: Sized;

    fn get_type(&self) -> &'static str;

    async fn get_model(&self) -> Result<String, Error>;

    async fn auth(&mut self, username: &str, password: &str) -> Result<(), Error>;

    async fn reboot(&mut self) -> Result<(), Error>;

    async fn get_hashrate(&self) -> Result<f64, Error>;

    async fn get_nameplate_rate(&self) -> Result<f64, Error>;

    async fn get_temperature(&self) -> Result<f64, Error>;

    async fn get_fan_speed(&self) -> Result<Vec<u32>, Error>;

    async fn get_pools(&self) -> Result<Vec<Pool>, Error>;

    async fn set_pools(&mut self, pools: Vec<Pool>) -> Result<(), Error>;

    async fn set_sleep(&mut self, sleep: bool) -> Result<(), Error>;

    async fn set_blink(&mut self, blink: bool) -> Result<(), Error>;

    async fn get_logs(&mut self) -> Result<Vec<String>, Error>;

    async fn get_mac(&self) -> Result<String, Error>;
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
    fn new(client: Client, ip: String, port: u16) -> Self
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

    async fn get_nameplate_rate(&self) -> Result<f64, Error> {
        self.miner.get_nameplate_rate().await
    }

    async fn get_temperature(&self) -> Result<f64, Error> {
        self.miner.get_temperature().await
    }

    async fn get_fan_speed(&self) -> Result<Vec<u32>, Error> {
        self.miner.get_fan_speed().await
    }

    async fn get_pools(&self) -> Result<Vec<Pool>, Error> {
        self.miner.get_pools().await
    }

    async fn set_pools(&mut self, pools: Vec<Pool>) -> Result<(), Error> {
        self.miner.set_pools(pools).await
    }

    async fn set_sleep(&mut self, sleep: bool) -> Result<(), Error> {
        self.miner.set_sleep(sleep).await
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
}