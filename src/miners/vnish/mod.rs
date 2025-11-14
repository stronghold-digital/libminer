use async_trait::async_trait;
use lazy_regex::regex;
use serde_json::json;
use crate::{Client, Miner, error::Error, Pool, miner::Profile};
use tokio::sync::{Mutex, MutexGuard};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

mod api;
mod error;

use error::VNISH_ERRORS;

use crate::miners::antminer::POWER_MAP;
use crate::miner::MinerError;

pub struct Vnish {
    ip: String,
    _port: u16,
    client: Client,
    token: String,

    status: Mutex<Option<api::MinerStatus>>,
    settings: Mutex<Option<api::Settings>>,
    info: Mutex<Option<api::Info>>,
    summary: Mutex<Option<api::Summary>>,
    presets: Mutex<Option<Vec<Profile>>>,
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
            _port: port,
            client,

            token: String::new(),
            status: Mutex::new(None),
            settings: Mutex::new(None),
            info: Mutex::new(None),
            summary: Mutex::new(None),
            presets: Mutex::new(None),
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
        match &summary.miner {
            Some(miner) => Ok(miner.instant_hashrate),
            None => Ok(0.0)
        }
    }

    async fn get_power(&self) -> Result<f64, Error> {
        let summary = self.get_summary().await?;
        let summary = summary.as_ref().unwrap_or_else(|| unreachable!());
        match &summary.miner {
            Some(miner) =>
                if miner.power_usage < 1.0 && miner.average_hashrate > 0.1 {
                    Ok(miner.average_hashrate * 36.0)
                } else {
                    Ok(miner.power_usage)
                },
            None => Ok(0.0)
        }
    }

    async fn get_nameplate_power(&self) -> Result<f64, Error> {
        let profile = self.get_profile().await?;

        match profile {
            Profile::Preset { name: _, power, ths: _ } => {
                Ok(power)
            }
            _ => {
                let model = self.get_model().await?;
                // Map s19-88 to s19
                let model = model.split('-').next().unwrap_or_else(|| unreachable!());
                let eff = POWER_MAP.get(model).ok_or(Error::ApiCallFailed("Invalid model".into()))?;
                Ok(eff.0 * 92.0)
            },
        }
    }

    async fn get_efficiency(&self) -> Result<f64, Error> {
        let summary = self.get_summary().await?;
        let summary = summary.as_ref().unwrap_or_else(|| unreachable!());
        match &summary.miner {
            Some(miner) => Ok(miner.power_efficiency),
            None => Ok(POWER_MAP.get(&self.get_model().await?).map(|e| e.0).unwrap_or(0.0))
        }
    }

    async fn get_nameplate_rate(&self) -> Result<f64, Error> {
        let summary = self.get_summary().await?;
        let summary = summary.as_ref().unwrap_or_else(|| unreachable!());
        // Convert from GH/s to TH/s
        match &summary.miner {
            Some(miner) => Ok(miner.chains.iter().map(|c| c.hashrate_ideal).sum::<f64>() / 1000.0),
            None => Ok(0.0)
        }
    }

    async fn get_temperature(&self) -> Result<f64, Error> {
        let summary = self.get_summary().await?;
        let summary = summary.as_ref().unwrap_or_else(|| unreachable!());
        match &summary.miner {
            Some(miner) => Ok(miner.chip_temp.max as f64),
            None => Ok(0.0)
        }
    }

    async fn get_fan_speed(&self) -> Result<Vec<u32>, Error> {
        let summary = self.get_summary().await?;
        let summary = summary.as_ref().unwrap_or_else(|| unreachable!());
        match &summary.miner {
            Some(miner) => Ok(miner.cooling.fans.iter().map(|f| f.rpm).collect()),
            None => Ok(vec![])
        }
    }

    async fn get_fan_pwm(&self) -> Result<f64, Error> {
        let summary = self.get_summary().await?;
        let summary = summary.as_ref().unwrap_or_else(|| unreachable!());
        match &summary.miner {
            Some(miner) => Ok(miner.cooling.fan_duty as f64),
            None => Ok(0.0)
        }
    }

    async fn get_pools(&self) -> Result<Vec<Pool>, Error> {
        let settings = self.get_settings().await?;
        let settings = settings.as_ref().unwrap_or_else(|| unreachable!());
        Ok(settings.miner.pools.clone())
    }

    async fn set_pools(&mut self, pools: Vec<Pool>) -> Result<(), Error> {
        let js = json!({
            "miner": {
                "pools": pools.iter().enumerate().map(|(i, p)| api::VPool {
                    url: &p.url,
                    user: &p.username,
                    pass: p.password.as_ref().map(|s| s.as_str()).unwrap_or(""),
                    order: i,
                }).collect::<Vec<_>>(),
            }
        });

        let resp = self.client.http_client
            .post(&format!("http://{}/api/v1/settings", self.ip))
            .bearer_auth(&self.token)
            .json(&js)
            .send()
            .await?;

        if resp.status().is_success() {
            self.invalidate().await?;
            Ok(())
        } else {
            Err(Error::ApiCallFailed("settings".into()))
        }
    }

    async fn get_sleep(&self) -> Result<bool, Error> {
        let summary = self.get_summary().await?;
        let summary = summary.as_ref().unwrap_or_else(|| unreachable!());
        match &summary.miner {
            Some(miner) => Ok(miner.miner_status.miner_state == api::StatusCode::Stopped),
            None => Ok(false)
        }
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
                match &summary.miner {
                    Some(miner) => {
                        if (miner.chip_temp.max - miner.chip_temp.min) < 5.0 ||
                            (miner.miner_status.miner_state == api::StatusCode::Stopped && miner.miner_status.miner_state_time >= 120) {
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
                    None => {
                        Err(Error::ApiCallFailed("mining/start not ready".into()))
                    }
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
            .post(&format!("http://{}/api/v1/find-miner", self.ip))
            .bearer_auth(&self.token)
            .json(&serde_json::json!({"on": blink}))
            .send()
            .await?;
        
        if resp.status().is_success() {
            Ok(())
        } else {
            Err(Error::ApiCallFailed("find_miner failed".into()))
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

    async fn get_errors(&mut self) -> Result<Vec<MinerError>, Error> {
        let logs = self.get_logs().await?.join("\n");
        // Only search since the last time we started up
        let re = regex!(r"INFO: Initializing PSU");
        let start = re.find_iter(&logs).last().map(|m| m.start()).unwrap_or(0);
        let logs = &logs[start..];

        let mut errors = HashSet::new();
        for err in VNISH_ERRORS.iter() {
            let mut logs = logs;
            while let Some(msg) = err.get_err(&logs) {
                let end = err.re.find(&logs).unwrap().end();
                logs = &logs[end..];
                errors.insert(msg);
            }
        }
        Ok(errors.into_iter().collect())
    }

    async fn get_dns(&self) -> Result<String, Error> {
        let info = self.get_info().await?;
        let info = info.as_ref().unwrap_or_else(|| unreachable!());
        Ok(info.system.network_status.dns.get(0).ok_or(Error::ApiCallFailed("No DNS servers found".into()))?.clone())
    }

    async fn get_profile(&self) -> Result<Profile, Error> {
        let presets = self.get_profiles().await?;
        let settings = self.get_settings().await?;
        let settings = settings.as_ref().unwrap_or_else(|| unreachable!());
        Ok(
            presets.iter().find(|p| {
                match p {
                    Profile::Default => {
                        settings.miner.overclock.preset == "disabled"
                        && settings.miner.overclock.globals.volt == settings.ui.consts.overclock.default_voltage
                        && settings.miner.overclock.globals.freq == settings.ui.consts.overclock.default_freq
                    },
                    Profile::Manual { .. }=> {
                        settings.miner.overclock.preset == "disabled"
                        && (settings.miner.overclock.globals.volt != settings.ui.consts.overclock.default_voltage
                        || settings.miner.overclock.globals.freq != settings.ui.consts.overclock.default_freq)
                    },
                    Profile::Preset { name, .. } => name == &settings.miner.overclock.preset,
                    Profile::LowPower => false,
                }
            }).unwrap_or_else(|| {
                tracing::error!("Invalid profile: {:?}", settings.miner.overclock.preset);
                tracing::error!("Settings: {:?}", settings.miner.overclock);
                tracing::error!("Constants: {:?}", settings.ui.consts.overclock);
                tracing::error!("Profiles: {:?}", presets);
                unreachable!()
            }).clone()
        )
    }

    async fn get_profiles(&self) -> Result<Vec<Profile>, Error> {
        let mut profiles = self.presets.lock().await;
        if profiles.is_none() {
            let resp = self.client.http_client
                .get(&format!("http://{}/api/v1/autotune/presets", self.ip))
                .bearer_auth(&self.token)
                .send()
                .await?;

            if !resp.status().is_success() {
                return Err(Error::ApiCallFailed("presets".into()));
            }
            let presets = resp.json::<api::Presets>().await?;

            let settings = self.get_settings().await?;
            let settings = settings.as_ref().unwrap_or_else(|| unreachable!());

            let mut presets: Vec<_> = presets.into_iter().map(|p| p.into()).collect();
            presets.push(Profile::Manual {
                volt: settings.miner.overclock.globals.volt,
                freq: settings.miner.overclock.globals.freq,
                min_freq: settings.ui.consts.overclock.min_freq,
                max_freq: settings.ui.consts.overclock.max_freq,
                min_volt: settings.ui.consts.overclock.min_voltage,
                max_volt: settings.ui.consts.overclock.max_voltage_stock_psu,
                def_volt: settings.ui.consts.overclock.default_voltage,
                def_freq: settings.ui.consts.overclock.default_freq,
            });
            presets.push(Profile::Default);
            *profiles = Some(presets);
        }
        Ok(profiles.as_ref().unwrap().clone())
    }

    async fn set_profile(&mut self, profile: Profile) -> Result<(), Error> {
        let presets = self.get_profiles().await?;
        let preset = presets.iter().find(|p| match (*p, &profile) {
            (Profile::Default, Profile::Default) => true,
            (Profile::Preset { name: n1, .. }, Profile::Preset { name: n2, .. }) => n1 == n2,
            (Profile::Manual { volt: _, freq: _, max_freq, min_freq, min_volt, max_volt, .. }, Profile::Manual { volt: v2, freq: f2, .. }) => {
                v2 >= min_volt && v2 <= max_volt && f2 >= min_freq && f2 <= max_freq
            },
            _ => false,
        })
            .ok_or(Error::ApiCallFailed("Invalid profile".into()))?;

        let js = {
            let settings = self.get_settings().await?;
            let settings = settings.as_ref().unwrap_or_else(|| unreachable!());

            let _chains = settings.miner.overclock.chains.iter().cloned()
                .map(|mut c| {
                    // Set to global freq
                    c.freq = 0;
                    // Set to chain freq
                    c.chips = vec![0; c.chips.len()];
                    c
                });
    
            match preset {
                Profile::Default => json!({
                    "miner": {
                        "overclock": {
                            "preset": "disabled",
                            "globals": {
                                "volt": settings.ui.consts.overclock.default_voltage,
                                "freq": settings.ui.consts.overclock.default_freq,
                            },
                        },
                    },
                }),
                Profile::Preset { name, .. } => json!({
                    "miner": {
                        "overclock": {
                            "preset": name,
                        },
                    },
                }),
                Profile::Manual { volt, freq, .. } => json!({
                    "miner": {
                        "overclock": {
                            "preset": "disabled",
                            "globals": {
                                "volt": volt,
                                "freq": freq,
                            },
                        },
                    },
                }),
                Profile::LowPower => return Err(Error::NotSupported),
            }
        };
    
        let resp = self.client.http_client
            .post(&format!("http://{}/api/v1/settings", self.ip))
            .bearer_auth(&self.token)
            .json(&js)
            .send()
            .await?;

        if resp.status().is_success() {
            // println!("{:?}", resp.text().await?);
            self.reboot().await?;
            self.invalidate().await?;
            Ok(())
        } else {
            Err(Error::ApiCallFailed("settings".into()))
        }
    }

    async fn get_hashboard(&mut self) -> Result<String, Error> {
        Err(Error::NotSupported)
    }

    async fn get_hashboards(&self) -> Result<usize, Error> {
        let summary = self.get_summary().await?;
        let summary = summary.as_ref().unwrap_or_else(|| unreachable!());
        match &summary.miner {
            Some(miner) => Ok(miner.chains.iter().filter(|c| c.status.state != api::ChainState::Failure).count()),
            None => Ok(0)
        }
    }
}