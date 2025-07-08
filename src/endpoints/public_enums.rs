use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "UPPERCASE")]
pub enum OrderSide {
    #[default]
    Buy,
    Sell,
}
