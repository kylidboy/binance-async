#![allow(dead_code)]

use crate::models::*;

use super::EndpointRequest;
use super::{Endpoint, OneOrManySymbol, SecurityType};


use serde::Serialize;

use binance_cex_macros::{APIEndPoint, APIRequestInit, APIRequestToString};

#[derive(Debug, APIEndPoint)]
#[allow(dead_code)]
pub enum SpotGeneralEP {
    #[endpoint(GET, None, url = "/api/v3/ping")]
    Ping,
    #[endpoint(GET, None, url = "/api/v3/time")]
    Time,
    #[endpoint(GET, None, url = "/api/v3/exchangeInfo")]
    ExchangeInfo,
}

#[derive(Debug, Serialize, APIRequestInit, APIRequestToString)]
pub struct ExchangeInfoRequest {
    pub symbols: Option<OneOrManySymbol>,
    pub permissions: Option<AccountAndSymbolPermission>,
}
impl EndpointRequest for ExchangeInfoRequest {
    type Response = ExchangeInformation;
}
