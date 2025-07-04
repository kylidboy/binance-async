// use crate::{
//     endpoints::{public_enums::*, BaseRequest, Endpoint, EndpointRequest, SecurityType},
//     models::*,
// };

// use binance_api_macros::{APIEndPoint, APIRequestInit, APIRequestToString};

// use serde::{Deserialize, Serialize};
// use strum::Display;


// #[derive(Debug, APIEndPoint)]
// #[allow(dead_code)]
// pub enum AccountInfoEP {
//     #[endpoint(GET, UserData, url = "/fapi/v3/account")]
//     AccountInfoV3,
// }


// #[derive(Debug, Serialize, APIRequestInit, APIRequestToString)]
// #[serde(rename_all = "camelCase")]
// pub struct AccountInfoV3Request(BaseRequest);
// impl EndpointRequest for AccountInfoV3Request {
//     type Response =
// }
