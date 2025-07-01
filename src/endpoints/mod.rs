pub mod convert;
pub mod margin;
pub mod market_data;
pub mod spot_account;
pub mod spot_trading;
pub mod wallet;

pub use http::Method;

use binance_cex_macros::{ApiRequestRequire, ApiRequestToString};
use serde::{ser::SerializeStruct, Deserialize, Serialize};

pub trait Endpoint {
    fn action_params(&self) -> (Method, SecurityType, String);
}

pub trait EndpointRequest: ToString {
    type Response: for<'de> Deserialize<'de>;
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SecurityType {
    None,
    Trade,
    Margin,
    UserData,
    UserStream,
    MarketData,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum OneOrMany<T> {
    One(T),
    Many(Vec<T>),
}

#[macro_export]
macro_rules! one_or_many_to_hashmap {
    ($one_or_many:expr, $key:ident) => {
        match $one_or_many {
            $crate::endpoints::OneOrMany::One(v) => {
                let mut r = std::collections::HashMap::new();
                r.insert(v.$key.clone(), v);
                r
            }
            $crate::endpoints::OneOrMany::Many(vs) => {
                vs.into_iter().map(|v| (v.$key.clone(), v)).collect()
            }
        }
    };
}

pub type OneOrManySymbol = OneOrMany<String>;
impl Serialize for OneOrManySymbol {
    fn serialize<S>(&self, serializer: S) -> std::prelude::v1::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::One(s) => {
                let mut seq = serializer.serialize_struct("OneOrMany", 1)?;
                seq.serialize_field("symbol", s)?;
                seq.end()
            }
            Self::Many(ss) => {
                let mut seq = serializer.serialize_struct("OneOrMany", 1)?;
                let mut ass = vec![];
                for item in ss.iter() {
                    ass.push(format!("\"{}\"", item));
                }
                seq.serialize_field("symbols", &format!("[{}]", ass.join(",")))?;
                seq.end()
            }
        }
    }
}

#[derive(Debug, Serialize, Default, ApiRequestRequire, ApiRequestToString)]
#[serde(rename_all = "camelCase")]
pub struct BaseRequest {
    recv_window: Option<u64>,
    timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum ResponseType {
    Ack,
    Result,
    Full,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum Response<T> {
    Error { code: i64, msg: String },
    Data(T),
}

#[cfg(test)]
mod tests {
    use crate::api_config;
    use crate::client;
    use static_init::dynamic;

    use super::*;
    // use futures_util::{AsyncReadExt, StreamExt};

    #[dynamic]
    static MAINNET: api_config::ApiConfig = api_config::ApiConfig::default();

    #[dynamic]
    static CLIENT: client::Client =
        client::Client::new(None, None, &MAINNET.rest_api_endpoint.clone());

    #[test]
    fn create_base_request() {
        let mut req = BaseRequest::require();
        req.recv_window = Some(30);
        println!("{:?}", req);
    }

    // #[tokio::test]
    // async fn wallet_system_status() {

    //     let wallet = wallet::Wallet::new(CLIENT.clone(), &MAINNET);
    //     wallet.system_status().await.unwrap();
    // }

    // #[tokio::test]
    // async fn ping() {
    //     let binance = spot_trading::SpotTrading::new(CLIENT.clone(), &MAINNET);
    //     // binance.ping().await.unwrap();
    // }
}
