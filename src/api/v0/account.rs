use serde::{Deserialize, Serialize};
use straitjacket_macro::straitjacket;

pub mod feature;
pub mod plan;
pub mod user;

pub type Metadata = crate::resources::Metadata;

#[straitjacket]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Account {
    id: String,
    state: String,
    org_name: String,
    extra_fields: Vec<String>,
    monthly_billing_enabled: bool,
    monthly_charging_enabled: bool,
    credit_card_stored: bool,
    plans: Vec<plan::Plan>,
    users: Vec<user::User>,
}
