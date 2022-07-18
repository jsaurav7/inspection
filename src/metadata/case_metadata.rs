
#[derive(serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CaseMetadata {
    pub fuel_types: Vec<Metadata>,
    pub vehicle_types: Vec<Metadata>,
    pub purpose_of_inspections: Vec<Metadata>,
    pub payment_methods: Vec<Metadata>,
}

#[derive(serde::Serialize, sqlx::FromRow)]
#[serde(rename_all = "PascalCase")]
pub struct Metadata {
    #[serde(rename = "ID")] pub id: String,
    pub name: String,
}
