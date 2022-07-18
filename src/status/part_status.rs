#[derive(serde::Serialize, sqlx::FromRow)]
#[serde(rename_all = "PascalCase")]
pub struct PartStatus {
    pub name: String,
    pub status: Option<String>,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct VehicleStatus {
    pub case_id: i32,
    pub status: String,
    pub inspection_time: Option<String>,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct QuoteStatus {
    pub status: String,
    pub updated_at: Option<String>,
}
