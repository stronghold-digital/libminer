use async_trait::async_trait;
use crate::{Client, Miner, error::Error, Pool};
use tokio::sync::{Mutex, MutexGuard};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

mod api;
mod error;

use error::VNISH_ERRORS;

pub struct Vnish {
    ip: String,
    port: u16,
    client: Client,
    token: String,

    status: Mutex<Option<api::MinerStatus>>,
    settings: Mutex<Option<api::Settings>>,
    info: Mutex<Option<api::Info>>,
    summary: Mutex<Option<api::Summary>>,
}

impl Vnish {
    async fn get_status(&self) -> Result<MutexGuard<'_, Option<api::MinerStatus>>, Error> {
        let mut status = self.status.lock().await;

        if status.is_none() {
            *status = Some(
                self.client.http_client
                    .get(&format!("http://{}/api/v1/status", self.ip))
                    .bearer_auth(&self.token)
                    .send()
                    .await?
                    .json::<api::MinerStatus>()
                    .await?
            );

        }

        Ok(status)
    }

    async fn get_settings(&self) -> Result<MutexGuard<'_, Option<api::Settings>>, Error> {
        let mut settings = self.settings.lock().await;

        if settings.is_none() {
            *settings = Some(
                self.client.http_client
                    .get(&format!("http://{}/api/v1/settings", self.ip))
                    .bearer_auth(&self.token)
                    .send()
                    .await?
                    .json::<api::Settings>()
                    .await?
            );

        }

        Ok(settings)
    }

    async fn get_info(&self) -> Result<MutexGuard<'_, Option<api::Info>>, Error> {
        let mut info = self.info.lock().await;

        if info.is_none() {
            *info = Some(
                self.client.http_client
                    .get(&format!("http://{}/api/v1/info", self.ip))
                    .bearer_auth(&self.token)
                    .send()
                    .await?
                    .json::<api::Info>()
                    .await?
            );

        }

        Ok(info)
    }

    async fn get_summary(&self) -> Result<MutexGuard<'_, Option<api::Summary>>, Error> {
        let mut summary = self.summary.lock().await;

        if summary.is_none() {
            *summary = Some(
                self.client.http_client
                    .get(&format!("http://{}/api/v1/summary", self.ip))
                    .bearer_auth(&self.token)
                    .send()
                    .await?
                    .json::<api::Summary>()
                    .await?
            );

        }

        Ok(summary)
    }

    async fn invalidate(&self) -> Result<(), Error> {
        *self.status.lock().await = None;
        *self.settings.lock().await = None;
        *self.info.lock().await = None;
        *self.summary.lock().await = None;

        Ok(())
    }
}

#[async_trait]
impl Miner for Vnish {
    fn new(client: Client, ip: String, port: u16) -> Self {
        Self {
            ip,
            port,
            client,

            token: String::new(),
            status: Mutex::new(None),
            settings: Mutex::new(None),
            info: Mutex::new(None),
            summary: Mutex::new(None),
        }
    }

    fn get_type(&self) -> &'static str {
        "Vnish"
    }

    async fn get_model(&self) -> Result<String, Error> {
        let info = self.get_info().await?;
        let info = info.as_ref().unwrap_or_else(|| unreachable!());
        Ok(info.model.clone())
    }

    async fn auth(&mut self, _username: &str, password: &str) -> Result<(), Error> {
        #[derive(Deserialize)]
        struct TokenResp {
            pub token: String,
        }

        #[derive(Serialize)]
        struct UnlockReq<'a> {
            pub pw: &'a str,
        }

        let resp = self.client.http_client
            .post(&format!("http://{}/api/v1/unlock", self.ip))
            .json(&UnlockReq {
                pw: password,
            })
            .send()
            .await?;

        match resp.status() {
            reqwest::StatusCode::OK => {},
            reqwest::StatusCode::FORBIDDEN => return Err(Error::Unauthorized),
            _ => return Err(Error::ApiCallFailed(format!("auth/unlock {:?}", resp.status()))),
        }

        self.token = resp.json::<TokenResp>().await?.token;
        Ok(())
    }

    async fn reboot(&mut self) -> Result<(), Error> {
        let resp = self.client.http_client
            .post(&format!("http://{}/api/v1/mining/restart", self.ip))
            .bearer_auth(&self.token)
            .send()
            .await?;

        self.invalidate().await?;

        match resp.status() {
            reqwest::StatusCode::OK => Ok(()),
            reqwest::StatusCode::UNAUTHORIZED => Err(Error::Unauthorized),
            _ => Err(Error::ApiCallFailed("mining/restart".into())),
        }
    }

    async fn get_hashrate(&self) -> Result<f64, Error> {
        let summary = self.get_summary().await?;
        let summary = summary.as_ref().unwrap_or_else(|| unreachable!());
        Ok(summary.miner.instant_hashrate)
    }

    async fn get_power(&self) -> Result<f64, Error> {
        let summary = self.get_summary().await?;
        let summary = summary.as_ref().unwrap_or_else(|| unreachable!());
        Ok(summary.miner.power_usage)
    }

    async fn get_efficiency(&self) -> Result<f64, Error> {
        let summary = self.get_summary().await?;
        let summary = summary.as_ref().unwrap_or_else(|| unreachable!());
        Ok(summary.miner.power_efficiency as f64)
    }

    async fn get_nameplate_rate(&self) -> Result<f64, Error> {
        let summary = self.get_summary().await?;
        let summary = summary.as_ref().unwrap_or_else(|| unreachable!());
        Ok(summary.miner.chains.iter().map(|c| c.hashrate_ideal).sum())
    }

    async fn get_temperature(&self) -> Result<f64, Error> {
        let summary = self.get_summary().await?;
        let summary = summary.as_ref().unwrap_or_else(|| unreachable!());
        Ok(summary.miner.chip_temp.max as f64)
    }

    async fn get_fan_speed(&self) -> Result<Vec<u32>, Error> {
        let summary = self.get_summary().await?;
        let summary = summary.as_ref().unwrap_or_else(|| unreachable!());
        Ok(summary.miner.cooling.fans.iter().map(|f| f.rpm).collect())
    }

    async fn get_fan_pwm(&self) -> Result<f64, Error> {
        let summary = self.get_summary().await?;
        let summary = summary.as_ref().unwrap_or_else(|| unreachable!());
        Ok(summary.miner.cooling.fan_duty as f64)
    }

    async fn get_pools(&self) -> Result<Vec<Pool>, Error> {
        let settings = self.get_settings().await?;
        let settings = settings.as_ref().unwrap_or_else(|| unreachable!());
        Ok(settings.miner.pools.clone())
    }

    async fn set_pools(&mut self, pools: Vec<Pool>) -> Result<(), Error> {
        Err(Error::NotSupported)
    }

    async fn get_sleep(&self) -> Result<bool, Error> {
        let summary = self.get_summary().await?;
        let summary = summary.as_ref().unwrap_or_else(|| unreachable!());
        Ok(summary.miner.miner_status.miner_state == api::StatusCode::Stopped || summary.miner.miner_status.miner_state == api::StatusCode::ShuttingDown)
    }

    async fn set_sleep(&mut self, sleep: bool) -> Result<(), Error> {
        if self.get_sleep().await? == sleep {
            return Ok(());
        }

        let r = match sleep {
            false => {
                // We need to make sure the miner is ready to start mining
                // In this case 1 of 2 situations must be true:
                // chip_temp.max - chip_temp.min < 5
                // (status = Stopped && miner_state_time >= 120)
                let summary = self.get_summary().await?;
                let summary = summary.as_ref().unwrap_or_else(|| unreachable!());
                if (summary.miner.chip_temp.max - summary.miner.chip_temp.min) < 5 ||
                    (summary.miner.miner_status.miner_state == api::StatusCode::Stopped && summary.miner.miner_status.miner_state_time >= 120) {
                        let resp = self.client.http_client
                            .post(&format!("http://{}/api/v1/mining/start", self.ip))
                            .bearer_auth(&self.token)
                            .send()
                            .await?;
                        return if resp.status().is_success() {
                            Ok(())
                        } else {
                            Err(Error::ApiCallFailed("mining/start failed".into()))
                        }
                    } else {
                        Err(Error::ApiCallFailed("mining/start not ready".into()))
                    }
            }
            true => {
                let resp = self.client.http_client
                    .post(&format!("http://{}/api/v1/mining/stop", self.ip))
                    .bearer_auth(&self.token)
                    .send()
                    .await?;
                return if resp.status().is_success() {
                    Ok(())
                } else {
                    Err(Error::ApiCallFailed("mining/stop failed".into()))
                }
            }
        };
        let _ = self.invalidate().await;
        r
    }

    async fn get_blink(&self) -> Result<bool, Error> {
        let status: MutexGuard<Option<api::MinerStatus>> = self.get_status().await?;
        let status = status.as_ref().unwrap_or_else(|| unreachable!());
        Ok(status.find_miner)
    }

    async fn set_blink(&mut self, blink: bool) -> Result<(), Error> {
        let status: MutexGuard<Option<api::MinerStatus>> = self.get_status().await?;
        let status = status.as_ref().unwrap_or_else(|| unreachable!());
        if status.find_miner == blink {
            return Ok(());
        }

        let resp = self.client.http_client
            .post(&format!("http://{}/api/v1/mining/find_miner", self.ip))
            .bearer_auth(&self.token)
            .json(&serde_json::json!({"on": blink}))
            .send()
            .await?;
        
        if resp.status().is_success() {
            Ok(())
        } else {
            Err(Error::ApiCallFailed("mining/find_miner failed".into()))
        }
    }

    async fn get_logs(&mut self) -> Result<Vec<String>, Error> {
        let resp = self.client.http_client
            .get(&format!("http://{}/api/v1/logs/miner", self.ip))
            .bearer_auth(&self.token)
            .send()
            .await?;

        if resp.status().is_success() {
            let logs = resp.text().await?;
            Ok(logs.lines().map(|l| l.to_string()).collect())
        } else {
            Err(Error::ApiCallFailed("logs failed".into()))
        }
    }

    async fn get_mac(&self) -> Result<String, Error> {
        let info = self.get_info().await?;
        let info = info.as_ref().unwrap_or_else(|| unreachable!());
        Ok(info.system.network_status.mac.clone())
    }

    async fn get_errors(&mut self) -> Result<Vec<String>, Error> {
        let resp = self.client.http_client
            .get(&format!("http://{}/api/v1/logs/miner", self.ip))
            .bearer_auth(&self.token)
            .send()
            .await?;

        if resp.status().is_success() {
            let logs = resp.text().await?;
            let mut errors = HashSet::new();
            for err in VNISH_ERRORS.iter() {
                if let Some(msg) = err.get_msg(&logs) {
                    errors.insert(msg);
                }
            }
            Ok(errors.into_iter().collect())
        } else {
            Err(Error::ApiCallFailed("logs failed".into()))
        }
    }

    async fn get_dns(&self) -> Result<String, Error> {
        let info = self.get_info().await?;
        let info = info.as_ref().unwrap_or_else(|| unreachable!());
        Ok(info.system.network_status.dns.get(0).ok_or(Error::ApiCallFailed("No DNS servers found".into()))?.clone())
    }
}