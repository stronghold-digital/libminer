use serde::Deserialize;

use super::Status;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Pool {
    #[serde(rename = "POOL")]
    pub pool: u8,
    #[serde(rename = "URL")]
    pub url: String,
    pub user: String,
}

impl Into<crate::Pool> for Pool {
    fn into(self) -> crate::Pool {
        crate::Pool {
            url: self.url,
            username: self.user,
            password: None,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct PoolResp {
    #[serde(rename = "STATUS")]
    pub status: [Status; 1],
    #[serde(rename = "POOLS")]
    pub pools: Vec<Pool>,
}

impl Into<Vec<crate::Pool>> for PoolResp {
    fn into(self) -> Vec<crate::Pool> {
        self.pools.into_iter().map(|p| p.into()).collect()
    }
}