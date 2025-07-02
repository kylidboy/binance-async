pub mod api_config;
pub mod client;
pub mod endpoints;
pub mod errors;
pub mod models;
pub mod primitives;
pub mod ws_streams;

mod utils;

use static_init::dynamic;

#[dynamic]
pub static MAINNET: api_config::ApiConfig = api_config::ApiConfig::default();
#[dynamic]
pub static TESTNET: api_config::ApiConfig = api_config::ApiConfig::testnet();

#[cfg(test)]
mod tests {
    use super::{MAINNET, TESTNET};

    use crate::{
        client,
        endpoints::{
            convert, margin, market_data, spot_account, wallet, BaseRequest, OneOrManySymbol,
        },
        models::*,
        ws_streams::market_streams::{MarketStreams, PartialBookDepthLevel},
    };

    #[tokio::test]
    async fn get_asset_info() {
        let client = client::Client::new(None, None, &MAINNET.rest_api_endpoint);
        let req = convert::AssetInfoRequest(BaseRequest::require());
        let resp = client
            .access::<convert::AssetInfoRequest>(&convert::ConvertEP::AssetInfo, Some(req))
            .await;
        println!("{:?}", resp);
    }

    #[tokio::test]
    async fn market_data_order_book() {
        let client = client::Client::new(None, None, &MAINNET.rest_api_endpoint);
        let req = market_data::OrderBookRequest::require("BTCUSDT".to_string());
        let resp = client
            .access::<market_data::OrderBookRequest>(
                &market_data::MarketDataEP::OrderBook,
                Some(req),
            )
            .await;
        println!("{:?}", resp);
    }

    #[tokio::test]
    async fn market_data_price_ticker() {
        let client = client::Client::new(None, None, &MAINNET.rest_api_endpoint);
        let req = market_data::PriceTickerRequest(Some(OneOrManySymbol::Many(vec![
            "ETHUSDT".to_string(),
            "BNBUSDT".to_string(),
        ])));
        let resp = client
            .access::<market_data::PriceTickerRequest>(
                &market_data::MarketDataEP::PriceTicker,
                Some(req),
            )
            .await;
        println!("{:?}", resp);
    }

    #[tokio::test]
    async fn market_data_klines() {
        let client = client::Client::new(None, None, &MAINNET.rest_api_endpoint);
        let req = market_data::KlinesRequest::require(
            "BTCUSDT".to_string(),
            market_data::KlineInterval::_1h,
        );
        let resp = client
            .access::<market_data::KlinesRequest>(&market_data::MarketDataEP::Klines, Some(req))
            .await
            .unwrap();
        let resp: Vec<KlineSummary> = resp
            .into_iter()
            .map(|v| KlineSummary::try_from(v).unwrap())
            .collect();
        println!("{:?}", resp);
    }

    #[tokio::test]
    async fn market_data_agg_trades() {
        let client = client::Client::new(None, None, &MAINNET.rest_api_endpoint);
        let req = market_data::AggTradesRequest::require("BTCUSDT".to_string());
        let resp = client
            .access::<market_data::AggTradesRequest>(
                &market_data::MarketDataEP::AggTrades,
                Some(req),
            )
            .await
            .unwrap();
        println!("{:?}", resp);
    }

    #[tokio::test]
    async fn spot_account_account_info() {
        let apikey = envmnt::get_or_panic("TEST_APIKEY");
        let secret = envmnt::get_or_panic("TEST_SECRET");
        let client = client::Client::new(Some(apikey), Some(secret), &MAINNET.rest_api_endpoint);
        let mut req = spot_account::AccountRequest::require(BaseRequest::require());
        req.omit_zero_balances = Some(true);
        let resp = client
            .access::<spot_account::AccountRequest>(
                &spot_account::SpotAccountEP::Account,
                Some(req),
            )
            .await
            .unwrap();
        println!("{:?}", resp);
    }

    #[tokio::test]
    async fn spot_account_trade_list() {
        let apikey = envmnt::get_or_panic("TEST_APIKEY");
        let secret = envmnt::get_or_panic("TEST_SECRET");
        let client = client::Client::new(Some(apikey), Some(secret), &MAINNET.rest_api_endpoint);
        let req =
            spot_account::TradeListRequest::require("BTCUSDT".to_string(), BaseRequest::require());
        let resp = client
            .access::<spot_account::TradeListRequest>(
                &spot_account::SpotAccountEP::TradeList,
                Some(req),
            )
            .await
            .unwrap();
        println!("{:?}", resp);
    }

    #[tokio::test]
    async fn wallet_system_status() {
        let apikey = envmnt::get_or_panic("TEST_APIKEY");
        let secret = envmnt::get_or_panic("TEST_SECRET");
        let client = client::Client::new(Some(apikey), Some(secret), &MAINNET.rest_api_endpoint);
        let resp = client
            .access::<wallet::SystemStatusRequest>(&wallet::WalletEP::SystemStatus, None)
            .await
            .unwrap();
        println!("{:?}", resp);
    }

    #[tokio::test]
    async fn capital_config_get_all() {
        let apikey = envmnt::get_or_panic("TEST_APIKEY");
        let secret = envmnt::get_or_panic("TEST_SECRET");
        let client = client::Client::new(Some(apikey), Some(secret), &MAINNET.rest_api_endpoint);
        let req = wallet::AllCoinsRequest(BaseRequest::require());
        let resp = client
            .access::<wallet::AllCoinsRequest>(&wallet::WalletEP::CapitalConfigGetAll, Some(req))
            .await
            .unwrap();
        println!("{:?}", resp);
    }

    #[tokio::test]
    async fn cross_margin_pairs() {
        let apikey = envmnt::get_or_panic("TEST_APIKEY");
        let secret = envmnt::get_or_panic("TEST_SECRET");
        let client = client::Client::new(Some(apikey), Some(secret), &MAINNET.rest_api_endpoint);
        let cross_margin_pairs = client.access::<margin::CrossMarginPairsRequest>(
            &margin::MarginEP::CrossMarginPairs,
            Some(margin::CrossMarginPairsRequest::require()),
        );
        let resp = cross_margin_pairs.await;
        println!("{:?}", resp);
    }

    #[tokio::test]
    async fn cross_margin_fees() {
        let apikey = envmnt::get_or_panic("TEST_APIKEY");
        let secret = envmnt::get_or_panic("TEST_SECRET");
        let client = client::Client::new(Some(apikey), Some(secret), &MAINNET.rest_api_endpoint);
        let resp = client
            .access::<margin::CrossMarginFeeDataRequest>(
                &margin::MarginEP::CrossMarginFeeData,
                Some(margin::CrossMarginFeeDataRequest::require(
                    BaseRequest::require(),
                )),
            )
            .await
            .unwrap();
        println!("{:?}", resp);
    }

    #[tokio::test]
    async fn isolated_margin_pairs() {
        let apikey = envmnt::get_or_panic("TEST_APIKEY");
        let secret = envmnt::get_or_panic("TEST_SECRET");
        let client = client::Client::new(Some(apikey), Some(secret), &MAINNET.rest_api_endpoint);
        let resp = client
            .access::<margin::IsolatedMarginPairsRequest>(
                &margin::MarginEP::IsolatedMarginPairs,
                Some(margin::IsolatedMarginPairsRequest::require()),
            )
            .await
            .unwrap();
        println!("{:?}", resp);
    }
}
