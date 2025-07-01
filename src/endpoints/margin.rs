use crate::models::*;
use crate::{
    endpoints::{ApiRequestRequire, BaseRequest, Endpoint, Method, SecurityType},
    models::BnbBurnStatus,
};

use binance_cex_macros::ApiRequestToString;
use serde::Serialize;
use serde_qs::to_string;
use strum_macros::Display;

use super::{EndpointRequest, OneOrMany};

#[derive(Debug, Display)]
#[allow(dead_code)]
pub enum MarginEP {
    #[strum(to_string = "/sapi/v1/bnbBurn")]
    GetBnbBurnStatus,
    #[strum(to_string = "/sapi/v1/margin/allPairs")]
    CrossMarginPairs,
    #[strum(to_string = "/sapi/v1/margin/crossMarginData")]
    CrossMarginFeeData,
    #[strum(to_string = "/sapi/v1/margin/isolated/allPairs")]
    IsolatedMarginPairs,
    #[strum(to_string = "/sapi/v1/margin/isolatedMarginData")]
    IsolatedMarginFeeData,
}

impl Endpoint for MarginEP {
    fn action_params(&self) -> (Method, SecurityType, String) {
        match self {
            MarginEP::GetBnbBurnStatus => (Method::GET, SecurityType::UserData, self.to_string()),
            MarginEP::CrossMarginPairs => (Method::GET, SecurityType::MarketData, self.to_string()),
            MarginEP::CrossMarginFeeData => (Method::GET, SecurityType::UserData, self.to_string()),
            MarginEP::IsolatedMarginPairs => {
                (Method::GET, SecurityType::UserData, self.to_string())
            }
            MarginEP::IsolatedMarginFeeData => {
                (Method::GET, SecurityType::UserData, self.to_string())
            }
        }
    }
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

#[derive(Debug, Serialize, ApiRequestRequire, ApiRequestToString)]
pub struct CrossMarginPairsRequest {
    pub symbol: Option<String>,
}
impl EndpointRequest for CrossMarginPairsRequest {
    type Response = OneOrMany<MarginPair>;
}

#[derive(Debug, Serialize, ApiRequestRequire, ApiRequestToString)]
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

#[derive(Debug, Serialize, ApiRequestRequire, ApiRequestToString)]
pub struct IsolatedMarginPairsRequest {
    pub symbol: Option<String>,
}
impl EndpointRequest for IsolatedMarginPairsRequest {
    type Response = OneOrMany<MarginPair>;
}

#[derive(Debug, Serialize, ApiRequestRequire, ApiRequestToString)]
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
