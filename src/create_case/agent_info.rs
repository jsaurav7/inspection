use serde::Serialize;

#[derive(Serialize, sqlx::FromRow, Default)]
pub struct AgentInfo {
    pub name: String,
    pub phone: String,
    pub email: String,
}
