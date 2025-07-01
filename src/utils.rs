#![allow(dead_code)]

use eyre::Result;
use serde_json::Value;
use std::collections::BTreeMap;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn request_to_query_string(parameters: BTreeMap<String, String>) -> String {
    assert!(!parameters.is_empty());
    parameters
        .iter()
        .fold(vec![], |mut acc, (k, v)| {
            acc.push(format!("{}={}", k, v));
            acc
        })
        .join("&")
}

// pub fn build_signed_request(
//     p: Option<BTreeMap<String, String>>,
//     recv_window: u64,
// ) -> String {
//     build_signed_request_custom(p, recv_window, SystemTime::now())
// }

// pub fn build_signed_request_custom(
//     parameters: Option<BTreeMap<String, String>>,
//     recv_window: u64,
//     start: SystemTime,
// ) -> String {
//     let mut params = parameters.unwrap_or_default();
//     params.insert(
//         "timestamp".into(),
//         get_timestamp(start).unwrap().to_string(),
//     );
//     if recv_window > 0 {
//         params.insert("recvWindow".into(), recv_window.to_string());
//     }
//     return request_to_query_string(params);
// }

pub fn to_i64(v: &Value) -> i64 {
    v.as_i64().unwrap()
}

pub fn to_f64(v: &Value) -> f64 {
    v.as_str().unwrap().parse().unwrap()
}

fn get_timestamp(start: SystemTime) -> Result<u64> {
    let since_epoch = start.duration_since(UNIX_EPOCH)?;
    Ok(since_epoch.as_secs() * 1000 + u64::from(since_epoch.subsec_nanos()) / 1_000_000)
}
