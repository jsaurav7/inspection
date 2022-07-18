#[derive(serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct PhotoDetails {
    pub snap_time: Option<String>,
    pub file_name: String,
    pub photo_type: String,
    pub latitude: Option<String>,
    pub longitude: Option<String>,
    pub comment: Option<String>,
    pub url: String,
}