use crate::models::OrderBook;
use chrono::{DateTime, Utc};

// e.g. "BTCUSDT", "ETHBNB"
pub type Pair = String;
// e.g. "BTC", "ETH"
pub type Symbol = String;

#[derive(Clone)]
pub struct PartialOrderBookDepthUpdate {
    pub at_utc: DateTime<Utc>,
    pub pair: Pair,
    pub orderbook: OrderBook,
}
impl Default for PartialOrderBookDepthUpdate {
    fn default() -> Self {
        Self {
            at_utc: chrono::Utc::now(),
            pair: String::new(),
            orderbook: Default::default(),
        }
    }
}
