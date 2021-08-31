use serde::{Deserialize, Serialize};
use straitjacket_macro::straitjacket;

pub type Metadata = crate::resources::Metadata;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum State {
    #[serde(rename(serialize = "publish", deserialize = "published"))]
    Published,
    #[serde(rename(serialize = "hide", deserialize = "hidden"))]
    Hidden,
    #[serde(other)]
    Unknown,
}

#[straitjacket(name_snake = "application_plan")]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Plan {
    id: u64,
    name: String,
    #[serde(rename = "type")]
    type_: Option<String>,
    #[serde(rename(serialize = "state_event"))]
    state: State,
    setup_fee: f64,
    cost_per_month: f64,
    trial_period_days: Option<u64>, // strangely this might return empty contents when listing but not when reading
    cancellation_period: Option<u64>,
    approval_required: bool,
    default: Option<bool>,
    custom: Option<bool>,
    system_name: Option<String>,
}

endpoint! { LIST, GET joining [ "/admin/api/services/", "/application_plans.json"] returning Plans }

#[cfg(test)]
mod test {
    use super::*;

    endpoint_test! { it_parses, LIST, r#"{
      "plans": [
        {
          "application_plan": {
            "id": 2357356012630,
            "name": "gold",
            "state": "published",
            "setup_fee": 0,
            "cost_per_month": 0,
            "trial_period_days": 0,
            "cancellation_period": 0,
            "approval_required": false,
            "default": true,
            "created_at": "2019-03-19T09:04:40+00:00",
            "updated_at": "2019-03-19T09:04:40+00:00",
            "custom": false,
            "system_name": "gold",
            "links": [
              {
                "rel": "service",
                "href": "https://istiodevel-admin.3scale.net/admin/api/services/2555417777820"
              },
              {
                "rel": "self",
                "href": "https://istiodevel-admin.3scale.net/admin/api/services/2555417777820/application_plans/2357356012630"
              }
            ]
          }
        },
        {
          "application_plan": {
            "id": 2357356012631,
            "name": "silver",
            "state": "published",
            "setup_fee": 0,
            "cost_per_month": 0,
            "trial_period_days": 0,
            "cancellation_period": 0,
            "approval_required": false,
            "default": false,
            "created_at": "2019-03-19T09:04:40+00:00",
            "updated_at": "2019-03-19T09:04:41+00:00",
            "custom": false,
            "system_name": "silver",
            "links": [
              {
                "rel": "service",
                "href": "https://istiodevel-admin.3scale.net/admin/api/services/2555417777820"
              },
              {
                "rel": "self",
                "href": "https://istiodevel-admin.3scale.net/admin/api/services/2555417777820/application_plans/2357356012631"
              }
            ]
          }
        },
        {
          "application_plan": {
            "id": 2357356113485,
            "name": "config-DB8qe3HXVpk",
            "state": "hidden",
            "setup_fee": 0,
            "cost_per_month": 0,
            "trial_period_days": 0,
            "cancellation_period": 0,
            "approval_required": false,
            "default": false,
            "created_at": "2019-07-12T12:51:16+01:00",
            "updated_at": "2019-07-12T12:51:16+01:00",
            "custom": false,
            "system_name": "config_db8qe3hxvpk",
            "links": [
              {
                "rel": "service",
                "href": "https://istiodevel-admin.3scale.net/admin/api/services/2555417777820"
              },
              {
                "rel": "self",
                "href": "https://istiodevel-admin.3scale.net/admin/api/services/2555417777820/application_plans/2357356113485"
              }
            ]
          }
        }
      ]
    }"# }
}
