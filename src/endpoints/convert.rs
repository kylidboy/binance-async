#![allow(dead_code)]

use super::{BaseRequest, Endpoint, EndpointRequest, Method, SecurityType};
use crate::models::CoinFraction;
use serde::Serialize;

use binance_cex_macros::ApiRequestToString;
use strum_macros::Display;

#[derive(Debug, Display)]
pub enum ConvertEP {
    #[strum(to_string = "/sapi/v1/convert/assetInfo")]
    AssetInfo,
}

impl Endpoint for ConvertEP {
    fn action_params(&self) -> (Method, SecurityType, String) {
        match self {
            ConvertEP::AssetInfo => (Method::GET, SecurityType::None, self.to_string()),
        }
    }
}

#[derive(Debug, Serialize, ApiRequestToString)]
pub struct AssetInfoRequest(pub BaseRequest);
impl EndpointRequest for AssetInfoRequest {
    type Response = Vec<CoinFraction>;
}
