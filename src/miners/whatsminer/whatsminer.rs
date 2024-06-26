use async_trait::async_trait;
use serde::Deserialize;
use serde_json::json;
use tokio::{net::TcpStream, io::{AsyncWriteExt, AsyncReadExt}, sync::{Mutex, MutexGuard}};
use lazy_regex::regex;
use std::collections::HashSet;
use phf::phf_map;

use crate::{Client, Miner, miner::MinerError, error::Error, Pool, miners::common, miners::whatsminer::wmapi, Cache, CacheItem, miner::Profile};
use super::{error::WHATSMINER_ERRORS, wmapi::StatusCode};

// (J/TH, Datasheet TH)
static EFF_MAP: phf::Map<&'static str, (f64, f64)> = phf_map! {
    "M20S" => (48.0, 68.0),
    "M31S" => (46.0, 72.0),
    "M31S+" => (42.0, 80.0),
    "M30S" => (38.0, 88.0),
    "M30S+" => (34.0, 100.0),
    "M33S+" => (34.0, 210.0),
    "M30S++" => (31.0, 108.0),
    "M33S++" => (31.0, 230.0),
    "M50" => (29.0, 115.0),
    "M53" => (29.0, 235.0),
    "M50S" => (26.0, 125.0),
    "M53S" => (26.0, 235.0),
    "M50S+" => (24.0, 138.0),
};

#[derive(Debug, Deserialize)]
pub struct LogLen {
    pub logfilelen: String,
}

#[derive(Debug, Deserialize)]
pub struct LogsResponse {
    #[serde(rename = "STATUS")]
    pub status: common::StatusCode,
    #[serde(rename = "When")]
    pub when: usize,
    #[serde(rename = "Code")]
    pub code: usize,
    #[serde(rename = "Msg")]
    pub msg: Option<LogLen>,
    #[serde(rename = "Description")]
    pub description: String,
}

pub struct Whatsminer {
    ip: String,
    port: u16,
    password: Option<String>,
    token: Option<wmapi::WhatsminerToken>,
    client: Client,
    cache: Option<Cache>,

    model: Mutex<Option<String>>,
    summary: Mutex<Option<wmapi::SummaryResp>>,
}

impl Whatsminer {
    async fn send_recv<T>(&self, data: &T) -> Result<String, Error>
        where T: ToString
    {
        let mut resp = self.client.send_recv(&self.ip, self.port, data).await?;
        // Whatsminer can return non-compliant JSON
        resp = resp.replace("inf", "\"inf\"");
        resp = resp.replace("nan", "\"nan\"");
        resp = resp.replace(",}", "}");
        Ok(resp)
    }

    async fn refresh_token(&mut self) -> Result<(), Error> {
        if let Some(passwd) = &self.password {
            let resp = self.send_recv(&json!({"cmd": "get_token"})).await?;
            match serde_json::from_str::<wmapi::TokenResponse>(&resp) {
                Ok(token_resp) => {
                    self.token = Some(
                        token_resp
                            .make_token(passwd)
                            .map_err(|_| Error::ApiCallFailed("Failed to make token".into()))?
                    );
                    if let Some(cache) = &self.cache {
                        let mut cache = cache.write().await;
                        if let Some(token) = &self.token {
                            cache.insert(
                                self.ip.clone(),
                                CacheItem {
                                    token: serde_json::to_string(token)?,
                                    token_expires: token.expires,
                                },
                            );
                        }
                    }
                    Ok(())
                },
                Err(e) => Err(e.into())
            }
        } else {
            Err(Error::Unauthorized)
        }
    }

    async fn token_cached(&mut self) -> Result<(), Error> {
        // If we don't have a token, check the cache
        if self.token.is_none() {
            if let Some(cache) = &self.cache {
                let cache = cache.read().await;
                if let Some(token) = cache.get(&self.ip) {
                    if token.token_expires > chrono::Utc::now() {
                        self.token = serde_json::from_str(&token.token)?;
                        return Ok(());
                    }
                }
            }
        }

        self.refresh_token().await
    }

    async fn send_recv_enc(&mut self, mut data: serde_json::Value) -> Result<String, Error> {
        if let Some(token) = &self.token {
            // Refresh our token if its expired
            if token.is_expired() {
                self.refresh_token().await?;
            }
            // We need to reborrow the token due to the possibility of it being mutated by the refresh
            // Should never panic so unwrap() is fine
            let token = self.token.as_ref().unwrap();
            // Stuff our token into the JSON
            data.as_object_mut().unwrap().insert("token".to_string(), serde_json::Value::String(token.get_token().into()));
            let enc_data = token.encrypt(&data)?;
            let resp = self.send_recv(&enc_data).await?;
            let js = serde_json::from_str(&resp).map_err(|_| Error::ApiCallFailed("Failed to parse JSON".into()))?;
            let dec_data = token.decrypt(&js)?;
            Ok(dec_data.to_string())
        } else {
            Err(Error::Unauthorized)
        }
    }

    async fn get_summary(&self) -> Result<MutexGuard<Option<wmapi::SummaryResp>>, Error> {
        let mut summary = self.summary.lock().await;
        if summary.is_none() {
            let resp = self.send_recv(&json!({"cmd": "summary"})).await?;
            if let Ok(s) = serde_json::from_str::<wmapi::Status>(&resp) {
                println!("Summary API call failed: {}", s.msg);
                return Err(Error::ApiCallFailed(s.msg));
            } else {
                let resp = serde_json::from_str::<wmapi::SummaryResp>(&resp)?;
                *summary = Some(resp);
            }
        }
        Ok(summary)
    }

    async fn invalidate(&self) {
        let _ = self.summary.lock().await.take();
    }
}

#[async_trait]
impl Miner for Whatsminer {
    fn new(client: Client, ip: String, port: u16) -> Self {
        Self {
            ip: ip.clone(),
            port,
            password: None,
            token: None,
            client,
            cache: None,
            summary: Mutex::new(None),
            model: Mutex::new(None),
        }
    }

    fn with_cache(mut self, cache: Option<Cache>) -> Self {
        self.cache = cache;
        self
    }

    fn get_type(&self) -> &'static str {
        "Whatsminer"
    }

    async fn get_model(&self) -> Result<String, Error> {
        let mut model = self.model.lock().await;

        if model.is_none() {
            let resp = self.client.http_client
                .get(format!("https://{}/cgi-bin/luci/admin/status/overview", self.ip))
                .send()
                .await?
                .text()
                .await?;
            let modelre = regex!(r#"<td.+>Model</td>\s*<td>WhatsMiner ([a-zA-Z0-9\+]+)(?:_V.+)?</td>"#);
            *model = Some(modelre.captures(&resp)
                .ok_or(Error::ExpectedReturn)?
                .get(1)
                .ok_or(Error::ExpectedReturn)?
                .as_str()
                .to_string());
        }

        Ok(model.as_ref().unwrap_or_else(|| unreachable!()).clone())
    }

    async fn auth(&mut self, username: &str, password: &str) -> Result<(), Error> {
        self.password = Some(password.to_string());
        let r = self.client.http_client
            .post(format!("https://{}/cgi-bin/luci", self.ip))
            .form(&[("luci_username", username), ("luci_password", password)])
            .send()
            .await?;
        if r.status() != 200 {
            return Err(Error::Unauthorized);
        }
        self.token_cached().await?;
        Ok(())
    }

    async fn reboot(&mut self) -> Result<(), Error> {
        let js = json!({
            "command": "reboot",
        });
        let _ = self.send_recv_enc(js).await?;
        Ok(())
    }

    async fn get_hashrate(&self) -> Result<f64, Error> {
        let sum = self.get_summary().await?;
        let sum = sum.as_ref().unwrap_or_else(|| unreachable!());
        Ok(sum.summary[0].hashrate_ths())
    }

    async fn get_power(&self) -> Result<f64, Error> {
        let sum = self.get_summary().await?;
        let sum = sum.as_ref().unwrap_or_else(|| unreachable!());

        Ok(sum.summary[0].power as f64)
    }

    async fn get_nameplate_power(&self) -> Result<f64, Error> {
        let model = self.get_model().await?;

        EFF_MAP.get(model.as_str()).ok_or(Error::UnknownModel(model.to_string())).map(|(jth, watts)| jth * watts)
    }

    async fn get_efficiency(&self) -> Result<f64, Error> {
        if let Ok(sum) = self.get_summary().await {
            let sum = sum.as_ref().unwrap_or_else(|| unreachable!());
            if sum.summary[0].hashrate_ths() > 0.0 {
                return Ok(sum.summary[0].power as f64 / (sum.summary[0].hashrate_ths()));
            }
        }
        // If we're not hashing return the dataspec efficiency
        let model = self.get_model().await?;
        EFF_MAP.get(model.as_str()).ok_or(Error::UnknownModel(model.to_string())).map(|(x, _)| *x)
    }

    async fn get_nameplate_rate(&self) -> Result<f64, Error> {
        if let Ok(sum) = self.get_summary().await {
            let sum = sum.as_ref().unwrap_or_else(|| unreachable!());
    
            Ok(sum.summary[0].factory_ghs as f64 / 1000.0)
        } else {
            // If we're not hashing return the dataspec efficiency
            // Cause whatsminer .-.
            let model = self.get_model().await?;
            EFF_MAP.get(model.as_str()).ok_or(Error::UnknownModel(model.to_string())).map(|(_, x)| *x)
        }
    }

    async fn get_temperature(&self) -> Result<f64, Error> {
        let sum = self.get_summary().await?;
        let sum = sum.as_ref().unwrap_or_else(|| unreachable!());

        Ok(sum.summary[0].temperature)
    }

    async fn get_fan_speed(&self) -> Result<Vec<u32>, Error> {
        let sum = self.get_summary().await?;
        let sum = sum.as_ref().unwrap_or_else(|| unreachable!());

        Ok(vec![sum.summary[0].fan_speed_in, sum.summary[0].fan_speed_out])
    }

    async fn get_fan_pwm(&self) -> Result<f64, Error> {
        // Whatsminers don't have fan pwm, max fan speed is 7000 RPM
        self.get_fan_speed().await?.iter()
            .max()
            .map(|x| (*x as f64 / 7000.0) * 100.0)
            .ok_or(Error::ApiCallFailed("No fan speed".to_string()))
    }

    async fn get_pools(&self) -> Result<Vec<Pool>, Error> {
        let resp = self.send_recv(&json!({"cmd":"pools"})).await?;
        let pools: common::PoolsResp = serde_json::from_str(&resp)?;
        Ok(pools.pools.iter().map(|p| Pool {
            url: p.url.clone(),
            username: p.user.clone(),
            password: None,
        }).collect())
    }

    async fn set_pools(&mut self, pools: Vec<Pool>) -> Result<(), Error> {
        //TODO: this can panic
        let js = json!({
            "cmd": "update_pools",
            "pool1": pools[0].url,
            "worker1": pools[0].username,
            "passwd1": pools[0].password,
            "pool2": pools[1].url,
            "worker2": pools[1].username,
            "passwd2": pools[1].password,
            "pool3": pools[2].url,
            "worker3": pools[2].username,
            "passwd3": pools[2].password,
        });
        let _ = self.send_recv_enc(js).await?;
        self.invalidate().await;
        Ok(())
    }

    async fn get_sleep(&self) -> Result<bool, Error> {
        // Grrrrrr, cg/btminer isn't always present in the process listing...
        if self.get_hashrate().await? > 0.0 {
            return Ok(false);
        }
        //This doesn't work for miners running cgminer
        let resp = self.send_recv(&json!({"cmd":"status"})).await;
        let sleep_stat = match resp {
            Ok(resp) => match serde_json::from_str::<wmapi::BtStatusResp>(&resp) {
                Ok(s) => {
                    // Implicitly trust v2
                    match s.msg {
                        wmapi::BtStatus::V2(status) => return Ok(status.mineroff),
                        wmapi::BtStatus::V1(status) => status.btmineroff,
                    }
                },
                Err(_) => true,
            }
            Err(_) => true,
        };

        // We know for sure we're awake if btmineroff is false
        if !sleep_stat {
            return Ok(false);
        }
        
        // Double check that cgminer isn't running
        // Scrape the web API yet again
        if let Ok(r) = self.client.http_client
            .get(&format!("https://{}/cgi-bin/luci/admin/status/processes", self.ip))
            .send()
            .await {
                if let Ok(r) = r.text().await {
                    let re = regex!(r#".COMMAND" value="(cg|bt)miner" />"#);
                    return Ok(!re.is_match(&r));
                }
            }
        // If we can't scrape the web API, return btstatus
        return Ok(sleep_stat)
    }

    async fn set_sleep(&mut self, sleep: bool) -> Result<(), Error> {
        let js = match sleep {
            true => json!({
                "cmd": "power_off",
                "respbefore": "true", // Please respond before power off
            }),
            false => json!({
                "cmd": "power_on",
            }),
        };
        let resp = self.send_recv_enc(js).await;
        match (sleep, resp) {
            (true, Err(e)) => {
                // If the error was a timeout assume we're sleeping
                if let Error::Timeout = e {
                    Ok(())
                } else {
                    Err(e)
                }
            },
            (_, Ok(resp)) => {
                let stat = serde_json::from_str::<wmapi::Status>(&resp)?;
                if stat.status == StatusCode::SUCC {
                    self.invalidate().await;
                    Ok(())
                } else {
                    Err(Error::ApiCallFailed(stat.msg))
                }
            },
            (_, Err(e)) => Err(e),
        }
    }

    async fn get_blink(&self) -> Result<bool, Error> {
        let resp = self.send_recv(&json!({"cmd":"get_miner_info"})).await?;
        if let Ok(_) = serde_json::from_str::<wmapi::Status>(&resp) {
            // We could error or assume not hashing
            // Err(Error::ApiCallFailed(status.msg))
            Ok(false)
        } else {
            let resp: wmapi::MinerInfoResponse = serde_json::from_str(&resp)?;
            Ok(resp.msg.ledstat != "auto")
        }
    }

    async fn set_blink(&mut self, blink: bool) -> Result<(), Error> {
        let js = match blink {
            true => json!({
                "command": "set_led",
                "color": "red",
                "period": 1000,
                "duration": 500,
                "start": 0,
            }),
            false => json!({
                "command": "set_led",
                "param": "auto",
            }),
        };
        let _ = self.send_recv_enc(js).await?;
        //println!("{}", resp);
        Ok(())
    }

    async fn get_logs(&mut self) -> Result<Vec<String>, Error> {
        if let Some(token) = &self.token {
            let js = token.encrypt(&json!({
                "command": "download_logs",
                "token": token.get_token(),
            }))?;
            // This responds in 2 parts, the first part is a status response for the command
            // the second part is the logs sent 10ms after the first part.
            let mut stream = TcpStream::connect(format!("{}:{}", &self.ip, self.port)).await?;
            stream.writable().await?;
            stream.write_all(js.to_string().as_bytes()).await?;
            let mut resp = String::new();
            stream.readable().await?;
            stream.read_to_string(&mut resp).await?;
            resp = resp.replace("\0", "");
            
            let status: LogsResponse = serde_json::from_str(&resp)?;
            if status.status == common::StatusCode::SUCC {
                let mut resp = String::new();
                stream.readable().await?;
                stream.read_to_string(&mut resp).await?;
                resp = resp.replace("\0", "");
                Ok(resp.split('\n').map(|s| s.to_string()).collect())
            } else {
                //println!("Failed to get logs");
                Err(Error::Unauthorized)
            }
        } else {
            Err(Error::Unauthorized)
        }
    }

    async fn get_mac(&self) -> Result<String, Error> {
        let resp = self.send_recv(&json!({"cmd":"get_miner_info"})).await?;
        if let Ok(_) = serde_json::from_str::<wmapi::Status>(&resp) {
            // Older API version
            let sum = self.get_summary().await?;
            let sum = sum.as_ref().unwrap_or_else(|| unreachable!());
            sum.summary[0].mac.clone().ok_or(Error::ApiCallFailed("Failed to get MAC".to_string()))
        } else {
            let resp: wmapi::MinerInfoResponse = serde_json::from_str(&resp)?;
            Ok(resp.msg.mac.clone())
        }
    }

    async fn get_errors(&mut self) -> Result<Vec<MinerError>, Error> {
        let resp = self.send_recv(&json!({"cmd":"get_error_code"})).await?;
        // Whatsminer again returning invalid JSON
        //{"error_code":["111":"2022-10-20 09:18:54","110":"2022-10-20 09:18:54","2010":"1970-01-02 08:00:04"]}
        //TODO: it might be cheaper to regex this
        let resp = resp.replace("[", "{").replace("]", "}");
        let resp = serde_json::from_str::<wmapi::ErrorResp>(&resp)?;
        // Our response is a hashmap of error_code : datetime
        // I only care about the error codes, throw them into a single string to regex against
        let log = resp.msg.error_code.keys()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join("\n");
        let mut errors = HashSet::new();
        for err in WHATSMINER_ERRORS.iter() {
            if let Some(msg) = err.get_err(&log) {
                errors.insert(msg);
            }
        }
        Ok(errors.into_iter().collect())
    }

    async fn get_dns(&self) -> Result<String, Error> {
        let resp = self.send_recv(&json!({"cmd":"get_miner_info"})).await?;
        if let Ok(_) = serde_json::from_str::<wmapi::Status>(&resp) {
            Err(Error::NotSupported)
        } else {
            let resp: wmapi::MinerInfoResponse = serde_json::from_str(&resp)?;
            Ok(resp.msg.dns.clone())
        }
    }

    async fn get_profile(&self) -> Result<Profile, Error> {
        Err(Error::NotSupported)
    }

    async fn get_profiles(&self) -> Result<Vec<Profile>, Error> {
        Err(Error::NotSupported)
    }

    async fn set_profile(&mut self, _profile: Profile) -> Result<(), Error> {
        Err(Error::NotSupported)
    }

    async fn get_hashboard(&mut self) -> Result<String, Error> {
        Err(Error::NotSupported)
    }

    async fn get_hashboards(&self) -> Result<usize, Error> {
        Err(Error::NotSupported)
    }
}
