use super::metric::Metric;
use serde::{Deserialize, Serialize};
use straitjacket_macro::straitjacket;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
enum DeploymentOption {
    Hosted,
    SelfManaged,
    ServiceMeshIstio,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
enum AuthenticationMode {
    #[serde(rename = "1")]
    APIKey,
    #[serde(rename = "2")]
    AppIdKey,
    #[serde(rename = "oidc")]
    OIDC,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Metadata;

#[straitjacket]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Service {
    id: String,
    account_id: String,
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
    metrics: Option<Vec<Metric>>,
}
