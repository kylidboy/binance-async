#![allow(dead_code)]

use crate::models::*;

use super::EndpointRequest;
use super::{Endpoint, OneOrMany, OneOrManySymbol, SecurityType};

use either::Either;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use binance_cex_macros::{APIEndPoint, APIRequestInit, APIRequestToString};


#[derive(Debug, APIEndPoint)]
#[allow(dead_code)]
pub enum SpotMarketEP {
    #[endpoint(GET, None, url = "/api/v3/depth")]
    OrderBook,
    #[endpoint(GET, None, url = "/api/v3/ticker/price")]
    PriceTicker,
    #[endpoint(GET, None, url = "/api/v3/avgPrice")]
    CurrentAvgPrice,
    #[endpoint(GET, None, url = "/api/v3/ticker/bookTicker")]
    SymbolOrderBookTicker,
    #[endpoint(GET, None, url = "/api/v3/ticker/24hr")]
    Ticker24hr,
    #[endpoint(GET, None, url = "/api/v3/aggTrades")]
    AggTrades,
    #[endpoint(GET, None, url = "/api/v3/klines")]
    Klines,
    #[endpoint(GET, None, url = "/api/v3/trades")]
    Trades,
    #[endpoint(GET, None, url = "/api/v3/historicalTrades")]
    HistoricalTrades,
}

#[derive(Debug, Serialize, APIRequestInit, APIRequestToString)]
pub struct OrderBookRequest {
    pub symbol: String,
    pub limit: Option<u64>,
}
impl EndpointRequest for OrderBookRequest {
    type Response = OrderBook;
}

#[derive(Debug, Serialize, APIRequestToString)]
pub struct PriceTickerRequest(pub Option<OneOrManySymbol>);
impl EndpointRequest for PriceTickerRequest {
    type Response = OneOrMany<SymbolPrice>;
}

#[derive(Debug, Serialize, APIRequestInit, APIRequestToString)]
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

#[derive(Debug, Serialize, APIRequestInit, APIRequestToString)]
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

#[derive(Debug, Serialize, APIRequestInit, APIRequestToString)]
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

#[derive(Debug, Serialize, APIRequestInit, APIRequestToString)]
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
