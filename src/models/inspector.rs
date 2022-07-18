use serde::Serialize;

#[derive(Serialize, sqlx::FromRow)]
#[serde(rename_all = "PascalCase")]
pub struct Inspector {
    pub phone_number: String,
    pub inspection_link: String,
}
