use serde::{Deserialize, Serialize};
use straitjacket_macro::straitjacket;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Metadata;

#[straitjacket]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Metric {
    id: u64,
    name: String,
    system_name: String,
    friendly_name: String,
    description: String,
    unit: String,
    // Note, 3scale versions up to 2.8 (at least) don't include this crucial piece of data from JSON queries,
    // so there's no way to know if this has no parents or it does unless using XML, which includes it.
    metric_id: Option<u64>,
}
