use serde::{Deserialize, Deserializer, Serialize};
use straitjacket_macro::straitjacket;

use crate::resources::Metadata;

use super::super::{AuthenticationMode, DeploymentOption};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Environment {
    Staging,
    Production,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Backend {
    #[serde(deserialize_with = "parse_url", skip_serializing)]
    endpoint: crate::deps::url::Url,
    host: String,
}

fn parse_url<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<crate::deps::url::Url, D::Error> {
    let string: String = Deserialize::deserialize(deserializer)?;
    let url = crate::deps::url::Url::parse(&string);
    url.map_err(serde::de::Error::custom)
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(
    rename_all = "snake_case",
    tag = "backend_authentication_type",
    content = "backend_authentication_value"
)]
pub enum BackendAuthentication {
    ServiceToken(String),
    ProviderKey(String),
    #[serde(other)]
    Unknown,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogoFile {
    #[serde(rename = "logo_file_name")]
    name: String,
    #[serde(rename = "logo_content_type")]
    content_type: Option<String>,
    #[serde(rename = "logo_file_size")]
    file_size: Option<u64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DefaultPlans {
    #[serde(rename = "default_application_plan_id")]
    application_plan_id: Option<u64>,
    #[serde(rename = "default_service_plan_id")]
    service_plan_id: Option<u64>,
    #[serde(rename = "default_end_user_plan_id")]
    end_user_plan_id: Option<u64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SupportEmails {
    #[serde(rename = "support_email")]
    support: String,
    #[serde(rename = "tech_support_email")]
    tech: Option<String>,
    #[serde(rename = "admin_support_email")]
    admin: Option<String>,
    #[serde(rename = "credit_card_support_email")]
    credit_card: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BuyerSettings {
    buyers_manage_apps: bool,
    buyers_manage_keys: bool,
    buyer_plan_change_permission: String,
    buyer_can_select_plan: bool,
    buyer_key_regenerate_enabled: bool,
}

#[straitjacket]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Content {
    id: u64,
    account_id: u64,
    name: String,
    oneline_description: Option<String>,
    description: Option<String>,
    /*
    txt_api: ???,
    txt_support: ???,
    txt_features: ???,
    infobar: ???,
    terms: ???,
    notification_settings: ???,
    kubernetes_service_link: ???,
    */
    #[serde(flatten)]
    logo_file: Option<LogoFile>,
    #[serde(flatten)]
    support_emails: SupportEmails,
    state: String,
    intentions_required: bool,
    draft_name: String,
    display_provider_keys: bool,
    custom_keys_enabled: bool,
    #[serde(flatten)]
    buyer_settings: BuyerSettings,
    #[serde(flatten)]
    default_plans: DefaultPlans,
    end_user_registration_required: bool,
    tenant_id: u64,
    system_name: String,
    backend_version: AuthenticationMode,
    mandatory_app_key: bool,
    referrer_filters_required: bool,
    deployment_option: DeploymentOption,
    #[serde(rename = "proxiable?")]
    proxiable: bool,
    #[serde(flatten)]
    backend_authentication_type: BackendAuthentication,
    //proxy: super::Proxy,
}

#[straitjacket(name_snake = "proxy_config", plural_snake = "proxy_configs")]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    id: u64,
    version: u64,
    environment: Environment,
    content: Content,
}

endpoint! { LIST, GET joining [ "/admin/api/services/", "/proxy/configs/", ".json" ] returning Configs }

#[cfg(test)]
mod tests {
    use super::*;

    endpoint_test! { it_parses, LIST, r##"{
      "proxy_configs": [
        {
          "proxy_config": {
            "id": 92726,
            "version": 1,
            "environment": "production",
            "content": {
              "id": 2555417777820,
              "account_id": 2445582571513,
              "name": "echo-api",
              "oneline_description": null,
              "description": "Echo API",
              "txt_api": null,
              "txt_support": null,
              "txt_features": null,
              "created_at": "2019-03-19T09:01:01Z",
              "updated_at": "2019-03-19T09:04:29Z",
              "logo_file_name": null,
              "logo_content_type": null,
              "logo_file_size": null,
              "state": "incomplete",
              "intentions_required": false,
              "draft_name": "",
              "infobar": null,
              "terms": null,
              "display_provider_keys": false,
              "tech_support_email": null,
              "admin_support_email": null,
              "credit_card_support_email": null,
              "buyers_manage_apps": true,
              "buyers_manage_keys": true,
              "custom_keys_enabled": true,
              "buyer_plan_change_permission": "request",
              "buyer_can_select_plan": false,
              "notification_settings": null,
              "default_application_plan_id": null,
              "default_service_plan_id": null,
              "default_end_user_plan_id": null,
              "end_user_registration_required": true,
              "tenant_id": 2445582571513,
              "system_name": "echo-api",
              "backend_version": "1",
              "mandatory_app_key": true,
              "buyer_key_regenerate_enabled": true,
              "support_email": "joaquim@redhat.com",
              "referrer_filters_required": false,
              "deployment_option": "self_managed",
              "kubernetes_service_link": null,
              "proxiable?": true,
              "backend_authentication_type": "service_token",
              "backend_authentication_value": "3d3bfe783a66ad7576c2389d4a8623ea613cc5146dce2e603b001ccac17e36f8",
              "proxy": {
                "id": 124012,
                "tenant_id": 2445582571513,
                "service_id": 2555417777820,
                "endpoint": "http://production.3scale.net:80",
                "deployed_at": null,
                "api_backend": "https://echo-api.3scale.net:443",
                "auth_app_key": "app_key",
                "auth_app_id": "app_id",
                "auth_user_key": "api-key",
                "credentials_location": "headers",
                "error_auth_failed": "Authentication failed",
                "error_auth_missing": "Authentication Missing",
                "created_at": "2019-03-19T09:01:01Z",
                "updated_at": "2019-03-19T09:04:29Z",
                "error_status_auth_failed": 403,
                "error_headers_auth_failed": "text/plain; charset=us-ascii",
                "error_status_auth_missing": 401,
                "error_headers_auth_missing": "text/plain; charset=us-ascii",
                "error_no_match": "No Mapping Rule matched",
                "error_status_no_match": 404,
                "error_headers_no_match": "text/plain; charset=us-ascii",
                "secret_token": "MySecretTokenBetweenApicastAndMyBackend_1237120312",
                "hostname_rewrite": "",
                "oauth_login_url": null,
                "sandbox_endpoint": "http://staging.3scale.net:80",
                "api_test_path": "/",
                "api_test_success": null,
                "apicast_configuration_driven": true,
                "oidc_issuer_endpoint": null,
                "lock_version": 3,
                "authentication_method": "1",
                "hostname_rewrite_for_sandbox": "echo-api.3scale.net",
                "endpoint_port": 80,
                "valid?": true,
                "service_backend_version": "1",
                "hosts": [
                  "production.3scale.net",
                  "staging.3scale.net"
                ],
                "backend": {
                  "endpoint": "https://su1.3scale.net",
                  "host": "su1.3scale.net"
                },
                "policy_chain": [
                  {
                    "name": "apicast",
                    "version": "builtin",
                    "configuration": {}
                  }
                ],
                "proxy_rules": [
                  {
                    "id": 375837,
                    "proxy_id": 124012,
                    "http_method": "GET",
                    "pattern": "/",
                    "metric_id": 2555418191876,
                    "metric_system_name": "hits",
                    "delta": 1,
                    "tenant_id": 2445582571513,
                    "created_at": "2019-03-19T09:01:01Z",
                    "updated_at": "2019-03-19T09:01:01Z",
                    "redirect_url": null,
                    "position": 1,
                    "last": false,
                    "parameters": [],
                    "querystring_parameters": {}
                  }
                ]
              }
            }
          }
        },
        {
          "proxy_config": {
            "id": 92728,
            "version": 2,
            "environment": "production",
            "content": {
              "id": 2555417777820,
              "account_id": 2445582571513,
              "name": "echo-api",
              "oneline_description": null,
              "description": "Echo API",
              "txt_api": null,
              "txt_support": null,
              "txt_features": null,
              "created_at": "2019-03-19T09:01:01Z",
              "updated_at": "2019-03-19T09:04:41Z",
              "logo_file_name": null,
              "logo_content_type": null,
              "logo_file_size": null,
              "state": "incomplete",
              "intentions_required": false,
              "draft_name": "",
              "infobar": null,
              "terms": null,
              "display_provider_keys": false,
              "tech_support_email": null,
              "admin_support_email": null,
              "credit_card_support_email": null,
              "buyers_manage_apps": true,
              "buyers_manage_keys": true,
              "custom_keys_enabled": true,
              "buyer_plan_change_permission": "request",
              "buyer_can_select_plan": false,
              "notification_settings": null,
              "default_application_plan_id": 2357356012631,
              "default_service_plan_id": null,
              "default_end_user_plan_id": null,
              "end_user_registration_required": true,
              "tenant_id": 2445582571513,
              "system_name": "echo-api",
              "backend_version": "1",
              "mandatory_app_key": true,
              "buyer_key_regenerate_enabled": true,
              "support_email": "joaquim@redhat.com",
              "referrer_filters_required": false,
              "deployment_option": "self_managed",
              "kubernetes_service_link": null,
              "proxiable?": true,
              "backend_authentication_type": "service_token",
              "backend_authentication_value": "3d3bfe783a66ad7576c2389d4a8623ea613cc5146dce2e603b001ccac17e36f8",
              "proxy": {
                "id": 124012,
                "tenant_id": 2445582571513,
                "service_id": 2555417777820,
                "endpoint": "http://production.3scale.net:80",
                "deployed_at": null,
                "api_backend": "https://echo-api.3scale.net:443",
                "auth_app_key": "app_key",
                "auth_app_id": "app_id",
                "auth_user_key": "api-key",
                "credentials_location": "headers",
                "error_auth_failed": "Authentication failed",
                "error_auth_missing": "Authentication Missing",
                "created_at": "2019-03-19T09:01:01Z",
                "updated_at": "2019-03-19T09:04:38Z",
                "error_status_auth_failed": 403,
                "error_headers_auth_failed": "text/plain; charset=us-ascii",
                "error_status_auth_missing": 401,
                "error_headers_auth_missing": "text/plain; charset=us-ascii",
                "error_no_match": "No Mapping Rule matched",
                "error_status_no_match": 404,
                "error_headers_no_match": "text/plain; charset=us-ascii",
                "secret_token": "MySecretTokenBetweenApicastAndMyBackend_1237120312",
                "hostname_rewrite": "",
                "oauth_login_url": null,
                "sandbox_endpoint": "http://staging.3scale.net:80",
                "api_test_path": "/",
                "api_test_success": null,
                "apicast_configuration_driven": true,
                "oidc_issuer_endpoint": null,
                "lock_version": 6,
                "authentication_method": "1",
                "hostname_rewrite_for_sandbox": "echo-api.3scale.net",
                "endpoint_port": 80,
                "valid?": true,
                "service_backend_version": "1",
                "hosts": [
                  "production.3scale.net",
                  "staging.3scale.net"
                ],
                "backend": {
                  "endpoint": "https://su1.3scale.net",
                  "host": "su1.3scale.net"
                },
                "policy_chain": [
                  {
                    "name": "apicast",
                    "version": "builtin",
                    "configuration": {}
                  }
                ],
                "proxy_rules": [
                  {
                    "id": 375841,
                    "proxy_id": 124012,
                    "http_method": "GET",
                    "pattern": "/",
                    "metric_id": 2555418191879,
                    "metric_system_name": "get_slash",
                    "delta": 1,
                    "tenant_id": 2445582571513,
                    "created_at": "2019-03-19T09:04:35Z",
                    "updated_at": "2019-03-19T09:04:39Z",
                    "redirect_url": null,
                    "position": 1,
                    "last": false,
                    "parameters": [],
                    "querystring_parameters": {}
                  },
                  {
                    "id": 375842,
                    "proxy_id": 124012,
                    "http_method": "POST",
                    "pattern": "/",
                    "metric_id": 2555418191880,
                    "metric_system_name": "post_slash",
                    "delta": 1,
                    "tenant_id": 2445582571513,
                    "created_at": "2019-03-19T09:04:36Z",
                    "updated_at": "2019-03-19T09:04:39Z",
                    "redirect_url": null,
                    "position": 2,
                    "last": false,
                    "parameters": [],
                    "querystring_parameters": {}
                  }
                ]
              }
            }
          }
        },
        {
          "proxy_config": {
            "id": 93812,
            "version": 3,
            "environment": "production",
            "content": {
              "id": 2555417777820,
              "account_id": 2445582571513,
              "name": "echo-api",
              "oneline_description": null,
              "description": "Echo API",
              "txt_api": null,
              "txt_support": null,
              "txt_features": null,
              "created_at": "2019-03-19T09:01:01Z",
              "updated_at": "2019-03-19T16:25:40Z",
              "logo_file_name": null,
              "logo_content_type": null,
              "logo_file_size": null,
              "state": "incomplete",
              "intentions_required": false,
              "draft_name": "",
              "infobar": null,
              "terms": null,
              "display_provider_keys": false,
              "tech_support_email": null,
              "admin_support_email": null,
              "credit_card_support_email": null,
              "buyers_manage_apps": true,
              "buyers_manage_keys": true,
              "custom_keys_enabled": true,
              "buyer_plan_change_permission": "request",
              "buyer_can_select_plan": false,
              "notification_settings": null,
              "default_application_plan_id": 2357356012630,
              "default_service_plan_id": null,
              "default_end_user_plan_id": null,
              "end_user_registration_required": true,
              "tenant_id": 2445582571513,
              "system_name": "echo-api",
              "backend_version": "1",
              "mandatory_app_key": true,
              "buyer_key_regenerate_enabled": true,
              "support_email": "joaquim@redhat.com",
              "referrer_filters_required": false,
              "deployment_option": "self_managed",
              "kubernetes_service_link": null,
              "proxiable?": true,
              "backend_authentication_type": "service_token",
              "backend_authentication_value": "3d3bfe783a66ad7576c2389d4a8623ea613cc5146dce2e603b001ccac17e36f8",
              "proxy": {
                "id": 124012,
                "tenant_id": 2445582571513,
                "service_id": 2555417777820,
                "endpoint": "http://production.3scale.net:9999",
                "deployed_at": null,
                "api_backend": "https://echo-api.3scale.net:443",
                "auth_app_key": "app_key",
                "auth_app_id": "app_id",
                "auth_user_key": "api-key",
                "credentials_location": "headers",
                "error_auth_failed": "Authentication failed",
                "error_auth_missing": "Authentication Missing",
                "created_at": "2019-03-19T09:01:01Z",
                "updated_at": "2019-03-19T16:25:40Z",
                "error_status_auth_failed": 403,
                "error_headers_auth_failed": "text/plain; charset=us-ascii",
                "error_status_auth_missing": 401,
                "error_headers_auth_missing": "text/plain; charset=us-ascii",
                "error_no_match": "No Mapping Rule matched",
                "error_status_no_match": 404,
                "error_headers_no_match": "text/plain; charset=us-ascii",
                "secret_token": "MySecretTokenBetweenApicastAndMyBackend_1237120312",
                "hostname_rewrite": "",
                "oauth_login_url": null,
                "sandbox_endpoint": "http://staging.3scale.net:80",
                "api_test_path": "/",
                "api_test_success": null,
                "apicast_configuration_driven": true,
                "oidc_issuer_endpoint": null,
                "lock_version": 7,
                "authentication_method": "1",
                "hostname_rewrite_for_sandbox": "echo-api.3scale.net",
                "endpoint_port": 9999,
                "valid?": true,
                "service_backend_version": "1",
                "hosts": [
                  "production.3scale.net",
                  "staging.3scale.net"
                ],
                "backend": {
                  "endpoint": "https://su1.3scale.net",
                  "host": "su1.3scale.net"
                },
                "policy_chain": [
                  {
                    "name": "apicast",
                    "version": "builtin",
                    "configuration": {}
                  }
                ],
                "proxy_rules": [
                  {
                    "id": 375841,
                    "proxy_id": 124012,
                    "http_method": "GET",
                    "pattern": "/",
                    "metric_id": 2555418191879,
                    "metric_system_name": "get_slash",
                    "delta": 1,
                    "tenant_id": 2445582571513,
                    "created_at": "2019-03-19T09:04:35Z",
                    "updated_at": "2019-03-19T09:04:39Z",
                    "redirect_url": null,
                    "position": 1,
                    "last": false,
                    "parameters": [],
                    "querystring_parameters": {}
                  },
                  {
                    "id": 375842,
                    "proxy_id": 124012,
                    "http_method": "POST",
                    "pattern": "/",
                    "metric_id": 2555418191880,
                    "metric_system_name": "post_slash",
                    "delta": 1,
                    "tenant_id": 2445582571513,
                    "created_at": "2019-03-19T09:04:36Z",
                    "updated_at": "2019-03-19T09:04:39Z",
                    "redirect_url": null,
                    "position": 2,
                    "last": false,
                    "parameters": [],
                    "querystring_parameters": {}
                  }
                ]
              }
            }
          }
        }
      ]
    }"## }

    #[test]
    fn it_serializes() {
        let support_emails = SupportEmails {
            support: "some@example.com".into(),
            tech: None,
            admin: None,
            credit_card: None,
        };
        let configs = Configs::from(vec![
            Config {
                id: 375841,
                version: 1,
                environment: Environment::Production,
                content: Content {
                    id: 2555417777820,
                    account_id: 2555418191879,
                    description: Some("a config description".into()),
                    oneline_description: None,
                    logo_file: None,
                    support_emails: support_emails.clone(),
                    name: "a config".into(),
                    state: "unknown".into(),
                    draft_name: "draft".into(),
                    intentions_required: false,
                    buyer_settings: BuyerSettings {
                        buyer_can_select_plan: true,
                        buyer_key_regenerate_enabled: true,
                        buyer_plan_change_permission: "all".into(),
                        buyers_manage_apps: true,
                        buyers_manage_keys: true,
                    },
                    backend_authentication_type: BackendAuthentication::ProviderKey(
                        "aproviderkey".into(),
                    ),
                    end_user_registration_required: false,
                    backend_version: AuthenticationMode::AppIdKey,
                    custom_keys_enabled: false,
                    default_plans: DefaultPlans {
                        application_plan_id: 1234.into(),
                        end_user_plan_id: None,
                        service_plan_id: None,
                    },
                    tenant_id: 101010,
                    deployment_option: DeploymentOption::SelfManaged,
                    display_provider_keys: true,
                    proxiable: true,
                    system_name: "my_config1".into(),
                    mandatory_app_key: false,
                    referrer_filters_required: false,
                },
            },
            Config {
                id: 375841,
                version: 2,
                environment: Environment::Production,
                content: Content {
                    id: 2555417777821,
                    account_id: 2555418191879,
                    description: Some("a config description".into()),
                    oneline_description: None,
                    logo_file: None,
                    support_emails: support_emails.clone(),
                    name: "a config".into(),
                    state: "unknown".into(),
                    draft_name: "draft".into(),
                    intentions_required: false,
                    buyer_settings: BuyerSettings {
                        buyer_can_select_plan: true,
                        buyer_key_regenerate_enabled: true,
                        buyer_plan_change_permission: "all".into(),
                        buyers_manage_apps: true,
                        buyers_manage_keys: true,
                    },
                    backend_authentication_type: BackendAuthentication::ProviderKey(
                        "aproviderkey".into(),
                    ),
                    end_user_registration_required: false,
                    backend_version: AuthenticationMode::AppIdKey,
                    custom_keys_enabled: false,
                    default_plans: DefaultPlans {
                        application_plan_id: 1234.into(),
                        end_user_plan_id: None,
                        service_plan_id: None,
                    },
                    tenant_id: 101010,
                    deployment_option: DeploymentOption::SelfManaged,
                    display_provider_keys: true,
                    proxiable: true,
                    system_name: "my_config1".into(),
                    mandatory_app_key: false,
                    referrer_filters_required: false,
                },
            },
        ]);
        let result = serde_json::to_string_pretty(&configs);
        match result {
            Err(ref e) => println!("Error: {:#?}", e),
            _ => (),
        }
        assert!(result.is_ok());
        println!("{}", result.unwrap());
    }
}
