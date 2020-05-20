use serde::{Deserialize, Serialize};
use straitjacket_macro::straitjacket;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Metadata;

#[straitjacket]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
// A service method
struct Method {
    id: String,
    name: String,
    system_name: String,
    friendly_name: String,
    service_id: String, // a service id
    description: String,
    metric_id: String,
}
