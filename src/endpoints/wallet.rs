#![allow(dead_code)]

use crate::endpoints::{Endpoint, SecurityType};
use crate::models::CoinInfo;

use serde::{Deserialize, Serialize};

use super::{BaseRequest, EndpointRequest};

use binance_cex_macros::{APIEndPoint, APIRequestInit, APIRequestToString};

#[derive(Debug, APIEndPoint)]
pub enum WalletEP {
    #[endpoint(GET, None, url = "/sapi/v1/system/status")]
    SystemStatus,
    #[endpoint(GET, UserData, url = "/sapi/v1/capital/config/getall")]
    CapitalConfigGetAll,
    #[endpoint(GET, UserData, url = "/sapi/v1/asset/assetDetail")]
    AssetDetail,
    #[endpoint(GET, UserData, url = "/sapi/v1/capital/deposit/address")]
    DepositAddress,
    #[endpoint(POST, UserData, url = "/sapi/v1/capital/withdraw/apply")]
    WithdrawApply,
    #[endpoint(GET, UserData, url = "/sapi/v1/capital/deposit/hisrec")]
    DepositHisrec,
    #[endpoint(GET, UserData, url = "/sapi/v1/capital/withdraw/history")]
    WithdrawHistory,
}

#[derive(Debug, Serialize, APIRequestToString)]
pub struct SystemStatusRequest;
impl EndpointRequest for SystemStatusRequest {
    type Response = SystemStatusResponse;
}
#[derive(Debug, Deserialize)]
pub struct SystemStatusResponse {
    pub status: i64,
    pub msg: String,
}

#[derive(Debug, Serialize, APIRequestToString)]
pub struct AllCoinsRequest(pub BaseRequest);
impl EndpointRequest for AllCoinsRequest {
    type Response = Vec<CoinInfo>;
}

// impl Wallet {
//     pub async fn system_status(&self) -> Result<()> {
//         let resp = self
//             .client
//             .get::<SystemStatusResponse>(&WalletEP::SystemStatus.to_string(), None)
//             .await?;
//         if resp.status == 0 && resp.msg == "normal" {
//             Ok(())
//         } else {
//             Err(BinanceApiError::WalletMaintenance.into())
//         }
//     }

//     pub async fn get_all_coins(&self) -> Result<Vec<CoinInfo>> {
//         let mut req = BaseRequest::default();
//         req.recv_window = Some(self.recv_window);
//         self.client
//             .get_signed(
//                 &WalletEP::AllCoins.to_string(),
//                 Some(&to_urlencoded_string(&req).unwrap()),
//             )
//             .await
//     }
// }

#[derive(Debug, Serialize, Deserialize, APIRequestInit, APIRequestToString)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawRequest {
    pub coin: String,
    pub address: String,
    pub amount: f64,
    pub withdraw_order_id: Option<String>,
    pub network: Option<String>,
    pub address_tag: Option<String>,
    pub transaction_fee_flag: Option<bool>,
    pub name: Option<String>,
    pub wallet_type: Option<i32>,
    pub recv_window: Option<u64>,
    pub timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawResponse {
    pub id: String,
}

impl EndpointRequest for WithdrawRequest {
    type Response = WithdrawResponse;
}

#[derive(Debug, Serialize, Deserialize, APIRequestInit, APIRequestToString)]
#[serde(rename_all = "camelCase")]
pub struct DepositHisrecRequest {
    pub include_source: Option<bool>,
    pub coin: Option<String>,
    pub status: Option<i64>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub recv_window: Option<i64>,
    pub timestamp: u64,
    pub tx_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DepositHisrecResponse {
    pub id: String,
    pub amount: String,
    pub coin: String,
    pub network: String,
    pub status: i64,
    pub address: String,
    pub address_tag: String,
    pub tx_id: String,
    pub insert_time: i64,
    pub transfer_type: i64,
    pub confirm_times: String,
    pub unlock_confirm: i64,
    pub wallet_type: i64,
}

impl EndpointRequest for DepositHisrecRequest {
    type Response = Vec<DepositHisrecResponse>;
}

#[derive(Debug, Serialize, Deserialize, APIRequestInit, APIRequestToString)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawHistoryRequest {
    pub coin: Option<String>,
    pub withdraw_order_id: Option<String>,
    pub status: Option<i64>,
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub recv_window: Option<i64>,
    pub timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawHistoryResponse {
    pub id: String,
    pub amount: String,
    pub transaction_fee: String,
    pub coin: String,
    pub status: i64,
    pub address: String,
    pub tx_id: String,
    pub apply_time: String,
    pub network: String,
    pub transfer_type: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub withdraw_order_id: Option<String>,
    pub info: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirm_no: Option<i64>,
    pub wallet_type: i64,
    pub tx_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complete_time: Option<String>,
}

impl WithdrawHistoryResponse {
    pub fn is_complete(&self) -> bool {
        self.complete_time.is_some() && self.status == 6
    }
}

impl EndpointRequest for WithdrawHistoryRequest {
    type Response = Vec<WithdrawHistoryResponse>;
}
