use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub data: Data,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub object: StripeObject,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StripeObject {
    pub id: String,
    pub object: String,
    pub adaptive_pricing: AdaptivePricing,
    pub amount_subtotal: i64,
    pub amount_total: i64,
    pub automatic_tax: AutomaticTax,
    pub custom_fields: Vec<CustomField>,
    pub customer_details: CustomerDetails,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdaptivePricing {
    pub enabled: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AutomaticTax {
    pub enabled: bool,
    pub liability: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomField {
    pub key: String,
    pub label: Label,
    pub text: Option<TextValue>,
    pub dropdown: Option<Dropdown>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Label {
    pub custom: String,
    pub r#type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TextValue {
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Dropdown {
    pub options: Vec<DropdownOption>,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DropdownOption {
    pub label: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomerDetails {
    pub address: Address,
    pub email: String,
    pub name: String,
    pub phone: Option<String>,
    pub tax_exempt: String,
    pub tax_ids: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Address {
    pub city: Option<String>,
    pub country: String,
    pub line1: Option<String>,
    pub line2: Option<String>,
    pub postal_code: Option<String>,
    pub state: Option<String>,
}

// Response type
pub type Response = ();
