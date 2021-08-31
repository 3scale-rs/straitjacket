use serde::{Deserialize, Serialize};
use straitjacket_macro::straitjacket;

pub type Metadata = crate::resources::Metadata;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum State {
    Live,
    Suspended,
    #[serde(other)]
    Unknown,
}

#[straitjacket]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Application {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub state: State,
    pub enabled: bool,
    pub service_id: u64,
    pub service_name: Option<String>,
    pub plan_id: u64,
    pub plan_name: Option<String>,
    pub account_id: u64,
    pub org_name: Option<String>,
    pub first_traffic_at: Option<String>,
    pub first_daily_traffic_at: Option<String>,
    pub user_key: Option<String>,
    pub provider_verification_key: Option<String>,
}

endpoint! { EP_LIST_APPLICATIONS, GET joining [ "/admin/api/accounts/", "/applications.json" ] returning Applications }
endpoint_test! { it_parses, EP_LIST_APPLICATIONS, r##"{
  "applications": [
    {
      "application": {
        "id": 1409618117181,
        "state": "live",
        "enabled": true,
        "created_at": "2018-12-20T10:01:26+00:00",
        "updated_at": "2020-07-15T10:43:01+01:00",
        "service_id": 2555417764324,
        "service_name": "hello-world",
        "plan_id": 2357355970342,
        "plan_name": "app-plan",
        "account_id": 2445582571514,
        "org_name": "Developer",
        "first_traffic_at": "2018-12-20T10:02:32+00:00",
        "first_daily_traffic_at": "2020-07-15T10:42:48+01:00",
        "user_key": "8941a95ad91100daf349a379541957ba",
        "provider_verification_key": "bf58729fe608d4d01045a2e1856a367e",
        "links": [
          {
            "rel": "self",
            "href": "https://istiodevel-admin.3scale.net/admin/api/accounts/2445582571514/applications/1409618117181"
          },
          {
            "rel": "service",
            "href": "https://istiodevel-admin.3scale.net/admin/api/services/2555417764324"
          },
          {
            "rel": "account",
            "href": "https://istiodevel-admin.3scale.net/admin/api/accounts/2445582571514"
          },
          {
            "rel": "plan",
            "href": "https://istiodevel-admin.3scale.net/admin/api/services/2555417764324/application_plans/2357355970342"
          },
          {
            "rel": "keys",
            "href": "https://istiodevel-admin.3scale.net/admin/api/accounts/2445582571514/applications/1409618117181/keys"
          },
          {
            "rel": "referrer_filters",
            "href": "https://istiodevel-admin.3scale.net/admin/api/accounts/2445582571514/applications/1409618117181/referrer_filters"
          }
        ],
        "name": "dummy-app",
        "description": "A dummy application"
      }
    },
    {
      "application": {
        "id": 1409618312497,
        "state": "live",
        "enabled": true,
        "created_at": "2019-03-31T02:59:35+01:00",
        "updated_at": "2020-04-05T22:59:25+01:00",
        "service_id": 2555417783508,
        "service_name": "Bookinfo API",
        "plan_id": 2357356173645,
        "plan_name": "App Plan (custom)",
        "account_id": 2445582716891,
        "org_name": "IstioUsers",
        "first_traffic_at": "2019-04-23T17:21:34+01:00",
        "first_daily_traffic_at": "2019-10-16T17:17:00+01:00",
        "user_key": "39990160ce907dcfee58594d27f0cf7f",
        "provider_verification_key": "12e9f41fdb9860d9f84e7de1fb002e82",
        "links": [
          {
            "rel": "self",
            "href": "https://istiodevel-admin.3scale.net/admin/api/accounts/2445582716891/applications/1409618312497"
          },
          {
            "rel": "service",
            "href": "https://istiodevel-admin.3scale.net/admin/api/services/2555417783508"
          },
          {
            "rel": "account",
            "href": "https://istiodevel-admin.3scale.net/admin/api/accounts/2445582716891"
          },
          {
            "rel": "plan",
            "href": "https://istiodevel-admin.3scale.net/admin/api/services/2555417783508/application_plans/2357356173645"
          },
          {
            "rel": "keys",
            "href": "https://istiodevel-admin.3scale.net/admin/api/accounts/2445582716891/applications/1409618312497/keys"
          },
          {
            "rel": "referrer_filters",
            "href": "https://istiodevel-admin.3scale.net/admin/api/accounts/2445582716891/applications/1409618312497/referrer_filters"
          }
        ],
        "name": "MyBookInfoApp",
        "description": "An App consuming Bookinfo"
      }
    },
    {
      "application": {
        "id": 1409618397539,
        "state": "live",
        "enabled": true,
        "created_at": "2019-05-13T15:32:54+01:00",
        "updated_at": "2020-05-11T13:55:18+01:00",
        "service_id": 2555417760888,
        "service_name": "Random API",
        "plan_id": 2357356077813,
        "plan_name": "test",
        "account_id": 2445582571514,
        "org_name": "Developer",
        "first_traffic_at": "2019-05-13T17:18:45+01:00",
        "first_daily_traffic_at": "2020-05-11T13:55:00+01:00",
        "application_id": "4946cbcc",
        "links": [
          {
            "rel": "self",
            "href": "https://istiodevel-admin.3scale.net/admin/api/accounts/2445582571514/applications/1409618397539"
          },
          {
            "rel": "service",
            "href": "https://istiodevel-admin.3scale.net/admin/api/services/2555417760888"
          },
          {
            "rel": "account",
            "href": "https://istiodevel-admin.3scale.net/admin/api/accounts/2445582571514"
          },
          {
            "rel": "plan",
            "href": "https://istiodevel-admin.3scale.net/admin/api/services/2555417760888/application_plans/2357356077813"
          },
          {
            "rel": "keys",
            "href": "https://istiodevel-admin.3scale.net/admin/api/accounts/2445582571514/applications/1409618397539/keys"
          },
          {
            "rel": "referrer_filters",
            "href": "https://istiodevel-admin.3scale.net/admin/api/accounts/2445582571514/applications/1409618397539/referrer_filters"
          }
        ],
        "name": "test",
        "description": "test"
      }
    },
    {
      "application": {
        "id": 1409618397543,
        "state": "live",
        "enabled": true,
        "created_at": "2019-05-13T15:33:12+01:00",
        "updated_at": "2019-05-13T15:33:12+01:00",
        "service_id": 2555417760888,
        "service_name": "Random API",
        "plan_id": 2357356077813,
        "plan_name": "test",
        "account_id": 2445582571514,
        "org_name": "Developer",
        "first_traffic_at": null,
        "first_daily_traffic_at": null,
        "application_id": "603f124d",
        "links": [
          {
            "rel": "self",
            "href": "https://istiodevel-admin.3scale.net/admin/api/accounts/2445582571514/applications/1409618397543"
          },
          {
            "rel": "service",
            "href": "https://istiodevel-admin.3scale.net/admin/api/services/2555417760888"
          },
          {
            "rel": "account",
            "href": "https://istiodevel-admin.3scale.net/admin/api/accounts/2445582571514"
          },
          {
            "rel": "plan",
            "href": "https://istiodevel-admin.3scale.net/admin/api/services/2555417760888/application_plans/2357356077813"
          },
          {
            "rel": "keys",
            "href": "https://istiodevel-admin.3scale.net/admin/api/accounts/2445582571514/applications/1409618397543/keys"
          },
          {
            "rel": "referrer_filters",
            "href": "https://istiodevel-admin.3scale.net/admin/api/accounts/2445582571514/applications/1409618397543/referrer_filters"
          }
        ],
        "name": "test",
        "description": "test"
      }
    },
    {
      "application": {
        "id": 1409618796841,
        "state": "live",
        "enabled": true,
        "created_at": "2019-12-10T16:40:32+00:00",
        "updated_at": "2019-12-10T16:43:14+00:00",
        "service_id": 2555417834780,
        "service_name": "GoL API",
        "plan_id": 2357356138849,
        "plan_name": "sample",
        "account_id": 2445582851727,
        "org_name": "Org",
        "first_traffic_at": "2019-12-10T16:42:57+00:00",
        "first_daily_traffic_at": "2019-12-10T16:42:57+00:00",
        "user_key": "8db9eea3edd6914d5f1ea0f3599e1019",
        "provider_verification_key": "4f5cfec69473c4923eafb93b1a560849",
        "links": [
          {
            "rel": "self",
            "href": "https://istiodevel-admin.3scale.net/admin/api/accounts/2445582851727/applications/1409618796841"
          },
          {
            "rel": "service",
            "href": "https://istiodevel-admin.3scale.net/admin/api/services/2555417834780"
          },
          {
            "rel": "account",
            "href": "https://istiodevel-admin.3scale.net/admin/api/accounts/2445582851727"
          },
          {
            "rel": "plan",
            "href": "https://istiodevel-admin.3scale.net/admin/api/services/2555417834780/application_plans/2357356138849"
          },
          {
            "rel": "keys",
            "href": "https://istiodevel-admin.3scale.net/admin/api/accounts/2445582851727/applications/1409618796841/keys"
          },
          {
            "rel": "referrer_filters",
            "href": "https://istiodevel-admin.3scale.net/admin/api/accounts/2445582851727/applications/1409618796841/referrer_filters"
          }
        ],
        "name": "my app",
        "description": "my app"
      }
    },
    {
      "application": {
        "id": 1409618796888,
        "state": "live",
        "enabled": true,
        "created_at": "2019-12-10T16:55:51+00:00",
        "updated_at": "2019-12-10T17:19:14+00:00",
        "service_id": 2555417834780,
        "service_name": "GoL API",
        "plan_id": 2357356138849,
        "plan_name": "sample",
        "account_id": 2445582851727,
        "org_name": "Org",
        "first_traffic_at": "2019-12-10T17:18:39+00:00",
        "first_daily_traffic_at": "2019-12-10T17:18:39+00:00",
        "user_key": "3f5579ff9a59d935b1541f1bb4910ee8",
        "provider_verification_key": "e3804c335f3ee7af85892cd4b9ce1fe6",
        "links": [
          {
            "rel": "self",
            "href": "https://istiodevel-admin.3scale.net/admin/api/accounts/2445582851727/applications/1409618796888"
          },
          {
            "rel": "service",
            "href": "https://istiodevel-admin.3scale.net/admin/api/services/2555417834780"
          },
          {
            "rel": "account",
            "href": "https://istiodevel-admin.3scale.net/admin/api/accounts/2445582851727"
          },
          {
            "rel": "plan",
            "href": "https://istiodevel-admin.3scale.net/admin/api/services/2555417834780/application_plans/2357356138849"
          },
          {
            "rel": "keys",
            "href": "https://istiodevel-admin.3scale.net/admin/api/accounts/2445582851727/applications/1409618796888/keys"
          },
          {
            "rel": "referrer_filters",
            "href": "https://istiodevel-admin.3scale.net/admin/api/accounts/2445582851727/applications/1409618796888/referrer_filters"
          }
        ],
        "name": "app-sdl",
        "description": "SDL app"
      }
    }
  ]
}"## }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_serializes() {
        let users = Applications::from(vec![Application {
            id: 2445583035585,
            service_id: 123456,
            service_name: None,
            plan_id: 1000,
            plan_name: Some("An app plan".into()),
            org_name: Some("An org name".into()),
            enabled: true,
            name: "AnApp".into(),
            description: "An app".into(),
            account_id: 123,
            state: State::Live,
            first_traffic_at: None,
            first_daily_traffic_at: None,
            user_key: None,
            provider_verification_key: None,
        }]);
        let result = serde_json::to_string_pretty(&users);
        match result {
            Err(ref e) => println!("Error: {:#?}", e),
            _ => (),
        }
        assert!(result.is_ok());
        println!("{}", result.unwrap());
    }
}
