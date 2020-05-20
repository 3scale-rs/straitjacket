use serde::{Deserialize, Serialize};
use straitjacket_macro::straitjacket;

pub type Metadata = crate::resources::Metadata;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Period {
    Minute,
    Hour,
    Day,
    Week,
    Month,
    Year,
    Eternity,
}

#[straitjacket]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Limit {
    id: String,
    metric_id: String,
    plan_id: String,
    period: Period,
    value: u64,
}
