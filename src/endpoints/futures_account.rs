use crate::{
    endpoints::{public_enums::*, BaseRequest, Endpoint, EndpointRequest, SecurityType},
    models::*,
};

use binance_api_macros::{APIEndPoint, APIRequestInit, APIRequestToString};

use serde::{Deserialize, Serialize};

#[derive(Debug, APIEndPoint)]
#[allow(dead_code)]
pub enum AccountInfoEP {
    // #[endpoint(GET, UserData, url = "/fapi/v3/account")]
    // AccountInfoV3,
    #[endpoint(GET, UserData, url = "/fapi/v1/balance")]
    Balance,
}

// #[derive(Debug, Serialize, APIRequestInit, APIRequestToString)]
// #[serde(rename_all = "camelCase")]
// pub struct AccountInfoV3Request(BaseRequest);
// impl EndpointRequest for AccountInfoV3Request {
//     type Response =
// }

#[derive(Debug, Serialize, APIRequestToString)]
#[serde(rename_all = "camelCase")]
pub struct BalanceRequest(pub BaseRequest);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BalanceResponse {
    pub account_alias: String,
    pub asset: String,
    pub balance: String,
    pub cross_wallet_balance: String,
    pub cross_un_pnl: String,
    pub available_balance: String,
    pub max_withdraw_amount: String,
    pub margin_available: String,
    pub update_time: i64,
}

impl EndpointRequest for BalanceRequest {
    type Response = Vec<BalanceResponse>;
}
