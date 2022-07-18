use crate::models::photo_on_hold::PhotoOnHold;

#[derive(serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CallbackDetails {
    #[serde(rename = "ID")]
    pub id: i32,
    pub inspection_number: Option<String>,
    pub quote_number: Option<String>,
    pub remarks: Option<String>,
    pub status: String,
    pub photos_on_hold: Vec<PhotoOnHold>,
    pub comment: Option<String>,
    #[serde(skip_serializing)]
    pub url: String,
}
