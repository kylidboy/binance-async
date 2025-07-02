#![allow(dead_code)]

use super::{BaseRequest, Endpoint, EndpointRequest, SecurityType};
use crate::models::CoinFraction;
use serde::Serialize;

use binance_cex_macros::{APIEndPoint, APIRequestToString};

#[derive(Debug, APIEndPoint)]
pub enum ConvertEP {
    #[endpoint(GET, UserData, url = "/sapi/v1/convert/assetInfo")]
    AssetInfo,
}

#[derive(Debug, Serialize, APIRequestToString)]
pub struct AssetInfoRequest(pub BaseRequest);
impl EndpointRequest for AssetInfoRequest {
    type Response = Vec<CoinFraction>;
}
