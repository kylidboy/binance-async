use crate::{
    endpoints::{public_enums::*, BaseRequest, Endpoint, EndpointRequest, SecurityType},
    models::*,
};

use binance_api_macros::{APIEndPoint, APIRequestInit, APIRequestToString};

use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Debug, APIEndPoint)]
#[allow(dead_code)]
pub enum FuturesTradingEP {
    #[endpoint(POST, Trade, url = "/fapi/v1/order")]
    Order,
    #[endpoint(POST, Trade, url = "/fapi/v1/order/test")]
    OrderTest,
    #[endpoint(POST, Trade, url = "/fapi/v1/leverage")]
    Leverage,
    #[endpoint(GET, UserData, url = "/fapi/v3/positionRisk")]
    PositionRiskV3,
}

#[derive(Debug, Serialize, APIRequestInit, APIRequestToString)]
#[serde(rename_all = "camelCase")]
pub struct NewOrderRequest {
    pub symbol: String,
    pub side: OrderSide,
    pub position_side: Option<PositionSide>,
    pub r#type: FutureOrderType,
    pub time_in_force: Option<TimeInForce>,
    pub quantity: Option<f64>,
    pub reduce_only: Option<bool>,
    pub price: Option<f64>,
    pub new_client_order_id: Option<String>,
    pub stop_price: Option<f64>,
    pub close_position: Option<String>,
    pub activation_price: Option<f64>,
    pub callback_rate: Option<f64>,
    pub working_type: Option<WorkingType>,
    pub price_protect: Option<String>,
    pub new_order_resp_type: Option<ResponseType>,
    pub price_match: Option<PriceMatch>,
    pub self_trade_prevention_mode: Option<SelfTradePreventionMode>,
    pub good_till_date: Option<u64>,
    #[serde(flatten)]
    pub base: BaseRequest,
}

// "clientOrderId": "testOrder",
// "cumQty": "0",
// "cumQuote": "0",
// "executedQty": "0",
// "orderId": 22542179,
// "avgPrice": "0.00000",
// "origQty": "10",
// "price": "0",
// "reduceOnly": false,
// "side": "BUY",
// "positionSide": "SHORT",
// "status": "NEW",
// "stopPrice": "9300",		// please ignore when order type is TRAILING_STOP_MARKET
// "closePosition": false,   // if Close-All
// "symbol": "BTCUSDT",
// "timeInForce": "GTD",
// "type": "TRAILING_STOP_MARKET",
// "origType": "TRAILING_STOP_MARKET",
// "activatePrice": "9020",	// activation price, only return with TRAILING_STOP_MARKET order
// "priceRate": "0.3",			// callback rate, only return with TRAILING_STOP_MARKET order
// "updateTime": 1566818724722,
// "workingType": "CONTRACT_PRICE",
// "priceProtect": false,      // if conditional order trigger is protected
// "priceMatch": "NONE",              //price match mode
// "selfTradePreventionMode": "NONE", //self trading preventation mode
// "goodTillDate": 1693207680000      //order pre-set auot cancel time for TIF GTD order
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewOrderResponse {
    pub client_order_id: String,
    pub cum_qty: String,
    pub cum_quote: String,
    pub executed_qty: String,
    pub order_id: u64,
    pub avg_price: String,
    pub orig_qty: String,
    pub price: String,
    pub reduce_only: bool,
    pub side: String,
    pub position_side: String,
    pub status: String,
    pub stop_price: String,
    pub close_position: bool,
    pub symbol: String,
    pub time_in_force: String,
    pub r#type: String,
    pub orig_type: String,
    pub activate_price: Option<String>,
    pub price_rate: Option<String>,
    pub update_time: u64,
    pub working_type: String,
    pub price_protect: bool,
    pub price_match: String,
    pub self_trade_prevention_mode: String,
    pub good_till_date: u64,
}
/*
{"orderId":4052525244,"symbol":"SUIUSDC","status":"NEW","clientOrderId":"L4tpAzg0l78K5gp20HCAWK","price":"0.000000","avgPrice":"0.00","origQty":"3.0","executedQty":"0.0","cumQty":"0.0","cumQuote":"0.0000000","timeInForce":"GTC","type":"MARKET","reduceOnly":false,"closePosition":false,"side":"SELL","positionSide":"BOTH","stopPrice":"0.000000","workingType":"CONTRACT_PRICE","priceProtect":false,"origType":"MARKET","priceMatch":"NONE","selfTradePreventionMode":"EXPIRE_MAKER","goodTillDate":0,"updateTime":1751627587841}
*/

impl EndpointRequest for NewOrderRequest {
    type Response = NewOrderResponse;

    fn validate(&self) -> anyhow::Result<()> {
        match &self.r#type {
            t @ FutureOrderType::Limit => {
                if self.quantity.is_none() || self.time_in_force.is_none() || self.price.is_none() {
                    anyhow::bail!("order type {t} requirements")
                }
            }
            t @ FutureOrderType::Market => {
                if self.quantity.is_none() {
                    anyhow::bail!("order type {t} requirements")
                }
            }
            t @ FutureOrderType::Stop | t @ FutureOrderType::TakeProfit => {
                if self.quantity.is_none() || self.stop_price.is_none() || self.price.is_none() {
                    anyhow::bail!("order type {t} requirements")
                }
            }
            t @ FutureOrderType::StopMarket | t @ FutureOrderType::TakeProfitMarket => {
                if self.stop_price.is_none() {
                    anyhow::bail!("order type {t} requirements")
                }
            }
            t @ FutureOrderType::TrailingStopMarket => {
                if self.callback_rate.is_none() {
                    anyhow::bail!("order type {t} requirements")
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "UPPERCASE")]
pub enum PositionSide {
    Both,
    Long,
    Short,
}

/*
 *
Type	Additional mandatory parameters
LIMIT	timeInForce, quantity, price
MARKET	quantity
STOP/TAKE_PROFIT	quantity, price, stopPrice
STOP_MARKET/TAKE_PROFIT_MARKET	stopPrice
TRAILING_STOP_MARKET	callbackRate
 */
#[derive(Debug, Display, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "UPPERCASE")]
pub enum FutureOrderType {
    Limit,
    Market,
    Stop,
    #[serde(rename = "TAKE_PROFIT")]
    TakeProfit,
    #[serde(rename = "STOP_MARKET")]
    StopMarket,
    #[serde(rename = "TAKE_PROFIT_MARKET")]
    TakeProfitMarket,
    #[serde(rename = "TRAILING_STOP_MARKET")]
    TrailingStopMarket,
}

#[allow(clippy::all)]
#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum TimeInForce {
    Gtc,
    Ioc,
    Fok,
    Gtd,
    Gtx,
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub enum WorkingType {
    #[serde(rename = "MARK_PRICE")]
    MarkPrice,
    #[serde(rename = "CONTRACT_PRICE")]
    ContractPrice,
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum ResponseType {
    Ack,
    Result,
}

/*
 * Price Match (priceMatch)

NONE (No price match)
OPPONENT (counterparty best price)
OPPONENT_5 (the 5th best price from the counterparty)
OPPONENT_10 (the 10th best price from the counterparty)
OPPONENT_20 (the 20th best price from the counterparty)
QUEUE (the best price on the same side of the order book)
QUEUE_5 (the 5th best price on the same side of the order book)
QUEUE_10 (the 10th best price on the same side of the order book)
QUEUE_20 (the 20th best price on the same side of the order book)
 */

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "UPPERCASE")]
pub enum PriceMatch {
    None,
    Opponent,
    #[serde(rename = "OPPONENT_5")]
    Opponent5,
    #[serde(rename = "OPPONENT_10")]
    Opponent10,
    #[serde(rename = "OPPONENT_20")]
    Opponent20,
    Queue,
    #[serde(rename = "QUEUE_5")]
    Queue5,
    #[serde(rename = "QUEUE_10")]
    Queue10,
    #[serde(rename = "QUEUE_20")]
    Queue20,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum SelfTradePreventionMode {
    #[serde(rename = "EXPIRE_TAKER")]
    ExpireTaker,
    #[serde(rename = "EXPIRE_BOTH")]
    ExpireBoth,
    #[serde(rename = "EXPIRE_MAKER")]
    ExpireMaker,
}

#[derive(Debug, Serialize, APIRequestInit, APIRequestToString)]
#[serde(rename_all = "camelCase")]
pub struct LeverageRequest {
    pub symbol: String,
    pub leverage: i32,
    #[serde(flatten)]
    pub base: BaseRequest,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeverageResponse {
    pub leverage: i32,
    pub max_notional_value: String,
    pub symbol: String,
}

impl EndpointRequest for LeverageRequest {
    type Response = LeverageResponse;
}


#[derive(Debug, Serialize, APIRequestInit, APIRequestToString)]
#[serde(rename_all = "camelCase")]
pub struct PositionRiskV3Request {
    pub symbol: Option<String>,
    #[serde(flatten)]
    pub base: BaseRequest,
}

impl EndpointRequest for PositionRiskV3Request {
    type Response = Vec<PositionRiskV3>;
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PositionRiskV3 {
    pub symbol: String,
    pub position_side: String,
    pub position_amt: String,
    pub entry_price: String,
    pub break_even_price: String,
    pub mark_price: String,
    pub un_realized_profit: String,
    pub liquidation_price: String,
    pub isolated_margin: String,
    pub notional: String,
    pub margin_asset: String,
    pub isolated_wallet: String,
    pub initial_margin: String,
    pub maint_margin: String,
    pub position_initial_margin: String,
    pub open_order_initial_margin: String,
    pub adl: i32,
    pub bid_notional: String,
    pub ask_notional: String,
    pub update_time: u64,
}

/*
 * "symbol": "ADAUSDT",
        "positionSide": "BOTH",               // 持仓方向
        "positionAmt": "30",
        "entryPrice": "0.385",
        "breakEvenPrice": "0.385077",
        "markPrice": "0.41047590",
        "unRealizedProfit": "0.76427700",     // 持仓未实现盈亏
        "liquidationPrice": "0",
        "isolatedMargin": "0",
        "notional": "12.31427700",
        "marginAsset": "USDT",
        "isolatedWallet": "0",
        "initialMargin": "0.61571385",        // 初始保证金
        "maintMargin": "0.08004280",          // 维持保证金
        "positionInitialMargin": "0.61571385",// 仓位初始保证金
        "openOrderInitialMargin": "0",        // 订单初始保证金
        "adl": 2,
        "bidNotional": "0",
        "askNotional": "0",
        "updateTime": 1720736417660           // 更新时间
 */
