#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCaseRequest {
    pub vehicle_number: String,
    pub agent_phone_number: Option<String>,
    #[serde(default = "default_customer_name")]
    pub customer_name: String,
    #[serde(default = "default_customer_phone")]
    pub customer_phone_number: String,
    #[serde(default = "default_customer_email")]
    pub customer_email: String,
    #[serde(default)]
    pub engine_number: String,
    #[serde(default)]
    pub chassis_number: String,
    #[serde(default = "default_vehicle_type")]
    pub vehicle_type: String,
    #[serde(default = "default_fuel_type")]
    pub fuel_type: String,
    #[serde(default)]
    pub quote_number: String,
    #[serde(default = "default_bool")]
    pub inspection_by_wimwisure: bool,
    #[serde(default = "default_bool")]
    pub communication_by_wimwisure: bool,
    #[serde(default)]
    pub notify_email: Vec<String>,
    #[serde(default)]
    pub notify_phone: Vec<String>,
    #[serde(default)]
    pub inspectors: Vec<String>,
    #[serde(default)]
    pub inspection_number: String,
    #[serde(default = "default_paid_by")]
    pub paid_by: String,
    #[serde(default = "default_purpose")]
    pub purpose_of_inspection: String,
    #[serde(rename = "callbackURL", default)]
    pub callback_url: String,
}

fn default_paid_by() -> String {
    String::from("PAID_BY_COMPANY")
}

fn default_purpose() -> String {
    String::from("break_in")
}

fn default_customer_name() -> String {
    String::from("Customer")
}

fn default_fuel_type() -> String {
    String::from("petrol")
}

fn default_bool() -> bool {
    true
}

fn default_customer_phone() -> String {
    String::from("invalid")
}

fn default_vehicle_type() -> String {
    String::from("2-wheeler")
}

fn default_customer_email() -> String {
    String::from("invalid@wimwisure.com")
}
