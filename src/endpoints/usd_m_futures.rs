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
    Future
}
pub enum ContractType {
    Perpetual,
    CurrentMonth,
    NextMonth,
    CurrentQuarter,
    NextQuarter,
    PerpetualDelivering,
}

#[derive(Debug, APIEndPoint)]
#[allow(dead_code, non_camel_case_types)]
pub enum USD_M_FutureEP {
    #[endpoint(GET, MarketData, url = "/fapi/v2/ticker/price")]
    SymbolPriceTicker,
    #[endpoint(GET, MarketData, url = "/fapi/v1/ticker/bookTicker")]
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
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SymbolPriceTicker {
    pub symbol: String,
    pub price: String,
    pub time: i64,
}

impl EndpointRequest for SymbolPriceTickerRequest {
    type Response = OneOrMany<SymbolPriceTicker>;
}
