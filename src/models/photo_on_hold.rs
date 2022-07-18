#[derive(serde::Serialize, sqlx::FromRow)]
#[serde(rename_all = "PascalCase")]
pub struct PhotoOnHold {
    pub photo_type_id: String,
    pub comment: String,
}
