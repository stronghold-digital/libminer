use std::collections::HashMap;

use async_trait::async_trait;
use reqwest::multipart::Form;
use serde_json::json;
use tracing::{warn, error};

use crate::Client;
use crate::miner::{Miner, Pool};
use crate::miners::{minerva, common};
use crate::error::Error;
use minerva::{cgminer, minera};

/// 4 fan Minervas use this interface
pub struct Minera {
    ip: String,
    port: u16,
    model: Option<String>,
    client: Client,
}

#[async_trait]
impl Miner for Minera {
    fn new(client: Client, ip: String, port: u16) -> Self {
        Minera {
            ip,
            port,
            model: None,
            client,
        }
    }

    fn get_type(&self) -> &'static str {
        "Minerva (Minera)"
    }

    async fn get_model(&self) -> Result<String, Error> {
        //TODO: Pull from web API
        let resp = self.client.send_recv(&self.ip, self.port, &json!({"command":"devdetails"})).await?;
        let js = serde_json::from_str::<common::DevDetailsResp>(&resp)?;
        Ok(js.devdetails.get(0).unwrap().model.clone())
    }

    async fn auth(&mut self, _username: &str, password: &str) -> Result<(), Error> {
        let mut form = HashMap::new();
        form.insert("password", password);
        let resp = self.client.http_client
            .post(&format!("http://{}/index.php/app/login", self.ip))
            .form(&form)
            .send()
            .await?;
        if resp.status().is_success() {
            Ok(())
        } else {
            Err(Error::HttpRequestFailed)
        }
    }

    async fn reboot(&mut self) -> Result<(), Error> {
        //TODO: This always times out as the API reboots before responding
        let resp = self.client.http_client
            .post(&format!("http://{}/index.php/app/reboot", self.ip))
            .query(&[("confirm", "1")])
            .send()
            .await?;
        if resp.status().is_success() {
            Ok(())
        } else {
            Err(Error::HttpRequestFailed)
        }
    }

    async fn get_hashrate(&self) -> Result<f64, Error> {
        let resp = self.client.http_client
            .get(&format!("http://{}/index.php/app/stats", self.ip))
            .send()
            .await?;
        if resp.status().is_success() {
            let stat: minera::StatsResp = resp.json().await?;
            // Convert to TH/S
            Ok((stat.totals.hashrate as f64) / 1000000000000.0)
        } else {
            Err(Error::HttpRequestFailed)
        }
    }

    async fn get_nameplate_rate(&self) -> Result<f64, Error> {
        unimplemented!()
    }

    async fn get_temperature(&self) -> Result<f64, Error> {
        let resp = self.client.http_client
            .get(&format!("http://{}/index.php/app/stats", self.ip))
            .send()
            .await?;
        if resp.status().is_success() {
            let stat = resp.json::<minera::StatsResp>().await?;
            Ok(stat.temp)
        } else {
            Err(Error::HttpRequestFailed)
        }
    }

    async fn get_fan_speed(&self) -> Result<Vec<u32>, Error> {
        // let resp = self.client.http_client
        //     .get(&format!("http://{}/index.php/app/api", self.ip))
        //     .query(&[("command", "miner_stats")])
        //     .send()
        //     .await?;
        // if resp.status().is_success() {
        //     println!("{:?}", resp.text().await?);
        //     Ok(vec![])
        // } else {
        //     Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Failed to get stats")))
        // }
        //TODO: I can get Fan0 Speed but not the others
        unimplemented!()
    }

    async fn get_pools(&self) -> Result<Vec<Pool>, Error> {
        let resp = self.client.http_client
            .get(&format!("http://{}/index.php/app/stats", self.ip))
            .send()
            .await?;
        if resp.status().is_success() {
            let stat = resp.json::<minera::StatsResp>().await?;
            Ok(stat.pools.iter().map(|p| Pool {
                url: p.url.clone(),
                username: p.user.clone(),
                password: if p.pass {Some("*****".to_string())} else {None},
            }).collect())
        } else {
            Err(Error::HttpRequestFailed)
        }
    }

    async fn set_pools(&mut self, pools: Vec<Pool>) -> Result<(), Error> {
        let mut form = Form::new()
            .text("save_miner_pools", "1");

        for pool in pools {
            form = form
                .text("pool_url[]", pool.url.clone())
                .text("pool_username[]", pool.username.clone())
                .text("pool_password[]", if let Some(ref password) = pool.password {
                    password.clone()
                } else {
                    "".to_string()
                });
        }
        let resp = self.client.http_client
            .post(&format!("http://{}/index.php/app/settings", self.ip))
            .multipart(form)
            .send()
            .await?;
        if resp.status().is_success() {
            Ok(())
        } else {
            Err(Error::HttpRequestFailed)
        }
    }

    async fn set_sleep(&mut self, sleep: bool) -> Result<(), Error> {
        let webresp = self.client.http_client
            .get(&format!("http://{}/index.php/app/save_settings", self.ip))
            .query(&[("save_config", "1")])
            .send()
            .await?;
        if webresp.status().is_success() {
            //println!("{:?}", webresp.text().await?);
        }
        let resp = self.client.send_recv(&self.ip, self.port, &json!({"command":"asccount"})).await?;
        let asccount : common::AscIdentifyResp = serde_json::from_str(&resp)?;
        for i in 0..asccount.ascs[0].count {
            let resp2 = self.client.send_recv(
                &self.ip,
                self.port,
                &json!({
                    "command" : if sleep { "ascdisable" } else { "ascenable" },
                    "parameter" : &i.to_string(),
                }),
            ).await?;
            //println!("{:?}", resp2);
        }
        Ok(())
    }

    async fn set_blink(&mut self, blink: bool) -> Result<(), Error> {
        unimplemented!()
    }

    async fn get_logs(&mut self) -> Result<Vec<String>, Error> {
        unimplemented!()
    }

    async fn get_mac(&self) -> Result<String, Error> {
        let resp = self.client.http_client
            .get(&format!("http://{}/index.php/app/stats", self.ip))
            .send()
            .await?;
        if resp.status().is_success() {
            let stat = resp.json::<minera::StatsResp>().await?;
            Ok(stat.mac_addr)
        } else {
            Err(Error::HttpRequestFailed)
        }
    }
}

/// 2 fan Minervas use this interface
pub struct Minerva {
    ip: String,
    port: u16,
    client: Client,
    token: String,
}

#[async_trait]
impl Miner for Minerva {
    fn new(client: Client, ip: String, port: u16) -> Self {
        Minerva {
            ip,
            port,
            client,
            token: "".to_string(),
        }
    }

    fn get_type(&self) -> &'static str {
        "Minerva"
    }

    async fn get_model(&self) -> Result<String, Error> {
        let resp = self.client.send_recv(&self.ip, self.port, &json!({"command":"devdetails"})).await?;
        let js = serde_json::from_str::<common::DevDetailsResp>(&resp)?;
        Ok(js.devdetails.get(0).unwrap().model.clone())
    }

    async fn auth(&mut self, username: &str, password: &str) -> Result<(), Error> {
        let resp = self.client.http_client
            .post(&format!("https://{}/api/v1/auth/login", self.ip))
            .json(&json!({
                "username": username,
                "password": password,
            }))
            .send()
            .await?;
        if resp.status().is_success() {
            let text = resp.text().await?;
            if let Ok(js) = serde_json::from_str::<cgminer::AuthResp>(&text) {
                self.token = js.data.access_token.clone();
                Ok(())
            } else if let Ok(_) = serde_json::from_str::<cgminer::ApiResp>(&text) {
                //TODO: Check returned status code and return appropriate error
                Err(Error::Unauthorized)
            } else {
                Err(Error::UnknownMinerType)
            }
        } else {
            Err(Error::HttpRequestFailed)
        }
    }

    async fn reboot(&mut self) -> Result<(), Error> {
        //TODO: This always times out as the API reboots before responding
        let resp = self.client.http_client
            .post(&format!("https://{}:/api/v1/cgminer/reboot", self.ip))
            .bearer_auth(&self.token)
            .send()
            .await;
        Ok(())
    }

    async fn get_hashrate(&self) -> Result<f64, Error> {
        let resp = self.client.http_client
            .get(&format!("https://{}/api/v1/cgminer/summary", self.ip))
            .bearer_auth(&self.token)
            .send()
            .await?;
        if resp.status().is_success() {
            let text = resp.text().await?;
            if let Ok(summary) = serde_json::from_str::<cgminer::SummaryResp>(&text) {
                // Convert to TH/s
                Ok(summary.data[0].mhs_5s / 1000000.0)
            } else if let Ok(status) = serde_json::from_str::<cgminer::ApiResp>(&text) {
                warn!("Failed to get hashrate {}", if let Some(msg) = status.data { msg } else { "Unknown error".to_string() });
                // The miners up but didn't give us a great response, so just return 0
                Ok(0.0)
            } else {
                Err(Error::ApiCallFailed("Unknown error".to_string()))
            }
        } else {
            Err(Error::HttpRequestFailed)
        }
    }

    async fn get_nameplate_rate(&self) -> Result<f64, Error> {
        unimplemented!()
    }

    async fn get_temperature(&self) -> Result<f64, Error> {
        let resp = self.client.http_client
            .get(&format!("https://{}/api/v1/cgminer/tempAndSpeed", self.ip))
            .bearer_auth(&self.token)
            .send()
            .await?;
        if resp.status().is_success() {
            let temp = resp.json::<cgminer::TempAndSpeedResp>().await?;
            // Convert to C
            Ok(temp.temperature)
        } else {
            Err(Error::HttpRequestFailed)
        }
    }

    async fn get_fan_speed(&self) -> Result<Vec<u32>, Error> {
        let resp = self.client.http_client
            .get(&format!("https://{}/api/v1/cgminer/tempAndSpeed", self.ip))
            .bearer_auth(&self.token)
            .send()
            .await?;
        if resp.status().is_success() {
            let temp = resp.json::<cgminer::TempAndSpeedResp>().await?;
            Ok(vec![temp.fan_speed1, temp.fan_speed2])
        } else {
            Err(Error::HttpRequestFailed)
        }
    }

    async fn get_pools(&self) -> Result<Vec<Pool>, Error> {
        let resp = self.client.http_client
            .get(&format!("https://{}/api/v1/cgminer/pools", self.ip))
            .bearer_auth(&self.token)
            .send()
            .await?;
        if resp.status().is_success() {
            let pools = resp.json::<cgminer::GetPoolsResp>().await?;
            let mut ret = Vec::new();
            for pool in pools.data {
                ret.push(Pool {
                    url: pool.url,
                    username: pool.user,
                    password: None,
                });
            }
            Ok(ret)
        } else {
            Err(Error::HttpRequestFailed)
        }
    }

    async fn set_pools(&mut self, pools: Vec<Pool>) -> Result<(), Error> {
        let resp = self.client.http_client
            .post(&format!("https://{}/api/v1/cgminer/changePool", self.ip))
            .bearer_auth(&self.token)
            .json(&cgminer::SetPoolRequest {
                pool0url: &pools[0].url,
                pool0user: &pools[0].username,
                pool0pwd: if let Some(pwd) = &pools[0].password {&pwd} else {""},
                pool1url: &pools[1].url,
                pool1user: &pools[1].username,
                pool1pwd: if let Some(pwd) = &pools[1].password {&pwd} else {""},
                pool2url: &pools[2].url,
                pool2user: &pools[2].username,
                pool2pwd: if let Some(pwd) = &pools[2].password {&pwd} else {""},
            })
            .send()
            .await?;
        if resp.status().is_success() {
            Ok(())
        } else {
            Err(Error::HttpRequestFailed)
        }
    }

    async fn set_sleep(&mut self, sleep: bool) -> Result<(), Error> {
        let resp1 = self.client.http_client
            .get(&format!("https://{}/api/v1/cgminer/workMode", self.ip))
            .bearer_auth(&self.token)
            .send()
            .await?;
        //println!("{}", resp1.text().await.unwrap());
        let js = resp1.json::<serde_json::Value>().await?;
        let mut hash = js.as_object().unwrap().clone();
        let data = hash.get_mut("data").unwrap();
        //data["mask"] = serde_json::Value::from(if sleep { "0x0" } else { "0xf" });
        let mut default = serde_json::Map::new();
        let data = data.as_object_mut().unwrap_or(&mut default);
        data.remove("mask");
        data.insert("mask".to_string(), serde_json::Value::from(if sleep { "0x0" } else { "0xf" }));
        //println!("{:?}", data);
        let resp = self.client.http_client
            .post(&format!("https://{}/api/v1/cgminer/setWorkMode", self.ip))
            .bearer_auth(&self.token)
            .json(&data)
            .send()
            .await?;
        if resp.status().is_success() {
            Ok(())
        } else {
            Err(Error::HttpRequestFailed)
        }
    }

    async fn set_blink(&mut self, blink: bool) -> Result<(), Error> {
        unimplemented!()
    }

    async fn get_logs(&mut self) -> Result<Vec<String>, Error> {
        unimplemented!()
    }

    async fn get_mac(&self) -> Result<String, Error> {
        let resp = self.client.http_client
            .get(&format!("https://{}/api/v1/systemInfo/network", self.ip))
            .bearer_auth(&self.token)
            .send()
            .await?;
        if resp.status().is_success() {
            let network = resp.json::<cgminer::NetworkResponse>().await?;
            Ok(network.data.hardwareAddress)
        } else {
            Err(Error::HttpRequestFailed)
        }
    }
}