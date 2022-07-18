use serde::Serialize;

#[derive(Serialize, sqlx::FromRow)]
pub struct CustomerInfo {
    pub name: String,
    pub phone: String,
    pub vehicle_number: String,
    pub insurance_company: String,
    pub support_phone: String,
}
