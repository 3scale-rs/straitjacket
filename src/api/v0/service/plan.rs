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

#[straitjacket(name_snake = "application_plan")]
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
    },
    {
      "application_plan": {
        "id": 2357356113486,
        "name": "config-JuTEpSpXODk",
        "state": "hidden",
        "setup_fee": 0,
        "cost_per_month": 0,
        "trial_period_days": 0,
        "cancellation_period": 0,
        "approval_required": false,
        "default": false,
        "created_at": "2019-07-12T12:51:34+01:00",
        "updated_at": "2019-07-12T12:51:34+01:00",
        "custom": false,
        "system_name": "config_jutepspxodk",
        "links": [
          {
            "rel": "service",
            "href": "https://istiodevel-admin.3scale.net/admin/api/services/2555417777820"
          },
          {
            "rel": "self",
            "href": "https://istiodevel-admin.3scale.net/admin/api/services/2555417777820/application_plans/2357356113486"
          }
        ]
      }
    },
    {
      "application_plan": {
        "id": 2357356113487,
        "name": "config-gw7l0IgFGLQ",
        "state": "hidden",
        "setup_fee": 0,
        "cost_per_month": 0,
        "trial_period_days": 0,
        "cancellation_period": 0,
        "approval_required": false,
        "default": false,
        "created_at": "2019-07-12T12:56:22+01:00",
        "updated_at": "2019-07-12T12:56:22+01:00",
        "custom": false,
        "system_name": "config_gw7l0igfglq",
        "links": [
          {
            "rel": "service",
            "href": "https://istiodevel-admin.3scale.net/admin/api/services/2555417777820"
          },
          {
            "rel": "self",
            "href": "https://istiodevel-admin.3scale.net/admin/api/services/2555417777820/application_plans/2357356113487"
          }
        ]
      }
    },
    {
      "application_plan": {
        "id": 2357356113488,
        "name": "config-B0l8Kxzr0_I",
        "state": "hidden",
        "setup_fee": 0,
        "cost_per_month": 0,
        "trial_period_days": 0,
        "cancellation_period": 0,
        "approval_required": false,
        "default": false,
        "created_at": "2019-07-12T12:56:39+01:00",
        "updated_at": "2019-07-12T12:56:39+01:00",
        "custom": false,
        "system_name": "config_b0l8kxzr0_i",
        "links": [
          {
            "rel": "service",
            "href": "https://istiodevel-admin.3scale.net/admin/api/services/2555417777820"
          },
          {
            "rel": "self",
            "href": "https://istiodevel-admin.3scale.net/admin/api/services/2555417777820/application_plans/2357356113488"
          }
        ]
      }
    },
    {
      "application_plan": {
        "id": 2357356113628,
        "name": "config-K9YMiYvtxS8",
        "state": "hidden",
        "setup_fee": 0,
        "cost_per_month": 0,
        "trial_period_days": 0,
        "cancellation_period": 0,
        "approval_required": false,
        "default": false,
        "created_at": "2019-07-15T16:57:00+01:00",
        "updated_at": "2019-07-15T16:57:00+01:00",
        "custom": false,
        "system_name": "config_k9ymiyvtxs8",
        "links": [
          {
            "rel": "service",
            "href": "https://istiodevel-admin.3scale.net/admin/api/services/2555417777820"
          },
          {
            "rel": "self",
            "href": "https://istiodevel-admin.3scale.net/admin/api/services/2555417777820/application_plans/2357356113628"
          }
        ]
      }
    },
    {
      "application_plan": {
        "id": 2357356113629,
        "name": "config-WO1zqocNSbE",
        "state": "hidden",
        "setup_fee": 0,
        "cost_per_month": 0,
        "trial_period_days": 0,
        "cancellation_period": 0,
        "approval_required": false,
        "default": false,
        "created_at": "2019-07-15T16:57:27+01:00",
        "updated_at": "2019-07-15T16:57:27+01:00",
        "custom": false,
        "system_name": "config_wo1zqocnsbe",
        "links": [
          {
            "rel": "service",
            "href": "https://istiodevel-admin.3scale.net/admin/api/services/2555417777820"
          },
          {
            "rel": "self",
            "href": "https://istiodevel-admin.3scale.net/admin/api/services/2555417777820/application_plans/2357356113629"
          }
        ]
      }
    },
    {
      "application_plan": {
        "id": 2357356117805,
        "name": "fbfdhrh",
        "state": "hidden",
        "setup_fee": 0,
        "cost_per_month": 0,
        "trial_period_days": 0,
        "cancellation_period": 0,
        "approval_required": false,
        "default": false,
        "created_at": "2019-08-15T15:57:42+01:00",
        "updated_at": "2019-08-15T15:57:42+01:00",
        "custom": false,
        "system_name": "fbfdhrh",
        "links": [
          {
            "rel": "service",
            "href": "https://istiodevel-admin.3scale.net/admin/api/services/2555417777820"
          },
          {
            "rel": "self",
            "href": "https://istiodevel-admin.3scale.net/admin/api/services/2555417777820/application_plans/2357356117805"
          }
        ]
      }
    }
  ]
}
        "##;
        let plans: Result<Plans, _> = serde_json::from_str(&body);
        assert!(plans.is_ok());
    }
}
