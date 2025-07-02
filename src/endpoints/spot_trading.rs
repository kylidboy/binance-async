use crate::endpoints::{
    ApiRequestRequire, ApiRequestToString, Endpoint, EndpointRequest, Method, ResponseType, SecurityType,
};
use crate::models::*;

use serde::{Deserialize, Serialize};
use strum_macros::Display;

#[derive(Debug, Display)]
#[allow(dead_code)]
pub enum SpotTradingEP {
    #[strum(to_string = "/api/v3/order")]
    Order,
    #[strum(to_string = "/api/v3/order/test")]
    OrderTest,
    #[strum(to_string = "/api/v3/openOrders")]
    OpenOrders,
    #[strum(to_string = "/api/v3/allOrders")]
    AllOrders,
    #[strum(to_string = "/api/v3/order/oco")]
    Oco,
    #[strum(to_string = "/api/v3/orderList")]
    OrderList,
    #[strum(to_string = "/api/v3/allOrderList")]
    AllOrderList,
    #[strum(to_string = "/api/v3/openOrderList")]
    OpenOrderList,
    #[strum(to_string = "/api/v3/userDataStream")]
    UserDataStream,
}
impl Endpoint for SpotTradingEP {
    fn action_params(&self) -> (Method, SecurityType, String) {
        match self {
            SpotTradingEP::Order => (Method::POST, SecurityType::Trade, self.to_string()),
            SpotTradingEP::OrderTest => todo!(),
            SpotTradingEP::OpenOrders => todo!(),
            SpotTradingEP::AllOrders => todo!(),
            SpotTradingEP::Oco => todo!(),
            SpotTradingEP::OrderList => todo!(),
            SpotTradingEP::AllOrderList => todo!(),
            SpotTradingEP::OpenOrderList => todo!(),
            SpotTradingEP::UserDataStream => todo!(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SelfTradePreventionMode {
    #[serde(rename = "EXPIRE_TAKER")]
    ExpireTaker,
    #[serde(rename = "EXPIRE_MAKER")]
    ExpireMaker,
    #[serde(rename = "EXPIRE_BOTH")]
    ExpireBoth,
    #[serde(rename = "NONE")]
    None,
}

#[derive(Debug, Serialize, Deserialize, ApiRequestRequire, ApiRequestToString)]
#[serde(rename_all = "camelCase")]
pub struct NewOrderRequest {
    pub symbol: String,
    pub side: OrderSide,
    pub r#type: OrderType,
    pub quantity: Option<f64>,
    pub quote_order_qty: Option<f64>,
    pub price: Option<f64>,
    pub stop_price: Option<f64>,
    pub time_in_force: Option<TimeInForce>,
    pub new_client_order_id: Option<String>,
    pub strategy_id: Option<i32>,
    pub strategy_type: Option<i32>,
    pub trailing_delta: Option<i64>,
    pub iceberg_qty: Option<f64>,
    pub new_order_resp_type: Option<ResponseType>,
    pub self_trade_prevention_mode: Option<SelfTradePreventionMode>,
    pub recv_window: Option<u64>,
    pub timestamp: u64,
}
impl EndpointRequest for NewOrderRequest {
    type Response = Transaction;
}

#[derive(Debug, Serialize, Deserialize)]
pub enum OrderType {
    #[serde(rename = "LIMIT")]
    Limit,
    #[serde(rename = "MARKET")]
    Market,
    #[serde(rename = "STOP_LOSS")]
    StopLoss,
    #[serde(rename = "STOP_LOSS_LIMIT")]
    StopLossLimit,
    #[serde(rename = "TAKE_PROFIT")]
    TakeProfit,
    #[serde(rename = "TAKE_PROFIT_LIMIT")]
    TakeProfitLimit,
    #[serde(rename = "LIMIT_MAKER")]
    LimitMaker,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum OrderSide {
    Buy,
    Sell,
}

#[allow(clippy::all)]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum TimeInForce {
    Gtc,
    Ioc,
    Fok,
}

// impl SpotTrading {
//     // Current open orders for ONE symbol
//     pub async fn get_open_orders<S>(&self, symbol: S) -> Result<Vec<Order>>
//     where
//         S: Into<String>,
//     {
//         let mut parameters: BTreeMap<String, String> = BTreeMap::new();
//         parameters.insert("symbol".into(), symbol.into());

//         let request = utils::build_signed_request(Some(parameters), self.recv_window);
//         self.client
//             .get_signed(
//                 Endpoint::SpotTrading(SpotTradingEndpoints::OpenOrders),
//                 Some(request),
//             )
//             .await
//     }

//     // All current open orders
//     pub async fn get_all_open_orders(&self) -> Result<Vec<Order>> {
//         let request = utils::build_signed_request(None, self.recv_window);
//         self.client
//             .get_signed(
//                 Endpoint::SpotTrading(SpotTradingEndpoints::OpenOrders),
//                 Some(request),
//             )
//             .await
//     }

//     // Cancel all open orders for a single symbol
//     pub async fn cancel_all_open_orders<S>(&self, symbol: S) -> Result<Vec<OrderCanceled>>
//     where
//         S: Into<String>,
//     {
//         let mut parameters: BTreeMap<String, String> = BTreeMap::new();
//         parameters.insert("symbol".into(), symbol.into());
//         let request = utils::build_signed_request(Some(parameters), self.recv_window);
//         self.client
//             .delete_signed(
//                 Endpoint::SpotTrading(SpotTradingEndpoints::OpenOrders),
//                 Some(request),
//             )
//             .await
//     }

//     // Check an order's status
//     pub async fn order_status<S>(&self, symbol: S, order_id: u64) -> Result<Order>
//     where
//         S: Into<String>,
//     {
//         let mut parameters: BTreeMap<String, String> = BTreeMap::new();
//         parameters.insert("symbol".into(), symbol.into());
//         parameters.insert("orderId".into(), order_id.to_string());

//         let request = utils::build_signed_request(Some(parameters), self.recv_window);
//         self.client
//             .get_signed(
//                 Endpoint::SpotTrading(SpotTradingEndpoints::Order),
//                 Some(request),
//             )
//             .await
//     }

//     /// Place a test status order
//     ///
//     /// This order is sandboxed: it is validated, but not sent to the matching engine.
//     pub async fn test_order_status<S>(&self, symbol: S, order_id: u64) -> Result<()>
//     where
//         S: Into<String>,
//     {
//         let mut parameters: BTreeMap<String, String> = BTreeMap::new();
//         parameters.insert("symbol".into(), symbol.into());
//         parameters.insert("orderId".into(), order_id.to_string());

//         let request = utils::build_signed_request(Some(parameters), self.recv_window);
//         self.client
//             .get_signed::<Empty>(
//                 Endpoint::SpotTrading(SpotTradingEndpoints::OrderTest),
//                 Some(request),
//             )
//             .await
//             .map(|_| ())
//     }

//     // Place a LIMIT order - BUY
//     pub async fn limit_buy<S, F>(&self, symbol: S, qty: F, price: f64) -> Result<Transaction>
//     where
//         S: Into<String>,
//         F: Into<f64>,
//     {
//         let buy = OrderRequest {
//             symbol: symbol.into(),
//             qty: qty.into(),
//             price,
//             stop_price: None,
//             order_side: OrderSide::Buy,
//             order_type: OrderType::Limit,
//             time_in_force: TimeInForce::GTC,
//             new_client_order_id: None,
//         };
//         let request = utils::build_signed_request(Some(buy.into()), self.recv_window);
//         self.client
//             .post_signed(Endpoint::SpotTrading(SpotTradingEndpoints::Order), request)
//             .await
//     }

//     /// Place a test limit order - BUY
//     ///
//     /// This order is sandboxed: it is validated, but not sent to the matching engine.
//     pub async fn test_limit_buy<S, F>(&self, symbol: S, qty: F, price: f64) -> Result<()>
//     where
//         S: Into<String>,
//         F: Into<f64>,
//     {
//         let buy = OrderRequest {
//             symbol: symbol.into(),
//             qty: qty.into(),
//             price,
//             stop_price: None,
//             order_side: OrderSide::Buy,
//             order_type: OrderType::Limit,
//             time_in_force: TimeInForce::GTC,
//             new_client_order_id: None,
//         };
//         let request = utils::build_signed_request(Some(buy.into()), self.recv_window);
//         self.client
//             .post_signed::<Empty>(Endpoint::SpotTrading(SpotTradingEndpoints::OrderTest), request)
//             .await
//             .map(|_| ())
//     }

//     // Place a LIMIT order - SELL
//     pub async fn limit_sell<S, F>(&self, symbol: S, qty: F, price: f64) -> Result<Transaction>
//     where
//         S: Into<String>,
//         F: Into<f64>,
//     {
//         let sell = OrderRequest {
//             symbol: symbol.into(),
//             qty: qty.into(),
//             price,
//             stop_price: None,
//             order_side: OrderSide::Sell,
//             order_type: OrderType::Limit,
//             time_in_force: TimeInForce::GTC,
//             new_client_order_id: None,
//         };
//         let request = utils::build_signed_request(Some(sell.into()), self.recv_window);
//         self.client
//             .post_signed(Endpoint::SpotTrading(SpotTradingEndpoints::Order), request)
//             .await
//     }

//     /// Place a test LIMIT order - SELL
//     ///
//     /// This order is sandboxed: it is validated, but not sent to the matching engine.
//     pub async fn test_limit_sell<S, F>(&self, symbol: S, qty: F, price: f64) -> Result<()>
//     where
//         S: Into<String>,
//         F: Into<f64>,
//     {
//         let sell = OrderRequest {
//             symbol: symbol.into(),
//             qty: qty.into(),
//             price,
//             stop_price: None,
//             order_side: OrderSide::Sell,
//             order_type: OrderType::Limit,
//             time_in_force: TimeInForce::GTC,
//             new_client_order_id: None,
//         };
//         let request = utils::build_signed_request(Some(sell.into()), self.recv_window);
//         self.client
//             .post_signed::<Empty>(Endpoint::SpotTrading(SpotTradingEndpoints::OrderTest), request)
//             .await
//             .map(|_| ())
//     }

//     // Place a MARKET order - BUY
//     pub async fn market_buy<S, F>(&self, symbol: S, qty: F) -> Result<Transaction>
//     where
//         S: Into<String>,
//         F: Into<f64>,
//     {
//         let buy = OrderRequest {
//             symbol: symbol.into(),
//             qty: qty.into(),
//             price: 0.0,
//             stop_price: None,
//             order_side: OrderSide::Buy,
//             order_type: OrderType::Market,
//             time_in_force: TimeInForce::GTC,
//             new_client_order_id: None,
//         };
//         let request = utils::build_signed_request(Some(buy.into()), self.recv_window);
//         self.client
//             .post_signed(Endpoint::SpotTrading(SpotTradingEndpoints::Order), request)
//             .await
//     }

//     /// Place a test MARKET order - BUY
//     ///
//     /// This order is sandboxed: it is validated, but not sent to the matching engine.
//     pub async fn test_market_buy<S, F>(&self, symbol: S, qty: F) -> Result<()>
//     where
//         S: Into<String>,
//         F: Into<f64>,
//     {
//         let buy = OrderRequest {
//             symbol: symbol.into(),
//             qty: qty.into(),
//             price: 0.0,
//             stop_price: None,
//             order_side: OrderSide::Buy,
//             order_type: OrderType::Market,
//             time_in_force: TimeInForce::GTC,
//             new_client_order_id: None,
//         };
//         let request = utils::build_signed_request(Some(buy.into()), self.recv_window);
//         self.client
//             .post_signed::<Empty>(Endpoint::SpotTrading(SpotTradingEndpoints::OrderTest), request)
//             .await
//             .map(|_| ())
//     }

//     // Place a MARKET order with quote quantity - BUY
//     pub async fn market_buy_using_quote_quantity<S, F>(
//         &self,
//         symbol: S,
//         quote_order_qty: F,
//     ) -> Result<Transaction>
//     where
//         S: Into<String>,
//         F: Into<f64>,
//     {
//         let buy = OrderQuoteQuantityRequest {
//             symbol: symbol.into(),
//             quote_order_qty: quote_order_qty.into(),
//             price: 0.0,
//             order_side: OrderSide::Buy,
//             order_type: OrderType::Market,
//             time_in_force: TimeInForce::GTC,
//             new_client_order_id: None,
//         };
//         let request = utils::build_signed_request(Some(buy.into()), self.recv_window);
//         self.client
//             .post_signed(Endpoint::SpotTrading(SpotTradingEndpoints::Order), request)
//             .await
//     }

//     /// Place a test MARKET order with quote quantity - BUY
//     ///
//     /// This order is sandboxed: it is validated, but not sent to the matching engine.
//     pub async fn test_market_buy_using_quote_quantity<S, F>(
//         &self,
//         symbol: S,
//         quote_order_qty: F,
//     ) -> Result<()>
//     where
//         S: Into<String>,
//         F: Into<f64>,
//     {
//         let buy = OrderQuoteQuantityRequest {
//             symbol: symbol.into(),
//             quote_order_qty: quote_order_qty.into(),
//             price: 0.0,
//             order_side: OrderSide::Buy,
//             order_type: OrderType::Market,
//             time_in_force: TimeInForce::GTC,
//             new_client_order_id: None,
//         };
//         let request = utils::build_signed_request(Some(buy.into()), self.recv_window);
//         self.client
//             .post_signed::<Empty>(Endpoint::SpotTrading(SpotTradingEndpoints::OrderTest), request)
//             .await
//             .map(|_| ())
//     }

//     // Place a MARKET order - SELL
//     pub async fn market_sell<S, F>(&self, symbol: S, qty: F) -> Result<Transaction>
//     where
//         S: Into<String>,
//         F: Into<f64>,
//     {
//         let sell = OrderRequest {
//             symbol: symbol.into(),
//             qty: qty.into(),
//             price: 0.0,
//             stop_price: None,
//             order_side: OrderSide::Sell,
//             order_type: OrderType::Market,
//             time_in_force: TimeInForce::GTC,
//             new_client_order_id: None,
//         };
//         let request = utils::build_signed_request(Some(sell.into()), self.recv_window);
//         self.client
//             .post_signed(Endpoint::SpotTrading(SpotTradingEndpoints::Order), request)
//             .await
//     }

//     /// Place a test MARKET order - SELL
//     ///
//     /// This order is sandboxed: it is validated, but not sent to the matching engine.
//     pub async fn test_market_sell<S, F>(&self, symbol: S, qty: F) -> Result<()>
//     where
//         S: Into<String>,
//         F: Into<f64>,
//     {
//         let sell = OrderRequest {
//             symbol: symbol.into(),
//             qty: qty.into(),
//             price: 0.0,
//             stop_price: None,
//             order_side: OrderSide::Sell,
//             order_type: OrderType::Market,
//             time_in_force: TimeInForce::GTC,
//             new_client_order_id: None,
//         };
//         let request = utils::build_signed_request(Some(sell.into()), self.recv_window);
//         self.client
//             .post_signed::<Empty>(Endpoint::SpotTrading(SpotTradingEndpoints::OrderTest), request)
//             .await
//             .map(|_| ())
//     }

//     // Place a MARKET order with quote quantity - SELL
//     pub async fn market_sell_using_quote_quantity<S, F>(
//         &self,
//         symbol: S,
//         quote_order_qty: F,
//     ) -> Result<Transaction>
//     where
//         S: Into<String>,
//         F: Into<f64>,
//     {
//         let sell = OrderQuoteQuantityRequest {
//             symbol: symbol.into(),
//             quote_order_qty: quote_order_qty.into(),
//             price: 0.0,
//             order_side: OrderSide::Sell,
//             order_type: OrderType::Market,
//             time_in_force: TimeInForce::GTC,
//             new_client_order_id: None,
//         };
//         let request = utils::build_signed_request(Some(sell.into()), self.recv_window);
//         self.client
//             .post_signed(Endpoint::SpotTrading(SpotTradingEndpoints::Order), request)
//             .await
//     }

//     /// Place a test MARKET order with quote quantity - SELL
//     ///
//     /// This order is sandboxed: it is validated, but not sent to the matching engine.
//     pub async fn test_market_sell_using_quote_quantity<S, F>(
//         &self,
//         symbol: S,
//         quote_order_qty: F,
//     ) -> Result<()>
//     where
//         S: Into<String>,
//         F: Into<f64>,
//     {
//         let sell = OrderQuoteQuantityRequest {
//             symbol: symbol.into(),
//             quote_order_qty: quote_order_qty.into(),
//             price: 0.0,
//             order_side: OrderSide::Sell,
//             order_type: OrderType::Market,
//             time_in_force: TimeInForce::GTC,
//             new_client_order_id: None,
//         };
//         let request = utils::build_signed_request(Some(sell.into()), self.recv_window);
//         self.client
//             .post_signed::<Empty>(Endpoint::SpotTrading(SpotTradingEndpoints::OrderTest), request)
//             .await
//             .map(|_| ())
//     }

//     /// Create a stop limit buy order for the given symbol, price and stop price.
//     /// Returning a `Transaction` value with the same parameters sent on the order.
//     ///
//     ///```no_run
//     /// use binance::api::Binance;
//     /// use binance::account::*;
//     ///
//     /// fn main() {
//     ///     let api_key = Some("api_key".into());
//     ///     let secret_key = Some("secret_key".into());
//     ///     let account: Account = Binance::new(api_key, secret_key);
//     ///     let result = account.stop_limit_buy_order("LTCBTC", 1, 0.1, 0.09, TimeInForce::GTC);
//     /// }
//     /// ```
//     pub async fn stop_limit_buy_order<S, F>(
//         &self,
//         symbol: S,
//         qty: F,
//         price: f64,
//         stop_price: f64,
//         time_in_force: TimeInForce,
//     ) -> Result<Transaction>
//     where
//         S: Into<String>,
//         F: Into<f64>,
//     {
//         let sell = OrderRequest {
//             symbol: symbol.into(),
//             qty: qty.into(),
//             price,
//             stop_price: Some(stop_price),
//             order_side: OrderSide::Buy,
//             order_type: OrderType::StopLossLimit,
//             time_in_force,
//             new_client_order_id: None,
//         };
//         let request = utils::build_signed_request(Some(sell.into()), self.recv_window);
//         self.client
//             .post_signed(Endpoint::SpotTrading(SpotTradingEndpoints::Order), request)
//             .await
//     }

//     /// Create a stop limit buy test order for the given symbol, price and stop price.
//     /// Returning a `Transaction` value with the same parameters sent on the order.
//     ///
//     /// This order is sandboxed: it is validated, but not sent to the matching engine.
//     ///
//     ///```no_run
//     /// use binance::api::Binance;
//     /// use binance::account::*;
//     ///
//     /// fn main() {
//     ///     let api_key = Some("api_key".into());
//     ///     let secret_key = Some("secret_key".into());
//     ///     let account: Account = Binance::new(api_key, secret_key);
//     ///     let result = account.test_stop_limit_buy_order("LTCBTC", 1, 0.1, 0.09, TimeInForce::GTC);
//     /// }
//     /// ```
//     pub async fn test_stop_limit_buy_order<S, F>(
//         &self,
//         symbol: S,
//         qty: F,
//         price: f64,
//         stop_price: f64,
//         time_in_force: TimeInForce,
//     ) -> Result<()>
//     where
//         S: Into<String>,
//         F: Into<f64>,
//     {
//         let sell = OrderRequest {
//             symbol: symbol.into(),
//             qty: qty.into(),
//             price,
//             stop_price: Some(stop_price),
//             order_side: OrderSide::Buy,
//             order_type: OrderType::StopLossLimit,
//             time_in_force,
//             new_client_order_id: None,
//         };
//         let request = utils::build_signed_request(Some(sell.into()), self.recv_window);
//         self.client
//             .post_signed::<Empty>(Endpoint::SpotTrading(SpotTradingEndpoints::OrderTest), request)
//             .await
//             .map(|_| ())
//     }

//     /// Create a stop limit sell order for the given symbol, price and stop price.
//     /// Returning a `Transaction` value with the same parameters sent on the order.
//     ///
//     ///```no_run
//     /// use binance::api::Binance;
//     /// use binance::account::*;
//     ///
//     /// fn main() {
//     ///     let api_key = Some("api_key".into());
//     ///     let secret_key = Some("secret_key".into());
//     ///     let account: Account = Binance::new(api_key, secret_key);
//     ///     let result = account.stop_limit_sell_order("LTCBTC", 1, 0.1, 0.09, TimeInForce::GTC);
//     /// }
//     /// ```
//     pub async fn stop_limit_sell_order<S, F>(
//         &self,
//         symbol: S,
//         qty: F,
//         price: f64,
//         stop_price: f64,
//         time_in_force: TimeInForce,
//     ) -> Result<Transaction>
//     where
//         S: Into<String>,
//         F: Into<f64>,
//     {
//         let sell = OrderRequest {
//             symbol: symbol.into(),
//             qty: qty.into(),
//             price,
//             stop_price: Some(stop_price),
//             order_side: OrderSide::Sell,
//             order_type: OrderType::StopLossLimit,
//             time_in_force,
//             new_client_order_id: None,
//         };
//         let request = utils::build_signed_request(Some(sell.into()), self.recv_window);
//         self.client
//             .post_signed(Endpoint::SpotTrading(SpotTradingEndpoints::Order), request)
//             .await
//     }

//     /// Create a stop limit sell order for the given symbol, price and stop price.
//     /// Returning a `Transaction` value with the same parameters sent on the order.
//     ///
//     /// This order is sandboxed: it is validated, but not sent to the matching engine.
//     ///
//     ///```no_run
//     /// use binance::api::Binance;
//     /// use binance::account::*;
//     ///
//     /// fn main() {
//     ///     let api_key = Some("api_key".into());
//     ///     let secret_key = Some("secret_key".into());
//     ///     let account: Account = Binance::new(api_key, secret_key);
//     ///     let result = account.test_stop_limit_sell_order("LTCBTC", 1, 0.1, 0.09, TimeInForce::GTC);
//     /// }
//     /// ```
//     pub async fn test_stop_limit_sell_order<S, F>(
//         &self,
//         symbol: S,
//         qty: F,
//         price: f64,
//         stop_price: f64,
//         time_in_force: TimeInForce,
//     ) -> Result<()>
//     where
//         S: Into<String>,
//         F: Into<f64>,
//     {
//         let sell = OrderRequest {
//             symbol: symbol.into(),
//             qty: qty.into(),
//             price,
//             stop_price: Some(stop_price),
//             order_side: OrderSide::Sell,
//             order_type: OrderType::StopLossLimit,
//             time_in_force,
//             new_client_order_id: None,
//         };
//         let request = utils::build_signed_request(Some(sell.into()), self.recv_window);
//         self.client
//             .post_signed::<Empty>(Endpoint::SpotTrading(SpotTradingEndpoints::OrderTest), request)
//             .await
//             .map(|_| ())
//     }

//     /// Place a custom order
//     #[allow(clippy::too_many_arguments)]
//     pub async fn custom_order<S, F>(
//         &self,
//         symbol: S,
//         qty: F,
//         price: f64,
//         stop_price: Option<f64>,
//         order_side: OrderSide,
//         order_type: OrderType,
//         time_in_force: TimeInForce,
//         new_client_order_id: Option<String>,
//     ) -> Result<Transaction>
//     where
//         S: Into<String>,
//         F: Into<f64>,
//     {
//         let sell = OrderRequest {
//             symbol: symbol.into(),
//             qty: qty.into(),
//             price,
//             stop_price,
//             order_side,
//             order_type,
//             time_in_force,
//             new_client_order_id,
//         };
//         let request = utils::build_signed_request(Some(sell.into()), self.recv_window);
//         self.client
//             .post_signed(Endpoint::SpotTrading(SpotTradingEndpoints::Order), request)
//             .await
//     }

//     /// Place a test custom order
//     ///
//     /// This order is sandboxed: it is validated, but not sent to the matching engine.
//     #[allow(clippy::too_many_arguments)]
//     pub async fn test_custom_order<S, F>(
//         &self,
//         symbol: S,
//         qty: F,
//         price: f64,
//         stop_price: Option<f64>,
//         order_side: OrderSide,
//         order_type: OrderType,
//         time_in_force: TimeInForce,
//         new_client_order_id: Option<String>,
//     ) -> Result<()>
//     where
//         S: Into<String>,
//         F: Into<f64>,
//     {
//         let sell = OrderRequest {
//             symbol: symbol.into(),
//             qty: qty.into(),
//             price,
//             stop_price,
//             order_side,
//             order_type,
//             time_in_force,
//             new_client_order_id,
//         };
//         let request = utils::build_signed_request(Some(sell.into()), self.recv_window);
//         self.client
//             .post_signed::<Empty>(Endpoint::SpotTrading(SpotTradingEndpoints::OrderTest), request)
//             .await
//             .map(|_| ())
//     }

//     // Check an order's status
//     pub async fn cancel_order<S>(&self, symbol: S, order_id: u64) -> Result<OrderCanceled>
//     where
//         S: Into<String>,
//     {
//         let mut parameters: BTreeMap<String, String> = BTreeMap::new();
//         parameters.insert("symbol".into(), symbol.into());
//         parameters.insert("orderId".into(), order_id.to_string());

//         let request = utils::build_signed_request(Some(parameters), self.recv_window);
//         self.client
//             .delete_signed(Endpoint::SpotTrading(SpotTradingEndpoints::Order), Some(request))
//             .await
//     }

//     pub async fn cancel_order_with_client_id<S>(
//         &self,
//         symbol: S,
//         orig_client_order_id: String,
//     ) -> Result<OrderCanceled>
//     where
//         S: Into<String>,
//     {
//         let mut parameters: BTreeMap<String, String> = BTreeMap::new();
//         parameters.insert("symbol".into(), symbol.into());
//         parameters.insert("origClientOrderId".into(), orig_client_order_id);

//         let request = utils::build_signed_request(Some(parameters), self.recv_window);
//         self.client
//             .delete_signed(Endpoint::SpotTrading(SpotTradingEndpoints::Order), Some(request))
//             .await
//     }

//     pub fn cancel_order_with_client_id_rs<S>() {}

//     /// Place a test cancel order
//     ///
//     /// This order is sandboxed: it is validated, but not sent to the matching engine.
//     pub async fn test_cancel_order<S>(&self, symbol: S, order_id: u64) -> Result<()>
//     where
//         S: Into<String>,
//     {
//         let mut parameters: BTreeMap<String, String> = BTreeMap::new();
//         parameters.insert("symbol".into(), symbol.into());
//         parameters.insert("orderId".into(), order_id.to_string());
//         let request = utils::build_signed_request(Some(parameters), self.recv_window);
//         self.client
//             .delete_signed::<Empty>(Endpoint::SpotTrading(SpotTradingEndpoints::OrderTest), Some(request))
//             .await
//             .map(|_| ())
//     }

//     fn build_quote_quantity_order(
//         &self,
//         order: OrderQuoteQuantityRequest,
//     ) -> BTreeMap<String, String> {
//         let mut order_parameters: BTreeMap<String, String> = BTreeMap::new();

//         order_parameters.insert("symbol".into(), order.symbol);
//         order_parameters.insert("side".into(), order.order_side.to_string());
//         order_parameters.insert("type".into(), order.order_type.to_string());
//         order_parameters.insert("quoteOrderQty".into(), order.quote_order_qty.to_string());

//         if order.price != 0.0 {
//             order_parameters.insert("price".into(), order.price.to_string());
//             order_parameters.insert("timeInForce".into(), order.time_in_force.to_string());
//         }

//         if let Some(client_order_id) = order.new_client_order_id {
//             order_parameters.insert("newClientOrderId".into(), client_order_id);
//         }

//         order_parameters
//     }
// }
