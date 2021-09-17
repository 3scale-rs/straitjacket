use serde::{Deserialize, Serialize};
use straitjacket_macro::straitjacket;

use crate::resources::Metadata;

use super::super::{AuthenticationMode, DeploymentOption};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Environment {
    Staging,
    Production,
    #[serde(other)]
    Unknown,
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
    proxy: super::Proxy,
    // Note that the structure is incomplete as some fields meaning/usage and even their types are unknown to me.
    // txt_api: ???,
    // txt_support: ???,
    // txt_features: ???,
    // infobar: ???,
    // terms: ???,
    // notification_settings: ???,
    // kubernetes_service_link: ???,
}

impl Content {
    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn account_id(&self) -> u64 {
        self.account_id
    }

    pub fn tenant_id(&self) -> u64 {
        self.tenant_id
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn system_name(&self) -> &str {
        self.system_name.as_str()
    }

    pub fn draft_name(&self) -> &str {
        self.draft_name.as_str()
    }

    pub fn state(&self) -> &str {
        self.state.as_str()
    }

    pub fn backend_version(&self) -> AuthenticationMode {
        self.backend_version
    }

    pub fn mandatory_app_key(&self) -> bool {
        self.mandatory_app_key
    }

    pub fn deployment_option(&self) -> DeploymentOption {
        self.deployment_option
    }

    pub fn is_proxiable(&self) -> bool {
        self.proxiable
    }

    pub fn backend_authentication(&self) -> &BackendAuthentication {
        &self.backend_authentication_type
    }

    pub fn proxy(&self) -> &super::Proxy {
        &self.proxy
    }
}

#[straitjacket(name_snake = "proxy_config", plural_snake = "proxy_configs")]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    id: u64,
    version: u64,
    environment: Environment,
    content: Content,
}

impl Config {
    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn version(&self) -> u64 {
        self.version
    }

    pub fn environment(&self) -> &Environment {
        &self.environment
    }

    pub fn content(&self) -> &Content {
        &self.content
    }
}

endpoint! { LIST, GET joining [ "/admin/api/services/", "/proxy/configs/", ".json" ] returning Configs }
endpoint! { LATEST, GET joining [ "/admin/api/services/", "/proxy/configs/", "/latest.json" ] returning Config }

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;

    mod list {
        use super::*;

        endpoint_test! { it_parses, LIST, r#"{
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
        }"# }

        #[test]
        fn it_serializes() {
            let support_emails = SupportEmails {
                support: "some@example.com".into(),
                tech: None,
                admin: None,
                credit_card: None,
            };
            let content = Content {
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
                proxy: Proxy {
                    id: 1,
                    tenant_id: 2,
                    service_id: 3,
                    api_backend: url::Url::parse("https://3scale.net")
                        .expect("failed to parse url"),
                    endpoint: url::Url::parse("https://3scale.net").expect("failed to parse url"),
                    hostname_rewrite: "/".into(),
                    authentication_method: AuthenticationMode::APIKey,
                    auth_user_key: "abc".into(),
                    apicast_configuration_driven: false,
                    api_test_path: "/test".into(),
                    backend: Backend {
                        endpoint: url::Url::parse("https://su1.3scale.net")
                            .expect("failed to parse url"),
                        host: "su1.3scale.net".into(),
                    },
                    credentials_location: CredentialsLocation::Query,
                    deployed_at: Some("preview-cluster".into()),
                    endpoint_port: 80,
                    error_config: ErrorConfig {
                        auth_failed: Some("auth failed".into()),
                        auth_missing: Some("auth missing".into()),
                        no_match: Some("no match".into()),
                        headers_auth_failed: Some("text/plain; charset=us-ascii".into()),
                        headers_auth_missing: Some("text/plain; charset=us-ascii".into()),
                        headers_limits_exceeded: Some("text/plain; charset=us-ascii".into()),
                        headers_no_match: Some("text/plain; charset=us-ascii".into()),
                        status_auth_failed: Some(403),
                        status_auth_missing: Some(403),
                        status_limits_exceeded: Some(428),
                        status_no_match: Some(400),
                    },
                    auth_app_id: "anid".into(),
                    auth_app_key: "akey".into(),
                    hosts: vec![],
                    jwt_claim: None,
                    hostname_rewrite_for_sandbox: "hostname".into(),
                    lock_version: 1,
                    oauth_login_url: None,
                    oidc_issuer: None,
                    sandbox_endpoint: None,
                    secret_token: "atoken".into(),
                    service_backend_version: "1".into(),
                    valid: true,
                    proxy_rules: vec![mapping_rules::MappingRule {
                        id: 1,
                        metric_id: 1,
                        delta: 1,
                        http_method: "POST".into(),
                        pattern: "/".into(),
                        last: false,
                        metric_system_name: None,
                        position: 1,
                        redirect_url: None,
                        tenant_id: None,
                    }],
                },
            };
            let content_clone = content.clone();

            let configs = Configs::from(vec![
                Config {
                    id: 375841,
                    version: 1,
                    environment: Environment::Production,
                    content,
                },
                Config {
                    id: 375841,
                    version: 2,
                    environment: Environment::Production,
                    content: content_clone,
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

    mod latest {
        use super::*;

        endpoint_test! { it_parses, LATEST, r#"{
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
          }
        }"# }

        #[test]
        fn it_serializes() {
            use super::super::*;

            let support_emails = SupportEmails {
                support: "some@example.com".into(),
                tech: None,
                admin: None,
                credit_card: None,
            };
            let content = Content {
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
                proxy: Proxy {
                    id: 1,
                    tenant_id: 2,
                    service_id: 3,
                    api_backend: url::Url::parse("https://3scale.net")
                        .expect("failed to parse url"),
                    endpoint: url::Url::parse("https://3scale.net").expect("failed to parse url"),
                    hostname_rewrite: "/".into(),
                    authentication_method: AuthenticationMode::APIKey,
                    auth_user_key: "abc".into(),
                    apicast_configuration_driven: false,
                    api_test_path: "/test".into(),
                    backend: Backend {
                        endpoint: url::Url::parse("https://su1.3scale.net")
                            .expect("failed to parse url"),
                        host: "su1.3scale.net".into(),
                    },
                    credentials_location: CredentialsLocation::Query,
                    deployed_at: Some("preview-cluster".into()),
                    endpoint_port: 80,
                    error_config: ErrorConfig {
                        auth_failed: Some("auth failed".into()),
                        auth_missing: Some("auth missing".into()),
                        no_match: Some("no match".into()),
                        headers_auth_failed: Some("text/plain; charset=us-ascii".into()),
                        headers_auth_missing: Some("text/plain; charset=us-ascii".into()),
                        headers_limits_exceeded: Some("text/plain; charset=us-ascii".into()),
                        headers_no_match: Some("text/plain; charset=us-ascii".into()),
                        status_auth_failed: Some(403),
                        status_auth_missing: Some(403),
                        status_limits_exceeded: Some(428),
                        status_no_match: Some(400),
                    },
                    auth_app_id: "anid".into(),
                    auth_app_key: "akey".into(),
                    hosts: vec![],
                    jwt_claim: None,
                    hostname_rewrite_for_sandbox: "hostname".into(),
                    lock_version: 1,
                    oauth_login_url: None,
                    oidc_issuer: None,
                    sandbox_endpoint: None,
                    secret_token: "atoken".into(),
                    service_backend_version: "1".into(),
                    valid: true,
                    proxy_rules: vec![mapping_rules::MappingRule {
                        id: 1,
                        metric_id: 1,
                        delta: 1,
                        http_method: "POST".into(),
                        pattern: "/".into(),
                        last: false,
                        metric_system_name: None,
                        position: 1,
                        redirect_url: None,
                        tenant_id: None,
                    }],
                },
            };

            let config = Config {
                id: 375841,
                version: 1,
                environment: Environment::Production,
                content,
            };
            let result = serde_json::to_string_pretty(&config);
            match result {
                Err(ref e) => println!("Error: {:#?}", e),
                _ => (),
            }
            assert!(result.is_ok());
            println!("{}", result.unwrap());
        }
    }
}
