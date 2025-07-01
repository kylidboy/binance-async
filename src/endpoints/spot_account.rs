#![allow(dead_code)]

use super::{ApiRequestRequire, Endpoint, EndpointRequest, Method, SecurityType};
use crate::models::*;

use binance_cex_macros::ApiRequestToString;
use serde::Serialize;
use strum_macros::Display;

use super::BaseRequest;

#[derive(Debug, Display)]
pub enum SpotAccountEP {
    #[strum(to_string = "/api/v3/account")]
    Account,
    #[strum(to_string = "/api/v3/myTrades")]
    TradeList,
    #[strum(to_string = "/api/v3/order")]
    QueryOrder,
}

impl Endpoint for SpotAccountEP {
    fn action_params(&self) -> (http::Method, super::SecurityType, String) {
        match self {
            SpotAccountEP::Account => (Method::GET, SecurityType::UserData, self.to_string()),
            SpotAccountEP::TradeList => (Method::GET, SecurityType::UserData, self.to_string()),
            SpotAccountEP::QueryOrder => (Method::GET, SecurityType::UserData, self.to_string()),
        }
    }
}

#[derive(Debug, Serialize, ApiRequestRequire, ApiRequestToString)]
#[serde(rename_all = "camelCase")]
pub struct AccountRequest {
    pub omit_zero_balances: Option<bool>,

    #[serde(flatten)]
    pub base: BaseRequest,
}
impl EndpointRequest for AccountRequest {
    type Response = AccountInformation;
}

#[derive(Debug, Serialize, ApiRequestRequire, ApiRequestToString)]
#[serde(rename_all = "camelCase")]
pub struct TradeListRequest {
    pub symbol: String,
    pub order_id: Option<u64>,
    pub start_time: Option<u64>,
    pub end_time: Option<u64>,
    pub from_id: Option<u64>,
    pub limit: Option<i32>,

    #[serde(flatten)]
    pub base: BaseRequest,
}
impl EndpointRequest for TradeListRequest {
    type Response = Vec<TradeHistory>;
}

#[derive(Debug, Serialize, ApiRequestRequire, ApiRequestToString)]
#[serde(rename_all = "camelCase")]
pub struct QueryOrderRequest {
    pub symbol: String,
    pub order_id: Option<u64>,
    pub orig_client_order_id: Option<String>,
    #[serde(flatten)]
    pub base: BaseRequest,
}
impl EndpointRequest for QueryOrderRequest {
    type Response = Order;
}
