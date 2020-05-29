use serde::{Deserialize, Serialize};
use straitjacket_macro::straitjacket;

pub mod method;
pub mod metric;
pub mod plan;

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeploymentOption {
    Hosted,
    SelfManaged,
    ServiceMeshIstio,
    PluginRest,
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum AuthenticationMode {
    #[serde(rename = "1")]
    APIKey,
    #[serde(rename = "2")]
    AppIdKey,
    #[serde(rename = "oidc")]
    OIDC,
}

// Work around embedding Metadata into Service.
// See straitjacket_macro#1.
#[doc(hidden)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmptyMetadata;

#[straitjacket(metadata = "EmptyMetadata")]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Service {
    id: u64,
    name: String,
    state: String,
    system_name: String,
    backend_version: AuthenticationMode,
    description: String,
    intentions_required: bool,
    buyers_manage_apps: bool,
    buyers_manage_keys: bool,
    referrer_filters_required: bool,
    custom_keys_enabled: bool,
    buyer_key_regenerate_enabled: bool,
    mandatory_app_key: bool,
    buyer_can_select_plan: bool,
    buyer_plan_change_permission: String,
    // from this point on, the next fields are only provided when calling Service Read
    // notification_settings: Option<NotificationSettings>,
    deployment_option: Option<DeploymentOption>,
    support_email: Option<String>,
    metrics: Option<Vec<metric::Metric>>,
    #[serde(flatten, skip_serializing)]
    metadata: Option<crate::resources::Metadata>,
}

impl Service {
    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn system_name(&self) -> &str {
        self.system_name.as_str()
    }

    pub fn description(&self) -> &str {
        self.description.as_str()
    }

    pub fn authentication_mode(&self) -> AuthenticationMode {
        self.backend_version
    }

    pub fn deployment_option(&self) -> Option<DeploymentOption> {
        self.deployment_option
    }

    pub fn metadata(&self) -> Option<&crate::resources::Metadata> {
        self.metadata.as_ref()
    }

    pub fn metrics(&self) -> Result<crate::deps::url::Url, Box<dyn std::error::Error>> {
        self.metadata()
            .ok_or("no metadata present")?
            .find_url("metrics")
    }

    pub fn application_plans(&self) -> Result<crate::deps::url::Url, Box<dyn std::error::Error>> {
        self.metadata()
            .ok_or("no metadata present")?
            .find_url("application_plans")
    }
}

impl std::fmt::Display for Service {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{id} {system_name} {authentication_mode:#?} {deployment_option:?} {description}",
            id = self.id(),
            system_name = self.system_name(),
            authentication_mode = self.authentication_mode(),
            deployment_option = self.deployment_option(),
            description = self.description()
        )
    }
}

endpoint!(LIST, GET joining [ "/admin/api/services.json"] returning Services);

#[cfg(test)]
mod test {
    use super::*;

    endpoint_test! { it_parses, LIST, r##"{
    "services": [
      {
        "service": {
          "id": 2555417760888,
          "name": "Random API",
          "state": "incomplete",
          "system_name": "test",
          "backend_version": "2",
          "deployment_option": "hosted",
          "support_email": "joaquim@redhat.com",
          "description": "This API is not managed by the Threescale API operator",
          "intentions_required": false,
          "buyers_manage_apps": true,
          "buyers_manage_keys": true,
          "referrer_filters_required": false,
          "custom_keys_enabled": true,
          "buyer_key_regenerate_enabled": true,
          "mandatory_app_key": true,
          "buyer_can_select_plan": false,
          "buyer_plan_change_permission": "request",
          "created_at": "2018-10-02T17:34:59+01:00",
          "updated_at": "2020-05-11T13:55:00+01:00",
          "links": [
            {
              "rel": "metrics",
              "href": "https://istiodevel-admin.3scale.net/admin/api/services/2555417760888/metrics"
            },
            {
              "rel": "self",
              "href": "https://istiodevel-admin.3scale.net/admin/api/services/2555417760888"
            },
            {
              "rel": "service_plans",
              "href": "https://istiodevel-admin.3scale.net/admin/api/services/2555417760888/service_plans"
            },
            {
              "rel": "application_plans",
              "href": "https://istiodevel-admin.3scale.net/admin/api/services/2555417760888/application_plans"
            },
            {
              "rel": "features",
              "href": "https://istiodevel-admin.3scale.net/admin/api/services/2555417760888/features"
            }
          ]
        }
      },
      {
        "service": {
          "id": 2555417764324,
          "name": "hello-world",
          "state": "incomplete",
          "system_name": "hello-world",
          "backend_version": "1",
          "deployment_option": "service_mesh_istio",
          "support_email": "joaquim@redhat.com",
          "description": "Hi world :D",
          "intentions_required": false,
          "buyers_manage_apps": true,
          "buyers_manage_keys": true,
          "referrer_filters_required": false,
          "custom_keys_enabled": true,
          "buyer_key_regenerate_enabled": true,
          "mandatory_app_key": true,
          "buyer_can_select_plan": false,
          "buyer_plan_change_permission": "request",
          "created_at": "2018-12-19T16:34:47+00:00",
          "updated_at": "2020-05-14T15:00:12+01:00",
          "links": [
            {
              "rel": "metrics",
              "href": "https://istiodevel-admin.3scale.net/admin/api/services/2555417764324/metrics"
            },
            {
              "rel": "self",
              "href": "https://istiodevel-admin.3scale.net/admin/api/services/2555417764324"
            },
            {
              "rel": "service_plans",
              "href": "https://istiodevel-admin.3scale.net/admin/api/services/2555417764324/service_plans"
            },
            {
              "rel": "application_plans",
              "href": "https://istiodevel-admin.3scale.net/admin/api/services/2555417764324/application_plans"
            },
            {
              "rel": "features",
              "href": "https://istiodevel-admin.3scale.net/admin/api/services/2555417764324/features"
            }
          ]
        }
      },
      {
        "service": {
          "id": 2555417777820,
          "name": "echo-api",
          "state": "incomplete",
          "system_name": "echo-api",
          "backend_version": "1",
          "deployment_option": "self_managed",
          "support_email": "joaquim@redhat.com",
          "description": "Echo API",
          "intentions_required": false,
          "buyers_manage_apps": true,
          "buyers_manage_keys": true,
          "referrer_filters_required": false,
          "custom_keys_enabled": true,
          "buyer_key_regenerate_enabled": true,
          "mandatory_app_key": true,
          "buyer_can_select_plan": false,
          "buyer_plan_change_permission": "request",
          "notification_settings": {
            "web_provider": [
              0
            ],
            "email_provider": [
              0
            ],
            "web_buyer": [
              0
            ],
            "email_buyer": [
              0
            ]
          },
          "created_at": "2019-03-19T09:01:01+00:00",
          "updated_at": "2020-04-05T22:44:57+01:00",
          "links": [
            {
              "rel": "metrics",
              "href": "https://istiodevel-admin.3scale.net/admin/api/services/2555417777820/metrics"
            },
            {
              "rel": "self",
              "href": "https://istiodevel-admin.3scale.net/admin/api/services/2555417777820"
            },
            {
              "rel": "service_plans",
              "href": "https://istiodevel-admin.3scale.net/admin/api/services/2555417777820/service_plans"
            },
            {
              "rel": "application_plans",
              "href": "https://istiodevel-admin.3scale.net/admin/api/services/2555417777820/application_plans"
            },
            {
              "rel": "features",
              "href": "https://istiodevel-admin.3scale.net/admin/api/services/2555417777820/features"
            }
          ]
        }
      },
      {
        "service": {
          "id": 2555417783508,
          "name": "Bookinfo API",
          "state": "incomplete",
          "system_name": "bookinfo",
          "backend_version": "1",
          "deployment_option": "service_mesh_istio",
          "support_email": "amr@redhat.com",
          "description": "",
          "intentions_required": false,
          "buyers_manage_apps": true,
          "buyers_manage_keys": true,
          "referrer_filters_required": false,
          "custom_keys_enabled": true,
          "buyer_key_regenerate_enabled": true,
          "mandatory_app_key": true,
          "buyer_can_select_plan": false,
          "buyer_plan_change_permission": "request",
          "created_at": "2019-03-31T02:33:10+01:00",
          "updated_at": "2019-10-13T22:23:29+01:00",
          "links": [
            {
              "rel": "metrics",
              "href": "https://istiodevel-admin.3scale.net/admin/api/services/2555417783508/metrics"
            },
            {
              "rel": "self",
              "href": "https://istiodevel-admin.3scale.net/admin/api/services/2555417783508"
            },
            {
              "rel": "service_plans",
              "href": "https://istiodevel-admin.3scale.net/admin/api/services/2555417783508/service_plans"
            },
            {
              "rel": "application_plans",
              "href": "https://istiodevel-admin.3scale.net/admin/api/services/2555417783508/application_plans"
            },
            {
              "rel": "features",
              "href": "https://istiodevel-admin.3scale.net/admin/api/services/2555417783508/features"
            }
          ]
        }
      },
      {
        "service": {
          "id": 2555417814278,
          "name": "phala-test",
          "state": "incomplete",
          "system_name": "phala-test",
          "backend_version": "1",
          "deployment_option": "hosted",
          "support_email": "joaquim@redhat.com",
          "description": "",
          "intentions_required": false,
          "buyers_manage_apps": true,
          "buyers_manage_keys": true,
          "referrer_filters_required": false,
          "custom_keys_enabled": true,
          "buyer_key_regenerate_enabled": true,
          "mandatory_app_key": true,
          "buyer_can_select_plan": false,
          "buyer_plan_change_permission": "request",
          "created_at": "2019-06-21T13:21:43+01:00",
          "updated_at": "2019-07-15T17:14:35+01:00",
          "links": [
            {
              "rel": "metrics",
              "href": "https://istiodevel-admin.3scale.net/admin/api/services/2555417814278/metrics"
            },
            {
              "rel": "self",
              "href": "https://istiodevel-admin.3scale.net/admin/api/services/2555417814278"
            },
            {
              "rel": "service_plans",
              "href": "https://istiodevel-admin.3scale.net/admin/api/services/2555417814278/service_plans"
            },
            {
              "rel": "application_plans",
              "href": "https://istiodevel-admin.3scale.net/admin/api/services/2555417814278/application_plans"
            },
            {
              "rel": "features",
              "href": "https://istiodevel-admin.3scale.net/admin/api/services/2555417814278/features"
            }
          ]
        }
      },
      {
        "service": {
          "id": 2555417834780,
          "name": "GoL API",
          "state": "incomplete",
          "system_name": "gol-api",
          "backend_version": "1",
          "deployment_option": "plugin_rest",
          "support_email": "joaquim@redhat.com",
          "description": "Game of Life API",
          "intentions_required": false,
          "buyers_manage_apps": true,
          "buyers_manage_keys": true,
          "referrer_filters_required": false,
          "custom_keys_enabled": true,
          "buyer_key_regenerate_enabled": true,
          "mandatory_app_key": true,
          "buyer_can_select_plan": false,
          "buyer_plan_change_permission": "request",
          "created_at": "2019-12-10T15:55:07+00:00",
          "updated_at": "2019-12-10T16:27:53+00:00",
          "links": [
            {
              "rel": "metrics",
              "href": "https://istiodevel-admin.3scale.net/admin/api/services/2555417834780/metrics"
            },
            {
              "rel": "self",
              "href": "https://istiodevel-admin.3scale.net/admin/api/services/2555417834780"
            },
            {
              "rel": "service_plans",
              "href": "https://istiodevel-admin.3scale.net/admin/api/services/2555417834780/service_plans"
            },
            {
              "rel": "application_plans",
              "href": "https://istiodevel-admin.3scale.net/admin/api/services/2555417834780/application_plans"
            },
            {
              "rel": "features",
              "href": "https://istiodevel-admin.3scale.net/admin/api/services/2555417834780/features"
            }
          ]
        }
      }
    ]
  }"## }

    // helper fn
    fn parse_services() -> Vec<Service> {
        let services = LIST.parse_str(RESPONSE);
        assert!(services.is_ok());
        services.unwrap().into()
    }

    #[test]
    fn it_has_all_services_with_metadata() {
        assert!(parse_services().iter().all(|svc| svc.metadata.is_some()));
    }

    #[test]
    fn it_has_all_services_with_a_metrics_link() {
        assert!(parse_services().iter().all(|svc| svc.metrics().is_ok()));
    }

    #[test]
    fn it_has_all_services_with_an_application_plans_link() {
        assert!(parse_services()
            .iter()
            .all(|svc| svc.application_plans().is_ok()));
    }
}
