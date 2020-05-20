use serde::{Serialize, Deserialize};
use straitjacket_macro::strait;

pub type Metadata = crate::resources::Metadata;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum State {
    #[serde(rename(serialize = "publish", deserialize = "published"))]
    Published,
    #[serde(rename(serialize = "hide", deserialize = "hidden"))]
    Hidden,
}

#[strait]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApplicationPlan {
    custom: bool,
    default: bool,
    id: String,
    name: String,
    #[serde(rename = "type")]
    type_: String,
    #[serde(rename(serialize = "state_event"))]
    state: State,
    approval_required: bool,
    setup_fee: f64,
    cost_per_month: f64,
    trial_period_days: Option<u64>, // strangely this might return empty contents when listing but not when reading
    cancellation_period: Option<u64>,
}

