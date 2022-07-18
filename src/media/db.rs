use rocket_db_pools::sqlx;
use rocket_db_pools::sqlx::Row;
use sqlx::types::chrono::NaiveDateTime;
use sqlx::types::Decimal;

use crate::db::DB;
use crate::error::Error;
use crate::media::photo_details::PhotoDetails;

impl DB {
    #[tracing::instrument(name = "db::get_case_id", skip(self))]
    pub async fn get_case_id(&self, download_key: &str) -> Result<i32, Error> {
        sqlx::query("SELECT id FROM cases WHERE download_key = ?")
            .bind(download_key)
            .fetch_one(&self.0)
            .await
            .and_then(|r| Ok(r.get("id")))
            .map_err(|e| e.into())
    }

    #[tracing::instrument(name = "db::get_download_key", skip(self))]
    pub async fn get_download_key(&self, case_id: i32, company: &str) -> Result<String, Error> {
        sqlx::query("SELECT download_key FROM cases WHERE id = ? AND insurance_company_id = ?")
            .bind(case_id)
            .bind(company)
            .fetch_one(&self.0)
            .await
            .and_then(|r| Ok(r.get("download_key")))
            .map_err(|e| e.into())
    }

    #[tracing::instrument(name = "db::get_photos_details", skip(self))]
    pub async fn get_photos_details(
        &self,
        case_id: i32,
        download_key: &str,
    ) -> Result<Vec<PhotoDetails>, Error> {
        sqlx::query("SELECT snap_time, file_name, photo_type, latitude, longitude, qc_answer FROM cases_photos WHERE case_id = ? AND photo_type != 'video'")
            .bind(case_id)
            .fetch_all(&self.0)
            .await
            .and_then(|r| {
                Ok(r.iter().map(|row| {
                    let snap_time: Option<NaiveDateTime> = row.get("snap_time");
                    let latitude: Option<Decimal> = row.get("latitude");
                    let longitude: Option<Decimal> = row.get("longitude");
                    let photo_type: String = row.get("photo_type");
                    let file_name: String = row.get("file_name");

                    PhotoDetails {
                        snap_time: snap_time.map(|t| t.to_string()),
                        url: format!("https://qc.wimwisure.com/util/case-image/{case_id}/{file_name}"),
                        file_name,
                        photo_type,
                        latitude: latitude.map(|l| l.to_string()),
                        longitude: longitude.map(|l| l.to_string()),
                        comment: row.get("qc_answer"),
                    }
                }).collect::<Vec<PhotoDetails>>())
            })
            .map_err(|e| {
                tracing::error!("Error fetching photos details {:?}", e);
                e.into()
            })
    }

    #[tracing::instrument(name = "db::get_video_details", skip(self))]
    pub async fn get_video_details(
        &self,
        case_id: i32,
        download_key: &str,
    ) -> Result<PhotoDetails, Error> {
        sqlx::query("SELECT snap_time, file_name, photo_type, latitude, longitude, qc_answer FROM cases_photos WHERE case_id = ? and photo_type = 'video' ORDER by snap_time DESC")
            .bind(case_id)
            .fetch_one(&self.0)
            .await
            .and_then(|row| {
                    let snap_time: Option<NaiveDateTime> = row.get("snap_time");
                    let latitude: Option<Decimal> = row.get("latitude");
                    let longitude: Option<Decimal> = row.get("longitude");
                    let photo_type: String = row.get("photo_type");
                    let file_name: String = row.get("file_name");

                    Ok(PhotoDetails {
                        snap_time: snap_time.map(|t| t.to_string()),
                        url: format!("https://qc.wimwisure.com/util/cases/download-uploaded-video/{download_key}/{file_name}"),
                        file_name,
                        photo_type,
                        latitude: latitude.map(|l| l.to_string()),
                        longitude: longitude.map(|l| l.to_string()),
                        comment: row.get("qc_answer"),
                    })
            })
            .map_err(|e| {
                tracing::error!("Error fetching photos details {:?}", e);
                e.into()
            })
    }
}
