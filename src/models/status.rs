#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    success: bool,
}

impl Status {
    pub fn success() -> Self {
        Self { success: true }
    }
}
