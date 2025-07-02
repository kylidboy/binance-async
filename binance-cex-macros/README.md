## Specs

Provide 2 derive macros.

Make life easier with `#[derive(ApiRequestRequire)]` and `#[derive(ApiRequestToString)]`.

For example:
``` rust
#[derive(Debug, Serialize, ApiRequestRequire, ApiRequestToString)]
#[serde(rename_all = "camelCase")]
pub struct QueryOrderRequest {
    pub symbol: String,
    pub order_id: Option<u64>,
    pub orig_client_order_id: Option<String>,
    #[serde(flatten)]
    pub base: BaseRequest,
}
```
