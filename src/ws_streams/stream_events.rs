use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

use crate::errors::BinanceApiError;
use crate::models::*;

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum BinanceStreamEvent {
    AccountUpdate(AccountUpdateEvent),
    BalanceUpdate(BalanceUpdateEvent),
    OrderTrade(OrderTradeEvent),
    AggrTrades(AggrTradesEvent),
    Trade(TradeEvent),
    OrderBook(OrderBook),
    DayTicker(DayTickerEvent),
    DayTickerAll(Vec<DayTickerEvent>),
    WindowTicker(WindowTickerEvent),
    WindowTickerAll(Vec<WindowTickerEvent>),
    Kline(KlineEvent),
    DepthOrderBook(DepthOrderBookEvent),
    BookTicker(BookTickerEvent),
    // PartialOrderBookDepth(PartialOrderBookDepthEvent),
}

impl TryFrom<&str> for BinanceStreamEvent {
    type Error = BinanceApiError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(serde_json::from_str(value)?)
    }
}
