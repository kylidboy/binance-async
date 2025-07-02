## Specs

Make life easier.

#### `#[derive(APIRequestInit)]`

Auto-gen "init" function receiveing non-optional fields through fn params.

Use case:
``` rust
#[derive(Debug, Serialize, APIRequestInit, APIRequestToString)]
#[serde(rename_all = "camelCase")]
pub struct TradeListRequest {
    pub symbol: String,
    pub order_id: Option<u64>,
    pub start_time: Option<u64>,
    pub end_time: Option<u64>,
    pub from_id: Option<u64>,
    pub limit: Option<i32>,

    #[serde(flatten)]
    pub base: BaseRequest,
    }
impl EndpointRequest for TradeListRequest {
    type Response = Vec<TradeHistory>;
}

let req = spot_account::TradeListRequest::init("BTCUSDT".to_string(), BaseRequest::init());
```

#### `#[derive(APIRequestToString)]`

Implement `to_string` function for requests to be transformed to query string by `serde_qs`.

#### `#[derive(APIEndPoint)]`

Allowed http method: GET, POST, PUT, DELETE

Allowed security type: 
``` rust
#[derive(Debug, Serialize, Deserialize)]
pub enum SecurityType {
    None,
    Trade,
    Margin,
    UserData,
    UserStream,
    MarketData,
}
```

The "url" attribute is from the Binance API doc.

*All three attributes are required*.

*Enum variant names are irrelevant*

Use case:
``` rust
#[derive(Debug, APIEndPoint)]
#[allow(dead_code)]
pub enum SpotMarketEP {
    #[endpoint(GET, None, url = "/api/v3/depth")]
    OrderBook,
    #[endpoint(GET, None, url = "/api/v3/ticker/price")]
    PriceTicker,
    #[endpoint(GET, None, url = "/api/v3/avgPrice")]
    CurrentAvgPrice,
    #[endpoint(GET, None, url = "/api/v3/ticker/bookTicker")]
    SymbolOrderBookTicker,
    #[endpoint(GET, None, url = "/api/v3/ticker/24hr")]
    Ticker24hr,
    #[endpoint(GET, None, url = "/api/v3/aggTrades")]
    AggTrades,
    #[endpoint(GET, None, url = "/api/v3/klines")]
    Klines,
    #[endpoint(GET, None, url = "/api/v3/trades")]
    Trades,
    #[endpoint(GET, None, url = "/api/v3/historicalTrades")]
    HistoricalTrades,
}
```
