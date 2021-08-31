use serde::{Deserialize, Deserializer, Serialize, Serializer};
use straitjacket_macro::straitjacket;

use crate::resources::Metadata;

pub mod configs;
pub mod mapping_rules;

pub(self) fn parse_url<'de, D: Deserializer<'de>>(deserializer: D) -> Result<url::Url, D::Error> {
    let string: String = Deserialize::deserialize(deserializer)?;
    let url = url::Url::parse(&string);
    url.map_err(serde::de::Error::custom)
}

pub(self) fn parse_url_opt<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<url::Url>, D::Error> {
    let string: Result<String, _> = Deserialize::deserialize(deserializer);
    if let Err(_) = string {
        return Ok(None);
    }
    let string = string.unwrap();
    let url = url::Url::parse(&string);
    url.map(|u| Some(u)).map_err(serde::de::Error::custom)
}

pub(self) fn serialize_url<S: Serializer>(
    url: &url::Url,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    let url_s = url.to_string();
    serializer.serialize_str(&url_s)
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Backend {
    #[serde(deserialize_with = "parse_url", serialize_with = "serialize_url")]
    endpoint: url::Url,
    host: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CredentialsLocation {
    Headers,
    Query,
    Authorization,
    #[serde(other)]
    Unknown,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorConfig {
    #[serde(rename = "error_auth_failed")]
    auth_failed: Option<String>,
    #[serde(rename = "error_auth_missing")]
    auth_missing: Option<String>,
    #[serde(rename = "error_status_auth_failed")]
    status_auth_failed: Option<u64>,
    #[serde(rename = "error_status_auth_missing")]
    status_auth_missing: Option<u64>,
    // These four fields below are Content-Type values
    #[serde(rename = "error_headers_auth_failed")]
    headers_auth_failed: Option<String>,
    #[serde(rename = "error_headers_auth_missing")]
    headers_auth_missing: Option<String>,
    #[serde(rename = "error_headers_no_match")]
    headers_no_match: Option<String>,
    #[serde(rename = "error_headers_limits_exceeded")]
    headers_limits_exceeded: Option<String>,
    #[serde(rename = "error_no_match")]
    no_match: Option<String>,
    #[serde(rename = "error_status_no_match")]
    status_no_match: Option<u64>,
    #[serde(rename = "error_status_limits_exceeded")]
    status_limits_exceeded: Option<u64>,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum JWTClaimClientIDType {
    Plain,
    Liquid,
    #[serde(other)]
    Unknown,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JWTClaim {
    #[serde(rename = "jwt_claim_with_client_id")]
    client_id: Option<String>,
    #[serde(rename = "jwt_claim_with_client_id_type")]
    client_type: Option<JWTClaimClientIDType>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OIDCIssuer {
    #[serde(rename = "oidc_issuer_endpoint")]
    endpoint: Option<String>,
    #[serde(rename = "oidc_issuer_type")]
    r#type: Option<String>,
}

#[straitjacket]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Proxy {
    id: u64,
    tenant_id: u64,
    service_id: u64,
    #[serde(deserialize_with = "parse_url", serialize_with = "serialize_url")]
    endpoint: url::Url,
    deployed_at: Option<String>,
    #[serde(deserialize_with = "parse_url", serialize_with = "serialize_url")]
    api_backend: url::Url,
    auth_app_key: String,
    auth_app_id: String,
    auth_user_key: String,
    credentials_location: CredentialsLocation,
    #[serde(flatten)]
    error_config: ErrorConfig,
    secret_token: String,
    hostname_rewrite: String,
    #[serde(deserialize_with = "parse_url_opt", skip_serializing)]
    oauth_login_url: Option<url::Url>,
    #[serde(deserialize_with = "parse_url_opt", skip_serializing)]
    sandbox_endpoint: Option<url::Url>,
    api_test_path: String,
    apicast_configuration_driven: bool,
    lock_version: u64,
    hostname_rewrite_for_sandbox: String,
    #[serde(flatten)]
    oidc_issuer: Option<OIDCIssuer>,
    #[serde(flatten)]
    jwt_claim: Option<JWTClaim>,
    endpoint_port: u64,
    #[serde(rename = "valid?")]
    valid: bool,
    // TODO these look like enums
    authentication_method: super::AuthenticationMode,
    service_backend_version: String,
    hosts: Vec<String>,
    backend: Backend,
    proxy_rules: Vec<self::mapping_rules::MappingRule>,
    // These below are currently ignored:
    //"api_test_success": null,
    //"policy_chain": [
    //  {
    //    "name": "apicast",
    //    "version": "builtin",
    //    "configuration": {}
    //  }
    //],
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses() {
        let proxy_json = r#"{
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
        }"#;

        let proxy = serde_json::from_str::<Proxy>(proxy_json);
        if let Err(e) = &proxy {
            println!("Error: {:#?}", e);
        }
        assert!(proxy.is_ok())
    }
}
