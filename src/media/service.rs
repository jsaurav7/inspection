use rocket::State;

use crate::error::Error;
use crate::{AppState, DB};
use crate::media::photo_details::PhotoDetails;
use crate::user_details::User;

#[tracing::instrument(name = "service::get_report", skip(db, state))]
pub async fn get_report(
    db: &DB,
    state: &State<AppState>,
    download_key: &str,
) -> Result<(i32, Vec<u8>), Error> {
    let case_id = db.get_case_id(download_key).await?;

    state
        .s3_client
        .get_file(format!("cases/{case_id}/{case_id}.pdf"))
        .await
        .map(|file| (case_id, file))
}

#[tracing::instrument(name = "service::get_photos", skip(db, state))]
pub async fn get_photos(
    db: &DB,
    state: &State<AppState>,
    download_key: &str,
) -> Result<(i32, Vec<u8>), Error> {
    let case_id = db.get_case_id(download_key).await?;

    state
        .s3_client
        .get_file(format!("cases/{case_id}/{case_id}.zip"))
        .await
        .map(|file| (case_id, file))
}

#[tracing::instrument(name = "service::get_photos_details", skip(db, user))]
pub async fn get_photos_details(
    db: &DB,
    user: &User,
    case_id: i32,
) -> Result<Vec<PhotoDetails>, Error> {
    let download_key = db.get_download_key(case_id, &user.company_name).await?;
    let mut photos = db.get_photos_details(case_id, &download_key).await?;
    let video = db.get_video_details(case_id, &download_key).await?;

    let report = PhotoDetails {
        snap_time: None,
        file_name: format!("{case_id}.pdf"),
        photo_type: "report".to_string(),
        latitude: None,
        longitude: None,
        comment: None,
        url: format!("https://inspection.wimwisure.com/v2/case/download/report/{download_key}")
    };

    photos.push(report);
    photos.push(video);

    Ok(photos)
}
