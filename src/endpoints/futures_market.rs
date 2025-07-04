use crate::{
    endpoints::{public_enums::*, BaseRequest, Endpoint, EndpointRequest, SecurityType},
    models::*,
};

use binance_api_macros::{APIEndPoint, APIRequestInit, APIRequestToString};

use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Debug, APIEndPoint)]
#[allow(dead_code)]
pub enum FuturesMarketEP {
    #[endpoint(GET, None, url = "/fapi/v1/exchangeInfo")]
    ExchangeInfo,
}

#[derive(Debug, APIRequestInit, APIRequestToString, Serialize, Deserialize)]
pub struct ExchangeInfoRequest {}

impl EndpointRequest for ExchangeInfoRequest {
    type Response = serde_json::Value;
}

/*
 * {
    "exchangeFilters": [],
     "rateLimits": [ // API访问的限制
         {
             "interval": "MINUTE", // 按照分钟计算
               "intervalNum": 1, // 按照1分钟计算
               "limit": 2400, // 上限次数
               "rateLimitType": "REQUEST_WEIGHT" // 按照访问权重来计算
           },
          {
              "interval": "MINUTE",
               "intervalNum": 1,
               "limit": 1200,
               "rateLimitType": "ORDERS" // 按照订单数量来计算
           }
       ],
     "serverTime": 1565613908500, // 请忽略。如果需要获取当前系统时间，请查询接口 “GET /fapi/v1/time”
     "assets": [ // 资产信息
         {
             "asset": "BTC",
               "marginAvailable": true, // 是否可用作保证金
               "autoAssetExchange": "-0.10" // 保证金资产自动兑换阈值
           },
         {
             "asset": "USDT",
               "marginAvailable": true, // 是否可用作保证金
               "autoAssetExchange": "0" // 保证金资产自动兑换阈值
           },
         {
             "asset": "BNB",
               "marginAvailable": false, // 是否可用作保证金
               "autoAssetExchange": null // 保证金资产自动兑换阈值
           }
       ],
     "symbols": [ // 交易对信息
         {
             "symbol": "BLZUSDT",  // 交易对
             "pair": "BLZUSDT",  // 标的交易对
             "contractType": "PERPETUAL",	// 合约类型
             "deliveryDate": 4133404800000,  // 交割日期
             "onboardDate": 1598252400000,	  // 上线日期
             "status": "TRADING",  // 交易对状态
             "maintMarginPercent": "2.5000",  // 请忽略
             "requiredMarginPercent": "5.0000", // 请忽略
             "baseAsset": "BLZ",  // 标的资产
             "quoteAsset": "USDT", // 报价资产
             "marginAsset": "USDT", // 保证金资产
             "pricePrecision": 5,  // 价格小数点位数(仅作为系统精度使用，注意同tickSize 区分）
             "quantityPrecision": 0,  // 数量小数点位数(仅作为系统精度使用，注意同stepSize 区分）
             "baseAssetPrecision": 8,  // 标的资产精度
             "quotePrecision": 8,  // 报价资产精度
             "underlyingType": "COIN",
             "underlyingSubType": ["STORAGE"],
             "settlePlan": 0,
             "triggerProtect": "0.15", // 开启"priceProtect"的条件订单的触发阈值
             "filters": [
                 {
                     "filterType": "PRICE_FILTER", // 价格限制
                     "maxPrice": "300", // 价格上限, 最大价格
                     "minPrice": "0.0001", // 价格下限, 最小价格
                     "tickSize": "0.0001" // 订单最小价格间隔
                 },
                {
                    "filterType": "LOT_SIZE", // 数量限制
                     "maxQty": "10000000", // 数量上限, 最大数量
                     "minQty": "1", // 数量下限, 最小数量
                     "stepSize": "1" // 订单最小数量间隔
                 },
                {
                    "filterType": "MARKET_LOT_SIZE", // 市价订单数量限制
                     "maxQty": "590119", // 数量上限, 最大数量
                     "minQty": "1", // 数量下限, 最小数量
                     "stepSize": "1" // 允许的步进值
                 },
                 {
                    "filterType": "MAX_NUM_ORDERS", // 最多订单数限制
                    "limit": 200
                  },
                  {
                    "filterType": "MAX_NUM_ALGO_ORDERS", // 最多条件订单数限制
                    "limit": 10
                  },
                  {
                      "filterType": "MIN_NOTIONAL",  // 最小名义价值
                      "notional": "5.0",
                  },
                  {
                    "filterType": "PERCENT_PRICE", // 价格比限制
                    "multiplierUp": "1.1500", // 价格上限百分比
                    "multiplierDown": "0.8500", // 价格下限百分比
                    "multiplierDecimal": "4"
                }
               ],
             "OrderType": [ // 订单类型
                   "LIMIT",  // 限价单
                   "MARKET",  // 市价单
                   "STOP", // 止损单
                   "STOP_MARKET", // 止损市价单
                   "TAKE_PROFIT", // 止盈单
                   "TAKE_PROFIT_MARKET", // 止盈暑市价单
                   "TRAILING_STOP_MARKET" // 跟踪止损市价单
               ],
               "timeInForce": [ // 有效方式
                   "GTC", // 成交为止, 一直有效
                   "IOC", // 无法立即成交(吃单)的部分就撤销
                   "FOK", // 无法全部立即成交就撤销
                   "GTX" // 无法成为挂单方就撤销
             ],
             "liquidationFee": "0.010000",	// 强平费率
               "marketTakeBound": "0.30",	// 市价吃单(相对于标记价格)允许可造成的最大价格偏离比例
         }
       ],
    "timezone": "UTC" // 服务器所用的时间区域
}
 */
