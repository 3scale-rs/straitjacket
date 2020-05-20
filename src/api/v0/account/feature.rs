use serde::{Deserialize, Serialize};
use straitjacket_macro::straitjacket;

pub type Metadata = crate::resources::Metadata;

#[straitjacket]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Feature {
    id: String,
    name: String,
    system_name: Option<String>,
    account_id: String,
    scope: String,
    visible: bool,
    description: Option<String>,
}
