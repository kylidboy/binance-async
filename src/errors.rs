use reqwest::Error as RWError;
use thiserror::Error;
use tungstenite::Error as WsErr;

pub type Result<T> = eyre::Result<T, BinanceApiError>;

#[derive(Debug, Error)]
pub enum BinanceApiError {
    #[error("URL: {0}")]
    URLError(#[from] url::ParseError),

    #[error("JSON: {0}")]
    JSONValueSerDeError(#[from] serde_json::Error),

    #[error("Websocket handshake: {0}")]
    WsHandShake(#[from] WsErr),

    #[error("Request error: {0}")]
    RequestError(#[from] RWError),

    #[error("invalid Vec for Kline: {1} at {0} is missing")]
    KlineValueMissingError(usize, &'static str),

    #[error("Symbol not found: {0}")]
    SymbolNotFound(String),

    #[error("Asset not found: {0}")]
    AssetNotFound(String),

    #[error("System maintenance")]
    WalletMaintenance,

    #[error("Stream has already connected")]
    StreamAlreadyConnected,

    #[error("Invalid header value")]
    InvalidHeaderValue,

    #[error("Api error: {0}, {1}")]
    ApiReturnError(i64, String),

    #[error("Custom error: {0}")]
    Custom(String),
}
