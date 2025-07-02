use crate::models::*;
use crate::{
    endpoints::{BaseRequest, Endpoint, SecurityType},
    models::BnbBurnStatus,
};
use binance_cex_macros::{APIEndPoint, APIRequestInit, APIRequestToString};

use serde::Serialize;
use serde_qs::to_string;

use super::{EndpointRequest, OneOrMany};

#[derive(Debug, APIEndPoint)]
#[allow(dead_code)]
pub enum MarginEP {
    #[endpoint(GET, UserData, url = "/sapi/v1/bnbBurn")]
    GetBnbBurnStatus,
    #[endpoint(GET, MarketData, url = "/sapi/v1/margin/allPairs")]
    CrossMarginPairs,
    #[endpoint(GET, UserData, url = "/sapi/v1/margin/crossMarginData")]
    CrossMarginFeeData,
    #[endpoint(GET, UserData, url = "/sapi/v1/margin/isolated/allPairs")]
    IsolatedMarginPairs,
    #[endpoint(GET, UserData, url = "/sapi/v1/margin/isolatedMarginData")]
    IsolatedMarginFeeData,
}

#[derive(Debug, Serialize)]
pub struct GetBnbBurnStatusRequest(pub BaseRequest);
impl ToString for GetBnbBurnStatusRequest {
    fn to_string(&self) -> String {
        to_string(self).unwrap()
    }
}
impl EndpointRequest for GetBnbBurnStatusRequest {
    type Response = BnbBurnStatus;
}
// pub type GetBnbBurnStatusResponse = BnbBurnStatus;

#[derive(Debug, Serialize, APIRequestInit, APIRequestToString)]
pub struct CrossMarginPairsRequest {
    pub symbol: Option<String>,
}
impl EndpointRequest for CrossMarginPairsRequest {
    type Response = OneOrMany<MarginPair>;
}

#[derive(Debug, Serialize, APIRequestInit, APIRequestToString)]
#[serde(rename_all = "camelCase")]
pub struct CrossMarginFeeDataRequest {
    pub vip_level: Option<i32>,
    pub coin: Option<String>,

    #[serde(flatten)]
    pub base_request: BaseRequest,
}
impl EndpointRequest for CrossMarginFeeDataRequest {
    type Response = OneOrMany<CrossMarginFee>;
}

#[derive(Debug, Serialize, APIRequestInit, APIRequestToString)]
pub struct IsolatedMarginPairsRequest {
    pub symbol: Option<String>,
}
impl EndpointRequest for IsolatedMarginPairsRequest {
    type Response = OneOrMany<MarginPair>;
}

#[derive(Debug, Serialize, APIRequestInit, APIRequestToString)]
#[serde(rename_all = "camelCase")]
pub struct IsolatedMarginFeeDataRequest {
    pub vip_level: Option<i32>,
    pub symbol: Option<String>,

    #[serde(flatten)]
    pub base_request: BaseRequest,
}
impl EndpointRequest for IsolatedMarginFeeDataRequest {
    type Response = OneOrMany<IsolatedMarginFee>;
}
