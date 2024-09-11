use async_trait::async_trait;
use serde_json::json;
use lazy_regex::regex;
use phf::phf_map;
use tokio::sync::{Mutex, MutexGuard};

use crate::miner::{Miner, Pool, Profile, MinerError};
use crate::miners::avalon::cgminer;
use crate::error::Error;
use crate::Client;

static EFF_MAP: phf::Map<&'static str, f64> = phf_map!{
    "A1026" => 67.0,
    "A1066" => 63.0,
    "A1047" => 62.5,
    "A1066Pro" => 60.0,
    "A1146" => 57.0,
    "A1126Pro" => 53.66,
    "A1146Pro" => 52.0,
    "A1166" => 47.0,
    "A1166Pro" => 45.33,
    "A1246" => 38.0,
    "A1266" => 35.0,
    "A1346" => 30.0,
    "A1346C" => 30.0,
    "A1366" => 25.0,
};

pub struct Avalon {
    ip: String,
    port: u16,
    username: String,
    password: String,
    client: Client,

    model: Mutex<Option<String>>,
    version: Mutex<Option<cgminer::VersionResp>>,
    estats: Mutex<Option<cgminer::EStats>>,
}

impl Avalon {
    async fn get_version(&self) -> Result<MutexGuard<Option<cgminer::VersionResp>>, Error> {
        let mut version = self.version.lock().await;
        if version.is_none() {
            let resp = self.client.send_recv(&self.ip, self.port, r#"{"command":"version"}"#).await?;
            let version_resp: cgminer::VersionResp = serde_json::from_str(&resp)?;
            *version = Some(version_resp);
        }
        Ok(version)
    }

    async fn get_estats(&self) -> Result<MutexGuard<Option<cgminer::EStats>>, Error> {
        let mut estats = self.estats.lock().await;
        if estats.is_none() {
            let resp = self.client.send_recv(&self.ip, self.port, r#"{"command":"estats"}"#).await?;
            let estats_resp: cgminer::StatsResp = serde_json::from_str(&resp)?;
            let estats_resp = cgminer::EStats::try_from(&estats_resp)?;
            *estats = Some(estats_resp);
        }
        Ok(estats)
    }

    async fn invalidate(&self) {
        let _ = self.estats.lock().await.take();
    }
}

#[async_trait]
impl Miner for Avalon {
    fn new(client: Client, ip: String, port: u16) -> Self {
        Avalon {
            ip,
            port,
            username: "".to_string(),
            password: "".to_string(),
            client,
            model: Mutex::new(None),
            version: Mutex::new(None),
            estats: Mutex::new(None),
        }
    }

    fn get_type(&self) -> &'static str {
        "Avalon"
    }

    async fn get_model(&self) -> Result<String, Error> {
        let mut model = self.model.lock().await;
        if model.is_none() {
            let version = self.get_version().await?;
            let version = version.as_ref().unwrap_or_else(|| unreachable!());
            if let Some(version) = &version.version {
                if let Some(version) = version.get(0) {
                    *model = Some(format!("A{}", version.model()?));
                } else {
                    return Err(Error::ApiCallFailed("version".to_string()));
                }
            } else {
                return Err(Error::ApiCallFailed("version".to_string()));
            }
        }
        Ok(model.as_ref().unwrap_or_else(|| unreachable!()).clone())
    }

    async fn auth(&mut self, username: &str, password: &str) -> Result<(), Error> {
        self.username = username.to_string();
        self.password = password.to_string();
        Ok(())
    }

    async fn reboot(&mut self) -> Result<(), Error> {
        let cmd = json!({
            "command": "ascset",
            "parameter": "0,reboot,0"
        });
        self.client.send(&self.ip, self.port, &cmd).await
    }

    async fn get_hashrate(&self) -> Result<f64, Error> {
        let estats = self.get_estats().await?;
        let estats = estats.as_ref().unwrap_or_else(|| unreachable!());
        Ok(estats.ghs_mm / 1000.0)
    }

    async fn get_power(&self) -> Result<f64, Error> {
        let cmd = r#"{"command":"ascset","parameter":"0,hashpower"}"#;
        let resp = self.client.send_recv(&self.ip, self.port, &cmd).await?;
        let psinfo = cgminer::PowerSupplyInfo::try_from(serde_json::from_str::<cgminer::StatusResp>(&resp)?)?;
        Ok(psinfo.power as f64)
    }

    async fn get_nameplate_power(&self) -> Result<f64, Error> {
        let nameplate_rate = self.get_nameplate_rate().await?;
        let model = self.get_model().await?;
        let eff = EFF_MAP.get(model.as_str()).ok_or(Error::UnknownModel(model.to_string())).map(|x| *x)?;
        Ok(nameplate_rate * eff)
    }

    async fn get_efficiency(&self) -> Result<f64, Error> {
        if let Ok(estats) = self.get_estats().await {
            let estats = estats.as_ref().unwrap_or_else(|| unreachable!());
            if estats.ghs_mm > 0.0 {
                return Ok(estats.ps.power as f64 / (estats.ghs_mm / 1000.0));
            }
        }
        // If we're not hashing return the dataspec efficiency
        let model = self.get_model().await?;
        EFF_MAP.get(model.as_str()).ok_or(Error::UnknownModel(model.to_string())).map(|x| *x)
    }

    async fn get_nameplate_rate(&self) -> Result<f64, Error> {
        let version = self.get_version().await?;
        let version = version.as_ref().unwrap_or_else(|| unreachable!());
        if let Some(version) = &version.version {
            if let Some(version) = version.get(0) {
                Ok(version.hashrate_th()?)
            } else {
                Err(Error::ApiCallFailed("version".to_string()))
            }
        } else {
            Err(Error::ApiCallFailed("version".to_string()))
        }
    }

    async fn get_temperature(&self) -> Result<f64, Error> {
        let estats = self.get_estats().await?;
        let estats = estats.as_ref().unwrap_or_else(|| unreachable!());
        Ok(estats.tmax as f64)
    }

    async fn get_fan_speed(&self) -> Result<Vec<u32>, Error> {
        let estats = self.get_estats().await?;
        let estats = estats.as_ref().unwrap_or_else(|| unreachable!());
        match &estats.fan3 {
            Some(_) => {
                Ok(vec![
                    estats.fan1,
                    estats.fan2,
                    estats.fan3.unwrap_or_else(|| unreachable!()),
                    estats.fan4.unwrap_or_else(|| unreachable!()),
                ])
            },
            None => {
                Ok(vec![
                    estats.fan1,
                    estats.fan2,
                ])
            }
        }
    }

    async fn get_fan_pwm(&self) -> Result<f64, Error> {
        let estats = self.get_estats().await?;
        let estats = estats.as_ref().unwrap_or_else(|| unreachable!());
        Ok(estats.fanr)
    }

    async fn get_pools(&self) -> Result<Vec<Pool>, Error> {
        // Returns a JS callback, we care about the JSON object inside of CGConfCallback()
        let resp = self.client.send_recv(&self.ip, self.port, r#"{"command":"pools"}"#).await?;
        Ok(
            serde_json::from_str::<cgminer::PoolResp>(&resp)?
                .pools
                .into_iter()
                .map(|p| p.into())
                .collect()
        )
    }

    async fn set_pools(&mut self, pools: Vec<Pool>) -> Result<(), Error> {
        for (i, pool) in pools.iter().enumerate() {
            let cmd = format!(r#"ascset|0,setpool,{},{},{},{},{},{}"#, self.username, self.password, i, pool.url, pool.username, pool.password.as_ref().map(|s| s.as_str()).unwrap_or(""));
            let resp = self.client.send_recv(&self.ip, 4028, &cmd).await?;
            if !resp.to_lowercase().contains("success") {
                return Err(Error::ApiCallFailed(resp));
            }
        }
        Ok(())
    }

    async fn get_sleep(&self) -> Result<bool, Error> {
        let estats = self.get_estats().await?;
        let estats = estats.as_ref().unwrap_or_else(|| unreachable!());
        Ok(estats.ps.power == 0 && estats.sys_status.work == "In Idle")
    }

    async fn set_sleep(&mut self, sleep: bool) -> Result<(), Error> {
        if sleep {
            let cmd = cgminer::PowerSupplyInfo::set_cmd(sleep).to_string();
            let s = self.client.send_recv(&self.ip, self.port, &cmd).await?;
            let status: cgminer::StatusResp = serde_json::from_str(&s)?;
            if status.status[0].status == cgminer::StatusCode::INFO {
                self.invalidate().await;
                Ok(())
            } else {
                Err(Error::ApiCallFailed(status.status[0].msg.clone()))
            }
        } else {
            // If we're waking up, we need to reboot the miner
            self.invalidate().await;
            self.reboot().await
        }
    }

    async fn get_blink(&self) -> Result<bool, Error> {
        let cmd = r#"{"command":"ascset","parameter":"0,led,1-255"}"#;
        let resp = self.client.send_recv(&self.ip, self.port, &cmd).await?;
        let status = serde_json::from_str::<cgminer::StatusResp>(&resp)?;
        if status.status[0].status == cgminer::StatusCode::INFO {
            let re = regex!(r#"LED\[(\d)\]"#);
            let caps = re.captures(&status.status[0].msg).ok_or(Error::InvalidResponse)?;
            let led = caps.get(1).ok_or(Error::InvalidResponse)?.as_str().parse::<u8>().map_err(|_| Error::InvalidResponse)?;
            Ok(led > 0)
        } else {
            Err(Error::ApiCallFailed(status.status[0].msg.clone()))
        }
    }

    async fn set_blink(&mut self, blink: bool) -> Result<(), Error> {
        let cmd = match blink {
            true => r#"{"command":"ascset","parameter":"0,led,1"}"#,
            false => r#"{"command":"ascset","parameter":"0,led,0"}"#,
        };
        let resp = self.client.send_recv(&self.ip, self.port, &cmd).await?;
        let status = serde_json::from_str::<cgminer::StatusResp>(&resp)?;
        if status.status[0].status == cgminer::StatusCode::SUCC {
            Ok(())
        } else {
            Err(Error::ApiCallFailed(status.status[0].msg.clone()))
        }
    }

    async fn get_logs(&mut self) -> Result<Vec<String>, Error> {
        Err(Error::NotSupported)
    }

    async fn get_mac(&self) -> Result<String, Error> {
        let version = self.get_version().await?;
        let version = version.as_ref().unwrap_or_else(|| unreachable!());
        if let Some(version) = &version.version {
            if let Some(version) = version.get(0) {
                Ok(version.mac_addr())
            } else {
                Err(Error::ApiCallFailed("version".to_string()))
            }
        } else {
            Err(Error::ApiCallFailed("version".to_string()))
        }
    }

    async fn get_errors(&mut self) -> Result<Vec<MinerError>, Error> {
        Err(Error::NotSupported)
    }

    async fn get_dns(&self) -> Result<String, Error> {
        Err(Error::NotSupported)
    }

    async fn get_profile(&self) -> Result<Profile, Error> {
        let estats = self.get_estats().await?;
        let estats = estats.as_ref().unwrap_or_else(|| unreachable!());

        match estats.workmode {
            2 | 0 => Ok(Profile::LowPower),
            1 => Ok(Profile::Default),
            _ => Err(Error::InvalidResponse),
        }
    }

    async fn get_profiles(&self) -> Result<Vec<Profile>, Error> {
        Ok(vec![
            Profile::Default,
            Profile::LowPower,
        ])
    }

    async fn set_profile(&mut self, profile: Profile) -> Result<(), Error> {
        // If success response is "ASC 0 set info: WORKMODE[1]"
        let model = self.get_model().await?;
        let re = regex!(r#"msg=asc 0 set info: workmode\[(\d)\]"#);
        let workmode = match (model.as_str(), profile) {
            ("A1466", Profile::LowPower) => 2,
            (_, Profile::Default) => 1,
            (_, Profile::LowPower) => 0,
            _ => return Err(Error::NotSupported),
        };
        let cmd = format!(r#"ascset|0,workmode,{}"#, workmode);
        let resp = self.client.send_recv(&self.ip, self.port, &cmd).await?.to_lowercase();
        let caps = re.captures(&resp).ok_or(Error::InvalidResponse)?;
        let resp = caps.get(1).ok_or(Error::InvalidResponse)?.as_str().parse::<u8>().map_err(|_| Error::InvalidResponse)?;
        if resp == workmode {
            self.reboot().await
        } else {
            Err(Error::ApiCallFailed(resp.to_string()))
        }
    }

    async fn get_hashboard(&mut self) -> Result<String, Error> {
        Err(Error::NotSupported)
    }

    async fn get_hashboards(&self) -> Result<usize, Error> {
        let stats = self.get_estats().await?;
        let stats = stats.as_ref().unwrap_or_else(|| unreachable!());
        Ok(stats.sys_status.nboards as usize)
    }
}
