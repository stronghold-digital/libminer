use serde::{Deserialize, de::Deserializer};

#[derive(PartialEq, Debug)]
pub enum StatusCode {
    Running,
    Initializing,
    AutoTuning,
    Restarting,
    Failure,
    ShuttingDown,
    Stopped,
}

impl<'de> Deserialize<'de> for StatusCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "mining" => Ok(StatusCode::Running),
            "initializing" => Ok(StatusCode::Initializing),
            "auto-tuning" => Ok(StatusCode::AutoTuning),
            "restarting" => Ok(StatusCode::Restarting),
            "failure" => Ok(StatusCode::Failure),
            "shutting-down" => Ok(StatusCode::ShuttingDown),
            "stopped" => Ok(StatusCode::Stopped),
            _ => Err(serde::de::Error::custom(format!("Unknown status code: {}", s))),
        }
    }
}

#[derive(Deserialize)]
pub struct MinerStatus {
    pub restart_required: bool,
    pub miner_state: StatusCode,
    pub miner_state_time: usize,
    pub find_miner: bool,
    pub unlocked: bool,
    pub warranty: bool,
}