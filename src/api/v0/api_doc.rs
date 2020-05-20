use serde::{Deserialize, Serialize};
use straitjacket_macro::straitjacket;

pub type Metadata = crate::resources::Metadata;

#[straitjacket]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiDoc {
    id: String,
    system_name: String,
    name: String,
    published: bool,
    skip_swagger_validations: bool,
    body: String,
}
