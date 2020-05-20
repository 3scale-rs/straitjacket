use serde::{Deserialize, Serialize};
use straitjacket_macro::straitjacket;

pub type Metadata = crate::resources::Metadata;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AuthenticationProviderKind {
    #[serde(rename = "auth0")]
    Auth0,
    #[serde(rename = "keycloak")]
    KeyCloak,
}

#[straitjacket]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthenticationProvider {
    kind: AuthenticationProviderKind,
    name: String,
    system_name: String,
    client_id: String,
    client_secret: String,
    site: String,
    skip_ssl_certificate_verification: bool,
    published: bool,
}
