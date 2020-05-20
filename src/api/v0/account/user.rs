use serde::{Deserialize, Serialize};
use straitjacket_macro::straitjacket;

pub type Metadata = crate::resources::Metadata;

#[straitjacket]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    id: String,
    account_id: String,
    state: String,
    role: String,
    username: String,
    email: String,
    extra_fields: Vec<String>,
}
