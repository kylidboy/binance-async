#![allow(dead_code)]

use super::{Endpoint, EndpointRequest, SecurityType};
use crate::models::*;

use binance_cex_macros::{APIRequestInit, APIEndPoint, APIRequestToString};
use serde::Serialize;

use super::BaseRequest;

#[derive(Debug, APIEndPoint)]
pub enum SpotAccountEP {
    #[endpoint(GET, UserData, url = "/api/v3/account")]
    Account,
    #[endpoint(GET, UserData, url = "/api/v3/myTrades")]
    TradeList,
    #[endpoint(GET, UserData, url = "/api/v3/order")]
    Order,
    #[endpoint(GET, UserData, url = "/api/v3/openOrders")]
    OpenOrders,
    #[endpoint(GET, UserData, url = "/api/v3/allOrders")]
    AllOrders,
    #[endpoint(GET, UserData, url = "/api/v3/orderList")]
    OrderList,
    #[endpoint(GET, UserData, url = "/api/v3/allOrderList")]
    AllOrderList,
    #[endpoint(GET, UserData, url = "/api/v3/openOrderList")]
    OpenOrderList,

    // deprecated
    // #[endpoint(url = "/api/v3/userDataStream")]
    // UserDataStream,
}

#[derive(Debug, Serialize, APIRequestInit, APIRequestToString)]
#[serde(rename_all = "camelCase")]
pub struct AccountRequest {
    pub omit_zero_balances: Option<bool>,

    #[serde(flatten)]
    pub base: BaseRequest,
}
impl EndpointRequest for AccountRequest {
    type Response = AccountInformation;
}

#[derive(Debug, Serialize, APIRequestInit, APIRequestToString)]
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

#[derive(Debug, Serialize, APIRequestInit, APIRequestToString)]
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
