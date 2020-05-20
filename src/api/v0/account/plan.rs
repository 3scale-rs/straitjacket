use serde::{Deserialize, Serialize};
use straitjacket_macro::straitjacket;

pub type Metadata = crate::resources::Metadata;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum State {
    #[serde(rename(serialize = "publish", deserialize = "published"))]
    Published,
    #[serde(rename(serialize = "hide", deserialize = "hidden"))]
    Hidden,
}

#[straitjacket(name_snake = "account_plan")]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Plan {
    custom: Option<bool>,
    default: Option<bool>,
    id: u64,
    name: String,
    #[serde(rename = "type")]
    type_: Option<String>,
    #[serde(rename(serialize = "state_event"))]
    state: State,
    approval_required: bool,
    setup_fee: f64,
    cost_per_month: f64,
    trial_period_days: Option<u64>, // strangely this might return empty contents when listing but not when reading
    cancellation_period: Option<u64>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let body = r##"
{
  "plans": [
    {
      "account_plan": {
        "id": 2357355953034,
        "name": "Default",
        "state": "hidden",
        "setup_fee": 0,
        "cost_per_month": 0,
        "trial_period_days": 0,
        "cancellation_period": 0,
        "approval_required": false,
        "default": true,
        "created_at": "2018-07-06T12:30:20+01:00",
        "updated_at": "2018-07-06T12:30:20+01:00"
      }
    }
  ]
}
        "##;
        let plans: Result<Plans, _> = serde_json::from_str(&body);
        assert!(plans.is_ok());
    }
}
