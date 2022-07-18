use crate::models::inspector::Inspector;

#[derive(serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CaseInfo {
    #[serde(rename = "ID")]
    pub id: i32,
    pub customer_name: String,
    pub customer_phone_number: String,
    pub customer_email: String,
    pub vehicle_number: String,
    pub fuel_type: String,
    pub quote_number: String,
    pub inspection_by_wimwisure: bool,
    pub communication_by_wimwisure: bool,
    pub status: String,
    pub creation_time: Option<String>,
    pub inspection_time: Option<String>,
    pub inspection_number: String,
    pub chassis_number: String,
    pub engine_number: String,
    pub remarks: Option<String>,
    pub comment: Option<String>,
    pub inspection_link: Option<String>,
    pub download_key: String,
    #[serde(rename = "callbackURL")]
    pub callback_url: String,
    pub inspection_by: Inspector,
    pub inspectors: Vec<Inspector>,
    pub app_version: String,
    pub qc_time: Option<String>,
    pub latest_comment: Comment,
    pub required_photos: RequiredPhotos,
    pub notify_email: Vec<String>,
    pub notify_phone: Vec<String>,
    pub report_link: String,
    pub photos_download_link: String,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct RequiredPhotos {
    pub transaction_id: String,
    pub photos: Vec<Photo>,
}

#[derive(serde::Serialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct Comment {
    pub comment: String,
    pub timestamp: String,
}

#[derive(serde::Serialize, sqlx::FromRow)]
#[serde(rename_all = "PascalCase")]
pub struct Photo {
    pub id: String,
    pub name: String,
    pub group_name: String,
    pub input_type: String,
    pub multiple: bool,
    pub comment: String,
}
