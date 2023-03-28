use reqwest::Error as ReqwestError;
use thiserror::Error;
use std::io::Error as IoError;
use serde_json::Error as JsonError;
use digest_auth::Error as DigestAuthError;
use reqwest::header::ToStrError;

#[derive(Error, Debug)]
pub enum Error {
    // Errors bubbled from dependencies
    #[error("Reqwest error {0}")]
    RequestError(#[from] ReqwestError),
    #[error("Io error {0}")]
    IoError(#[from] IoError),
    #[error("Json error {0}")]
    ParseError(#[from] JsonError),
    #[error("Digest auth error {0}")]
    DigestAuthError(#[from] DigestAuthError),
    #[error("ToStr error")]
    ToStrError(#[from] ToStrError),
    #[error("Failed to acquire semaphore")]
    SemaphoreError(#[from] tokio::sync::AcquireError),

    #[cfg(feature = "avalon")]
    #[error("Avalon deserializer error")]
    AvalonDeserializerError(#[from] crate::miners::avalon::DeError),

    // Errors from this library
    // Detection errors
    #[error("No host detected")]
    NoHostDetected,
    #[error("Unknown miner type {0}")]
    UnknownMinerType(String),
    #[error("No miner detected")]
    NoMinerDetected,
    
    // Response parsing errors
    #[error("Encode error")]
    EncodingError,

    // Network errors
    #[error("Timeout")]
    Timeout,
    #[error("Connection refused")]
    ConnectionRefused,
    #[error("Failed to execute HTTP request")]
    HttpRequestFailed,

    // API errors
    #[error("Token expired")]
    TokenExpired,
    #[error("Unauthorized")]
    Unauthorized,
    #[error("API Call failed: {0}")]
    ApiCallFailed(String),
    #[error("Expected return")]
    ExpectedReturn,
    #[error("Not supported")]
    NotSupported,
    #[error("Invalid response")]
    InvalidResponse,
    #[error("Unknown model {0}")]
    UnknownModel(String),
}
