use crate::models::*;
use crate::{
    endpoints::{BaseRequest, Endpoint, SecurityType},
    models::BnbBurnStatus,
};
use binance_api_macros::{APIEndPoint, APIRequestInit, APIRequestToString};

use serde::{Serialize, Deserialize};
use serde_qs::to_string;

use super::{EndpointRequest, OneOrMany};

pub enum SymbolType {
    Delivery,
    Perpetual,
}
pub enum ContractType {
    Perpetual,
    CurrentQuarter,
    NextQuarter,
    CurrentQuarterDelivering,
    NextQuarterDelivering,
    PerpetualDelivering,
}

#[derive(Debug, APIEndPoint)]
#[allow(dead_code)]
pub enum COIN_M_FutureEP {
    #[endpoint(GET, MarketData, url = "/dapi/v1/ticker/price")]
    SymbolPriceTicker,
    #[endpoint(GET, MarketData, url = "/dapi/v1/ticker/bookTicker")]
    SymbolOrderBookTicker,
    // CrossMarginPairs,
    // #[endpoint(GET, UserData, url = "/sapi/v1/margin/crossMarginData")]
    // CrossMarginFeeData,
    // #[endpoint(GET, UserData, url = "/sapi/v1/margin/isolated/allPairs")]
    // IsolatedMarginPairs,
    // #[endpoint(GET, UserData, url = "/sapi/v1/margin/isolatedMarginData")]
    // IsolatedMarginFeeData,
}

#[derive(Debug, Serialize, APIRequestInit, APIRequestToString)]
pub struct SymbolPriceTickerRequest {
    pub symbol: Option<String>,
    pub pair: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SymbolPriceTickerResponse {
    pub symbol: String,
    pub ps: String,
    pub price: String,
    pub time: i64,
}

impl EndpointRequest for SymbolPriceTickerRequest {
    type Response = OneOrMany<SymbolPriceTickerResponse>;

    fn validate(&self) -> anyhow::Result<()> {
        assert!(self.symbol.is_none() || self.pair.is_none());
        Ok(())
    }
}
