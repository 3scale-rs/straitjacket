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
    id: u64,
    account_id: u64,
    kind: AuthenticationProviderKind,
    automatically_approve_accounts: bool,
    authorize_url: String,
    name: String,
    system_name: String,
    client_id: String,
    client_secret: String,
    site: String,
    skip_ssl_certificate_verification: bool,
    published: bool,
    username_key: String,
    account_type: String,
    callback_url: String,
    identifier_key: String,
    trust_email: bool,
}

endpoint! { EP_LIST_AUTHN_PROVIDER_ADMIN, GET joining [ "/admin/api/account/authentication_providers.json"] returning AuthenticationProviders }
endpoint_test! { it_parses, EP_LIST_AUTHN_PROVIDER_ADMIN, r##"{
   "authentication_providers" : [
      {
         "authentication_provider" : {
            "system_name" : "keycloak_4c4449b6fed8",
            "authorize_url" : "https://secret.com/protocol/openid-connect/auth?client_id=any&redirect_uri=https%3A%2F%2Fistiodevel-admin.3scale.net%2Fauth%2Fkeycloak_4c4449b6fed8%2Fcallback%3Faccess_token%3D6a8bd22d84c65e3574ccf82f54d10ee13ef28f11c3b68a69f6c68d52869b800d&response_type=code&scope=openid",
            "automatically_approve_accounts" : false,
            "created_at" : "2020-05-22T15:17:19+01:00",
            "id" : 17423,
            "published" : false,
            "username_key" : "login",
            "kind" : "keycloak",
            "name" : "Red Hat Single Sign-On",
            "account_type" : "provider",
            "updated_at" : "2020-05-22T15:17:19+01:00",
            "account_id" : 2445582571513,
            "client_id" : "any",
            "skip_ssl_certificate_verification" : false,
            "identifier_key" : "id",
            "trust_email" : false,
            "site" : "https://secret.com",
            "callback_url" : "https://istiodevel-admin.3scale.net/auth/keycloak_4c4449b6fed8/callback",
            "client_secret" : "secret"
         }
      }
   ]
}"## }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_serializes() {
        let ap = AuthenticationProviders::from(vec![
            AuthenticationProvider {
                id: 2639696107074,
                account_id: 1,
                kind: AuthenticationProviderKind::KeyCloak,
                automatically_approve_accounts: false,
                authorize_url: "https://secret.com/protocol/openid-connect/auth/?client_id=any&redirect_uri=https%3A%2F%2Fistiodevel-admin.3scale.net%2Fauth%2Fkeycloak_4c4449b6fed8%2Fcallback%3Faccess_token%3D6a8bd22d84c65e3574ccf82f54d10ee13ef28f11c3b68a69f6c68d52869b800d&response_type=code&scope=openid".to_string(),
                name: "Red Hat Single Sign-On".to_string(),
                system_name: "sys_name".to_string(),
                client_id: "c".to_string(),
                client_secret: "s".to_string(),
                site: "https://secret.com".to_string(),
                skip_ssl_certificate_verification: false,
                published: false,
                username_key: "any".to_string(),
                account_type: "any".to_string(),
                callback_url: "any".to_string(),
                identifier_key: "any".to_string(),
                trust_email: false
            },
        ]);
        println!("{}", serde_json::to_string_pretty(&ap).unwrap());
    }
}
