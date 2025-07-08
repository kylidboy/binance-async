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
    #[endpoint(GET, UserData, url = "/fapi/v3/balance")]
    Balance,
    #[endpoint(GET, UserData, url = "/fapi/v1/income")]
    IncomeHistory,
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BalanceResponse {
    pub account_alias: String,
    pub asset: String,
    pub balance: String,
    pub cross_wallet_balance: String,
    pub cross_un_pnl: String,
    pub available_balance: String,
    pub max_withdraw_amount: String,
    pub margin_available: bool,
    pub update_time: i64,
}

impl EndpointRequest for BalanceRequest {
    type Response = Vec<BalanceResponse>;
}

#[derive(Debug, Serialize, APIRequestInit, APIRequestToString)]
#[serde(rename_all = "camelCase")]
pub struct IncomeHistoryRequest {
    pub symbol: Option<String>,
    pub income_type: Option<IncomeType>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub page: Option<i32>,
    pub limit: Option<u32>,
    #[serde(flatten)]
    pub base: BaseRequest,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum IncomeType {
    #[default]
    #[serde(rename = "TRANSFER")]
    Transfer,
    #[serde(rename = "WELCOME_BONUS")]
    WelcomeBonus,
    #[serde(rename = "REALIZED_PNL")]
    RealizedPnl,
    #[serde(rename = "FUNDING_FEE")]
    FundingFee,
    #[serde(rename = "COMMISSION")]
    Commission,
    #[serde(rename = "INSURANCE_CLEAR")]
    InsuranceClear,
    #[serde(rename = "REFERRAL_KICKBACK")]
    ReferralKickback,
    #[serde(rename = "COMMISSION_REBATE")]
    CommissionRebate,
    #[serde(rename = "API_REBATE")]
    ApiRebate,
    #[serde(rename = "CONTEST_REWARD")]
    ContestReward,
    #[serde(rename = "CROSS_COLLATERAL_TRANSFER")]
    CrossCollateralTransfer,
    #[serde(rename = "OPTIONS_PREMIUM_FEE")]
    OptionsPremiumFee,
    #[serde(rename = "OPTIONS_SETTLE_PROFIT")]
    OptionsSettleProfit,
    #[serde(rename = "INTERNAL_TRANSFER")]
    InternalTransfer,
    #[serde(rename = "AUTO_EXCHANGE")]
    AutoExchange,
    #[serde(rename = "DELIVERED_SETTELMENT")]
    DeliveredSettelment,
    #[serde(rename = "COIN_SWAP_DEPOSIT")]
    CoinSwapDeposit,
    #[serde(rename = "COIN_SWAP_WITHDRAW")]
    CoinSwapWithdraw,
    #[serde(rename = "POSITION_LIMIT_INCREASE_FEE")]
    PositionLimitIncreaseFee,
    #[serde(rename = "STRATEGY_UMFUTURES_TRANSFER")]
    StrategyUmfuturesTransfer,
    #[serde(rename = "FEE_RETURN")]
    FeeReturn,
    #[serde(rename = "BFUSD_REWARD")]
    BfusdReward,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct IncomeHistory {
    pub symbol: Option<String>,
    pub income_type: IncomeType,
    pub income: String,
    pub asset: String,
    pub info: String,
    pub time: i64,
    pub tran_id: u64,
    pub trade_id: Option<String>,
}

impl EndpointRequest for IncomeHistoryRequest {
    type Response = Vec<IncomeHistory>;
}
