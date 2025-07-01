use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum PartialBookDepthLevel {
    Five,
    Ten,
    Twenty,
}
impl Display for PartialBookDepthLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PartialBookDepthLevel::Five => write!(f, "{}", 5),
            PartialBookDepthLevel::Ten => write!(f, "{}", 10),
            PartialBookDepthLevel::Twenty => write!(f, "{}", 20),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MarketStreams {
    DiffDepth1s(String),
    DiffDepth100ms(String),
    PartialBookDepth1s(String, PartialBookDepthLevel),
    PartialBookDepth100ms(String, PartialBookDepthLevel),
}

impl Into<String> for &MarketStreams {
    fn into(self) -> String {
        match self {
            MarketStreams::DiffDepth1s(s) => format!("{}@depth", s.to_lowercase()),
            MarketStreams::DiffDepth100ms(s) => format!("{}@depth@100ms", s.to_lowercase()),
            MarketStreams::PartialBookDepth1s(s, l) => format!("{}@depth{}", s.to_lowercase(), l),
            MarketStreams::PartialBookDepth100ms(s, l) => {
                format!("{}@depth{}@100ms", s.to_lowercase(), l)
            }
        }
    }
}
