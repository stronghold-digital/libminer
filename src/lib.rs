mod util;
pub mod miners;
mod miner;

pub use miner::{Miner, Pool, Profile, MinerError, ErrorType};
pub mod error;

use miners::*;
use error::Error;
use reqwest;
use serde_json::json;
use tracing::{debug, instrument};
use lazy_regex::regex;
use std::sync::Arc;
use std::collections::HashMap;
use chrono::{DateTime, Utc};

use tokio::{
    self,
    net::TcpStream,
    io::{AsyncWriteExt, AsyncReadExt},
    sync::{RwLock, Semaphore},
    time::Duration,
};

/*
 * Cgminer socket API has a tendency to fail often but is generally universal
 * Failing this, most miners have an API exposed over HTTP, but these are highly specific
 */

#[derive(Debug, Clone)]
pub struct CacheItem {
    pub token: String,
    pub token_expires: DateTime<Utc>,
}

pub type Cache = Arc<RwLock<HashMap<String, CacheItem>>>;

pub struct ClientBuilder {
    connect_timeout: Duration,
    request_timeout: Duration,
    max_connections: usize,
    cache_token: bool,
}

impl ClientBuilder {
    pub fn new () -> Self {
        Self {
            connect_timeout: Duration::from_secs(15),
            request_timeout: Duration::from_secs(30),
            max_connections: 0,
            cache_token: false,
        }
    }

    /// Set the connect timeout for the client
    /// Default is 5 seconds
    pub fn connect_timeout(mut self, timeout: Duration) -> Self {
        self.connect_timeout = timeout;
        self
    }

    /// Set the request timeout for the client
    /// Default is 10 seconds
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = timeout;
        self
    }

    /// Set the max amount of simultaneous connections for the client
    /// Default is 0, or unlimited
    pub fn max_connections(mut self, max: usize) -> Self {
        self.max_connections = max;
        self
    }

    /// Set whether or not to cache the tokens for the client instance
    pub fn cache_token(mut self, cache: bool) -> Self {
        self.cache_token = cache;
        self
    }

    pub fn build(self) -> Result<Client, Error> {
        let client = reqwest::ClientBuilder::new()
            .user_agent("libminer/0.1")
            .connect_timeout(self.connect_timeout)
            .timeout(self.request_timeout)
            //.tcp_keepalive(None)
            .tcp_nodelay(true) // Disable Nagle's algorithm, which can cause latency issues
            .danger_accept_invalid_certs(true) // Accept self-signed certs
            .cookie_store(true) // Some miners require a cookie store
            .pool_max_idle_per_host(0)
            .pool_idle_timeout(Duration::from_secs(10))
            .build()?;
        let lock = {
            if self.max_connections > 0 {
                Some(Arc::new(Semaphore::new(self.max_connections)))
            } else {
                None
            }
        };
        Ok(Client {
            http_client: client,
            connect_timeout: self.connect_timeout,
            request_timeout: self.request_timeout,
            lock,
            tokens: if self.cache_token { Some(Arc::new(RwLock::new(HashMap::new()))) } else { None },
        })
    }
}

#[derive(Clone, Debug)]
pub struct Client {
    http_client: reqwest::Client,
    connect_timeout: Duration,
    request_timeout: Duration,
    lock: Option<Arc<Semaphore>>,
    tokens: Option<Cache>,
}

impl Client {
    /// Connect to a given host with the timeout specified
    async fn connect(&self, ip: &str, port: u16) -> Result<TcpStream, Error> {
        match tokio::time::timeout(
            self.connect_timeout,
            TcpStream::connect(format!("{}:{}", ip, port))
        ).await {
            Ok(Ok(stream)) => Ok(stream),
            Ok(Err(_)) => Err(Error::NoHostDetected),
            Err(_) => Err(Error::Timeout),
        }
    }

    /// Connect to a host and send data return data as String, close connection after request
    async fn send_recv<T>(&self, ip: &str, port: u16, data: &T) -> Result<String, Error> 
        where T: ToString + ?Sized
    {
        let mut stream = self.connect(ip, port).await?;
        match tokio::time::timeout(
            self.request_timeout,
            async {
                stream.writable().await?;
                stream.write_all(data.to_string().as_bytes()).await?;
                let mut buf = String::new();
                stream.readable().await?;
                stream.read_to_string(&mut buf).await?;
                buf = buf.replace("\0", ""); // Fix for Antminer bug
                Ok(buf)
            }
        ).await {
            Ok(result) => result,
            Err(_) => Err(Error::Timeout),
        }
    }

    /// Send data over a websocket to a host
    async fn send<T>(&self, ip: &str, port: u16, data: &T) -> Result<(), Error> 
        where T: ToString
    {
        let mut stream = self.connect(ip, port).await?;
        match tokio::time::timeout(
            self.request_timeout,
            async {
                stream.writable().await?;
                stream.write_all(data.to_string().as_bytes()).await?;
                Ok(())
            }
        ).await {
            Ok(result) => result,
            Err(_) => Err(Error::Timeout),
        }
    }

    /// Attempts to perform miner detection against the cgminer socket API roughly implemented by most miners
    /// NOTES:
    /// * On Minervas using the Minera interface, the cgminer API can be deadlocked
    /// * On Whatsminers, the socket API can be responsive but btminer deadlocked, this results in detection successful but every call failing
    async fn socket_detect(&self, ip: &str, port: u16) -> Result<Box<dyn Miner + Send + Sync>, Error> {
        debug!("Trying socket detection...");
        match self.send_recv(ip, port, &json!({"command": "stats"})).await {
            Ok(resp) => {
                debug!("Received response from socket API...");
                if let Ok(stats_resp) = serde_json::from_str::<common::StatsResp>(&resp) {
                    debug!("Received valid cgminer response.");
                    if stats_resp.status[0].status != common::StatusCode::SUCC {
                        return Err(Error::ApiCallFailed(stats_resp.status[0].msg.clone()));
                    }
                    if let Some(stats) = stats_resp.stats {
                        debug!("Checking for type in stats response...");
                        for stat in stats {
                            match stat {
                                #[cfg(feature = "antminer")]
                                common::Stats::AmVersion(_) => {
                                    debug!("Found Antminer miner at {}", ip);
                                    return Ok(Box::new(antminer::Antminer::new(self.clone(), ip.into(), port)));
                                },
                                #[cfg(feature = "avalon")]
                                common::Stats::AvaStats(_) => {
                                    debug!("Found Avalon miner at {}", ip);
                                    return Ok(Box::new(avalon::Avalon::new(self.clone(), ip.into(), port)));
                                },
                                #[cfg(feature = "minerva")]
                                common::Stats::Dev(stat) => {
                                    if let Some(type_) = stat.type_ {
                                        if type_ == "Minerva" {
                                            // We need to differentiate between the 2 interfaces
                                            // easiest thing is to send a GET request to /index.php
                                            // If we get a 200, we know its running minera
                                            debug!("Found Minerva, determining interface...");
                                            let resp2 = self.http_client
                                                .get(&format!("http://{}/index.php", ip))
                                                .send()
                                                .await?;
                                            return match resp2.status() {
                                                reqwest::StatusCode::NOT_FOUND => {
                                                    debug!("Found Minerva (Custom Interface) at {}", ip);
                                                    Ok(Box::new(minerva::Minerva::new(self.clone(), ip.into(), port)))
                                                }
                                                reqwest::StatusCode::OK => {
                                                    debug!("Found Minerva (Minera Interface) at {}", ip);
                                                    Ok(Box::new(minerva::Minera::new(self.clone(), ip.into(), port)))
                                                }
                                                _ => {
                                                    debug!("Unable to determine interface for Minerva at {}", ip);
                                                    Err(Error::UnknownMinerType("Unable to determine interface for Minerva".into()))
                                                },
                                            };
                                        } else {
                                            debug!("Unsupported miner type: {} at {}", type_, ip);
                                            return Err(Error::UnknownMinerType(format!("Unsupported miner type: {}", type_)));
                                        }
                                    } else {
                                        debug!("Miner did not include type in response at {}", ip);
                                        return Err(Error::UnknownMinerType("Miner did not include type in response".into()));
                                    }
                                }
                                _ => {} // We don't care about the other stats
                            }
                        }
                        debug!("Stats did not include a section containing type at {}\n{}", ip, resp);
                        return Err(Error::UnknownMinerType("Stats did not include a section containing type".into()));
                    } else {
                        debug!("Unable to parse stats response at {}\n{}", ip, resp);
                        return Err(Error::UnknownMinerType("Unable to parse stats response".into()));
                    }
                } else if let Ok(status) = serde_json::from_str::<common::Status>(&resp) {
                    // Whatsminer returns just the cgminer status error with invalid json and a description containing whatsminer
                    // {"STATUS":"E","When":"0","Code":23,"Msg":"Invalid JSON","Description":"whatsminer"}
                    //TODO: Don't hardcode the status code for Invalid Command
                    #[cfg(feature = "whatsminer")]
                    if status.status == common::StatusCode::ERROR && status.code == Some(14) {
                        // lowercase and regex the description for "whatsminer"
                        if let Some(desc) = status.description {
                            if desc.to_lowercase().contains("whatsminer") {
                                debug!("Found Whatsminer at {}", ip);
                                return Ok(Box::new(whatsminer::Whatsminer::new(self.clone(), ip.into(), port)));
                            }
                        }
                    }
                    debug!("Received error response but not whatsminer at {}\n{}", ip, resp);
                    return Err(Error::UnknownMinerType("Received error response but not whatsminer".into()));
                } else {
                    debug!("Unable to parse response from socket API: {}", resp);
                    return Err(Error::UnknownMinerType("Unable to parse response from socket API".into()));
                }
            }
            Err(e) => {
                match e {
                    Error::NoHostDetected => Err(Error::NoMinerDetected),
                    e => Err(e),
                }
            }
        }
    }

    async fn http_detect(&self, ip: &str, port: u16) -> Result<Box<dyn Miner + Send + Sync>, Error> {
        debug!("Trying HTTP detection...");
        // To reduce traffic and since detection is entirely on status response, we can just send a HEAD request
        // Start with Antminer, if this fails to connect return a timeout
        match self.http_client.get(&format!("http://{}/", ip)).send().await {
            Ok(resp) => {
                debug!("Received response from HTTP API...");
                //TODO: In theory we could probably do this with a single request
                #[cfg(feature = "antminer")]
                if resp.status() == reqwest::StatusCode::UNAUTHORIZED {
                    if let Some(auth) = resp.headers().get("WWW-Authenticate") {
                        let re = regex!(r"^[Dd]igest");
                        if re.is_match(auth.to_str()?) {
                            debug!("Found Antminer at {}", ip);
                            return Ok(Box::new(antminer::Antminer::new(self.clone(), ip.into(), port)));
                        }
                    }
                }
                #[cfg(feature = "vnish")]
                {
                    debug!("Checking for VNISH...");
                    if let Ok(resp) = self.http_client.get(&format!("http://{}/", ip)).send().await {
                        let re = regex!(r#"(<title>miner-dash-app</title>|<meta name="firmware" content="AnthillOS">)"#);
                        if re.is_match(&resp.text().await?) {
                            debug!("Found VNISH at {}", ip);
                            return Ok(Box::new(vnish::Vnish::new(self.clone(), ip.into(), port)));
                        }
                    }
                }
                #[cfg(feature = "avalon")]
                {
                    let re = regex!(r"<title>Avalon Device</title>");
                    if let Ok(resp) = self.http_client.get(&format!("http://{}/", ip)).send().await {
                        if re.is_match(&resp.text().await?) {
                            debug!("Found Avalon at {}", ip);
                            return Ok(Box::new(avalon::Avalon::new(self.clone(), ip.into(), port)));
                        }
                    }
                }
                #[cfg(feature = "minerva")]
                {
                    // 4 fan minervas permit a request to /index.php/app/stats even when not logged in
                    debug!("Checking for minera Minerva...");
                    let text = resp.text().await?;
                    let re2 = regex!(r"minera.js");
                    if re2.is_match(&text) {
                        debug!("Found Minerva at {}", ip);
                        return Ok(Box::new(minerva::Minera::new(self.clone(), ip.into(), port)));
                    }
                    // 2 fan minervas have the title Minerva and are based off umi
                    debug!("Checking for custom Minerva...");
                    let re = regex!(r"Minerva(.|\n)+umi");
                    let resp = self.http_client.get(&format!("https://{}", ip)).send().await;
                    if let Ok(resp) = resp {
                        let text = resp.text().await?;
                        if re.is_match(&text) {
                            debug!("Found Minerva (Custom Interface) at {}", ip);
                            return Ok(Box::new(minerva::Minerva::new(self.clone(), ip.into(), port)));
                        }
                    }
                }

                #[cfg(feature = "whatsminer")]
                {
                    // Lastly check whatsminers, /cgi-bin/luci and look for whatsminer in the body
                    debug!("Checking for Whatsminer...");
                    let resp = self.http_client.get(&format!("http://{}/cgi-bin/luci", ip)).send().await?;
                    if resp.status() == reqwest::StatusCode::FORBIDDEN {
                        let re = regex!(r"<title>WhatsMiner");
                        if re.is_match(&resp.text().await?) {
                            debug!("Detected Whatsminer at {}:{}", ip, port);
                            //warn!("Socket API did not respond, this miner may not work.");
                            return Ok(Box::new(whatsminer::Whatsminer::new(self.clone(), ip.to_string(), port).with_cache(self.tokens.clone())));
                        }
                    }
                }

                debug!("Unable to determine miner type {}", ip);
                Err(Error::UnknownMinerType("".into()))
            }
            Err(e) => {
                if e.is_timeout() {
                    Err(Error::Timeout)
                } else if e.is_connect() {
                    Err(Error::NoMinerDetected)
                } else {
                    Err(e.into())
                }
            }
        }
    }

    /// Detects the type of miner at the given IP and port
    /// Default port is 4028
    #[instrument]
    pub async fn get_miner(&self, ip: &str, port: Option<u16>) -> Result<Box<dyn Miner + Send + Sync>, Error> {
        let port = port.unwrap_or(4028);
        let permit = {
            if let Some(lock) = &self.lock {
                Some(lock.clone().acquire_owned().await?)
            } else {
                None
            }
        };
        debug!("Detecting miner at {}:{}", ip, port);
        let miner = {
            match self.http_detect(ip, port).await {
                Ok(miner) => Ok(miner),
                Err(e) => {
                    match self.socket_detect(ip, port).await {
                        Ok(miner) => Ok(miner),
                        Err(e2) => {
                            // Handle some special error cases, default to returning the first error
                            match (e, e2) {
                                (Error::Timeout, Error::Timeout) => Err(Error::Timeout),
                                (Error::Timeout, e) => Err(e),
                                (e, Error::Timeout) => Err(e),
                                (Error::NoMinerDetected, Error::NoMinerDetected) => Err(Error::NoMinerDetected),
                                (Error::UnknownMinerType(s), Error::UnknownMinerType(s2)) => Err(Error::UnknownMinerType(format!("{} and {}", s, s2))),
                                (Error::UnknownMinerType(s), e) => Err(Error::UnknownMinerType(format!("{} and {}", s, e))),
                                (e, Error::UnknownMinerType(s)) => Err(Error::UnknownMinerType(format!("{} and {}", e, s))),
                                (Error::NoMinerDetected, e) => Err(Error::UnknownMinerType(format!("No miner detected and {}", e))),
                                (e, _) => { Err(e) }
                            }
                        }
                    }
                }
            }
        }?;
        if let Some(permit) = permit {
            Ok(Box::new(miner::LockMiner::new_locked(
                miner,
                permit,
            )) as Box<dyn Miner + Send + Sync>)
        } else {
            Ok(miner)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_miner() {
        let client = ClientBuilder::new().build().unwrap();

        let mut miner = client.get_miner("10.136.101.1", None).await.unwrap();

        miner.auth("root", "root").await.unwrap();

        miner.get_sleep().await.unwrap();
        miner.get_model().await.unwrap();
        miner.get_efficiency().await.unwrap();
        miner.get_hashrate().await.unwrap();
        miner.get_nameplate_power().await.unwrap();
        miner.get_power().await.unwrap();
    }
}