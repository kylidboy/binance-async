#![allow(dead_code)]

use crate::models::*;

use super::EndpointRequest;
use super::{Endpoint, Method, OneOrMany, OneOrManySymbol, SecurityType};

use binance_cex_macros::ApiRequestToString;
use either::Either;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use strum_macros::Display;

use binance_cex_macros::ApiRequestRequire;

#[derive(Debug, Display)]
#[allow(dead_code)]
pub enum MarketDataEP {
    #[strum(to_string = "/api/v3/ping")]
    Ping,
    #[strum(to_string = "/api/v3/time")]
    Time,
    #[strum(to_string = "/api/v3/exchangeInfo")]
    ExchangeInfo,
    #[strum(to_string = "/api/v3/depth")]
    OrderBook,
    #[strum(to_string = "/api/v3/ticker/price")]
    PriceTicker,
    #[strum(to_string = "/api/v3/avgPrice")]
    AvgPrice,
    #[strum(to_string = "/api/v3/ticker/bookTicker")]
    SymbolOrderBookTicker,
    #[strum(to_string = "/api/v3/ticker/24hr")]
    Ticker24hr,
    #[strum(to_string = "/api/v3/aggTrades")]
    AggTrades,
    #[strum(to_string = "/api/v3/klines")]
    Klines,
    #[strum(to_string = "/api/v3/trades")]
    Trades,
    #[strum(to_string = "/api/v3/historicalTrades")]
    HistoricalTrades,
}

impl Endpoint for MarketDataEP {
    fn action_params(&self) -> (http::Method, super::SecurityType, String) {
        match self {
            MarketDataEP::Ping => (Method::GET, SecurityType::None, self.to_string()),
            MarketDataEP::Time => (Method::GET, SecurityType::None, self.to_string()),
            MarketDataEP::ExchangeInfo => (Method::GET, SecurityType::None, self.to_string()),
            MarketDataEP::OrderBook => (Method::GET, SecurityType::None, self.to_string()),
            MarketDataEP::PriceTicker => (Method::GET, SecurityType::None, self.to_string()),
            MarketDataEP::AvgPrice => (Method::GET, SecurityType::None, self.to_string()),
            MarketDataEP::SymbolOrderBookTicker => (Method::GET, SecurityType::None, self.to_string()),
            MarketDataEP::Ticker24hr => (Method::GET, SecurityType::None, self.to_string()),
            MarketDataEP::AggTrades => (Method::GET, SecurityType::None, self.to_string()),
            MarketDataEP::Klines => (Method::GET, SecurityType::None, self.to_string()),
            MarketDataEP::Trades => (Method::GET, SecurityType::None, self.to_string()),
            MarketDataEP::HistoricalTrades => (Method::GET, SecurityType::None, self.to_string()),
        }
    }
}

#[derive(Debug, Serialize, ApiRequestRequire, ApiRequestToString)]
pub struct ExchangeInfoRequest {
    pub symbols: Option<OneOrManySymbol>,
    pub permissions: Option<AccountAndSymbolPermission>,
}
impl EndpointRequest for ExchangeInfoRequest {
    type Response = ExchangeInformation;
}

#[derive(Debug, Serialize, ApiRequestRequire, ApiRequestToString)]
pub struct OrderBookRequest {
    pub symbol: String,
    pub limit: Option<u64>,
}
impl EndpointRequest for OrderBookRequest {
    type Response = OrderBook;
}

#[derive(Debug, Serialize, ApiRequestToString)]
pub struct PriceTickerRequest(pub Option<OneOrManySymbol>);
impl EndpointRequest for PriceTickerRequest {
    type Response = OneOrMany<SymbolPrice>;
}

#[derive(Debug, Serialize, ApiRequestRequire, ApiRequestToString)]
#[serde(rename_all = "camelCase")]
pub struct AveragePriceRequest {
    pub symbol: String,
}
impl EndpointRequest for AveragePriceRequest {
    type Response = AveragePrice;
}

#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Ticker24hRequest {
    pub symbols: Option<OneOrManySymbol>,
    pub r#type: Option<Ticker24hReqType>,
}
#[derive(Debug, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Ticker24hReqType {
    FULL,
    MINI,
}
pub type Ticker24hResponse = OneOrMany<Either<PriceStatsFull, PriceStatsMini>>;

#[derive(Debug, Serialize, ApiRequestRequire, ApiRequestToString)]
#[serde(rename_all = "camelCase")]
pub struct KlinesRequest {
    symbol: String,
    interval: KlineInterval,
    start_time: Option<u64>,
    end_time: Option<u64>,
    time_zone: Option<String>,
    limit: Option<u16>,
}
impl EndpointRequest for KlinesRequest {
    type Response = Vec<Vec<Value>>;
}

#[derive(Debug, Serialize)]
pub enum KlineInterval {
    #[serde(rename = "1s")]
    _1s,
    #[serde(rename = "1m")]
    _1m,
    #[serde(rename = "3m")]
    _3m,
    #[serde(rename = "5m")]
    _5m,
    #[serde(rename = "15m")]
    _15m,
    #[serde(rename = "30m")]
    _30m,
    #[serde(rename = "1h")]
    _1h,
    #[serde(rename = "2h")]
    _2h,
    #[serde(rename = "4h")]
    _4h,
    #[serde(rename = "6h")]
    _6h,
    #[serde(rename = "8h")]
    _8h,
    #[serde(rename = "12h")]
    _12h,
    #[serde(rename = "1d")]
    _1d,
    #[serde(rename = "3d")]
    _3d,
    #[serde(rename = "1w")]
    _1w,
    #[serde(rename = "1M")]
    _1M,
}

#[derive(Debug, Serialize, ApiRequestRequire, ApiRequestToString)]
#[serde(rename_all = "camelCase")]
pub struct AggTradesRequest {
    pub symbol: String,
    pub from_id: Option<u64>,
    pub start_time: Option<u64>,
    pub end_time: Option<u64>,
    pub limit: Option<u16>,
}
impl EndpointRequest for AggTradesRequest {
    type Response = Vec<AggTrade>;
}

#[derive(Debug, Serialize, ApiRequestRequire, ApiRequestToString)]
#[serde(rename_all = "camelCase")]
pub struct AvgPriceRequest {
    pub symbol: String,
}
impl EndpointRequest for AvgPriceRequest {
    type Response = AvgPriceResponse;
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AvgPriceResponse {
    pub mins: u16,
    pub price: String,
    pub close_time: u64,
}
