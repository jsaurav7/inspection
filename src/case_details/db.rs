use rocket_db_pools::sqlx;
use rocket_db_pools::sqlx::Row;
use sqlx::types::chrono::NaiveDateTime;

use crate::case_details::case_info::{CaseInfo, Comment, Photo, RequiredPhotos};
use crate::db::DB;
use crate::error::Error;
use crate::models::inspector::Inspector;
use crate::user_details::User;

impl DB {
    #[tracing::instrument(name = "db::get_case_info", skip(self, user))]
    pub async fn get_case_info(&self, user: &User, case_id: i32) -> Result<CaseInfo, Error> {
        let required_photos = self.get_case_required_photos(case_id).await?;
        let latest_comment = self.get_latest_comment(case_id).await.unwrap_or_default();
        let inspectors = self.get_case_inspectors(case_id).await?;
        let notification_users = self.get_notification_users(case_id).await?;

        sqlx::query(GET_CASE_DETAILS)
            .bind(case_id)
            .bind(&user.company_name)
            .fetch_one(&self.0)
            .await
            .and_then(|r| {
                let inspection_time: Option<NaiveDateTime> = r.get("inspection_time");
                let qc_time: Option<NaiveDateTime> = r.get("qc_time");
                let creation_time: Option<NaiveDateTime> = r.get("creation_time");
                let download_key: String = r.get("download_key");

                Ok(CaseInfo {
                    id: r.get("id"),
                    customer_name: r.get("customer_name"),
                    customer_phone_number: r.get("customer_phone"),
                    customer_email: r.get("customer_email"),
                    vehicle_number: r.get("vehicle_number"),
                    fuel_type: r.get("vehicle_fuel_type"),
                    quote_number: r.get("quote_number"),
                    inspection_by_wimwisure: r.get("inspection_by_wimwisure"),
                    communication_by_wimwisure: r.get("communication"),
                    status: r.get("status"),
                    creation_time: creation_time.map(|t| t.to_string()),
                    inspection_time: inspection_time.map(|t| t.to_string()),
                    inspection_number: r.get("inspection_number"),
                    chassis_number: r.get("chassis_number"),
                    engine_number: r.get("engine_number"),
                    remarks: r.get("remark"),
                    comment: r.get("comment"),
                    inspection_link: r.get("inspection_link"),
                    callback_url: r.get("callback_url"),
                    inspection_by: Inspector { phone_number: "".to_string(), inspection_link: "".to_string() },
                    inspectors,
                    app_version: r.get("app_version"),
                    qc_time: qc_time.map(|t| t.to_string()),
                    latest_comment,
                    required_photos,
                    notify_email: notification_users.emails,
                    notify_phone: notification_users.phones,
                    photos_download_link: format!("https://inspection.wimwisure.com/v2/case/download/photos/{download_key}"),
                    report_link: format!("https://inspection.wimwisure.com/v2/case/download/report/{download_key}"),
                    download_key,
                })
            })
            .map_err(|e| {
                error!("Error fetching case info: {:?}", e);
                e.into()
            })
    }

    #[tracing::instrument(name = "db::get_required_photos", skip(self))]
    async fn get_case_required_photos(&self, case_id: i32) -> Result<RequiredPhotos, Error> {
        let transaction_id = self.get_transaction_id(case_id).await?;

        let photos = sqlx::query_as::<_, Photo>(
            r#"
        SELECT p.id, name, group_name, input_type, multiple, c.comment
        FROM cases_photos_req c
        LEFT JOIN photo_types p ON c.photo_type_id = p.id
        WHERE case_id = ? AND enabled = 1
        ORDER BY photo_order;
        "#,
        )
        .bind(case_id)
        .fetch_all(&self.0)
        .await
        .map_err(|e| {
            tracing::error!("Failed to get url: {:?}", e);
            Error::Sqlx { source: e}
        })?;

        Ok(RequiredPhotos {
            transaction_id,
            photos,
        })
    }

    #[tracing::instrument(name = "db::get_latest_comment", skip(self))]
    async fn get_latest_comment(&self, case_id: i32) -> Result<Comment, Error> {
        sqlx::query("SELECT comment, comment_time FROM cases_comments WHERE case_id = ? ORDER BY comment_time DESC")
            .bind(case_id)
            .fetch_one(&self.0)
            .await
            .and_then(|r| {
                let timestamp: NaiveDateTime = r.get("comment_time");

                Ok(Comment {
                    comment: r.get("comment"),
                    timestamp: timestamp.to_string(),
                })
            })
            .map_err(|e| {
                tracing::error!("Failed to get url: {:?}", e);
                e.into()
            })
    }

    #[tracing::instrument(name = "db::get_transaction_id", skip(self))]
    async fn get_transaction_id(&self, case_id: i32) -> Result<String, Error> {
        sqlx::query("SELECT transaction_id FROM cases_photos_req WHERE case_id = ? AND enabled = 1")
            .bind(case_id)
            .fetch_one(&self.0)
            .await
            .and_then(|r| Ok(r.get("transaction_id")))
            .map_err(|e| {
                tracing::error!("Failed to get transaction_id: {:?}", e);
                e.into()
            })
    }
}

const GET_CASE_DETAILS: &str = r#"
SELECT c.id,
       customer_name,
       customer_phone,
       customer_email,
       c.vehicle_number,
       vehicle_fuel_type,
       quote_number,
       inspection_by_wimwisure,
       CASE
           WHEN current_stage = 3 THEN 'SCHEDULED'
           WHEN current_stage = -1 THEN 'CLOSED'
           WHEN current_stage = 4 THEN 'QC'
           WHEN current_stage = 5 THEN 'COMPLETED' END        as status,
       creation_time,
       inspection_time,
       c.inspection_number,
       c.chassis_number,
       c.engine_number,
       CASE
           WHEN remark = 'recommended' THEN 'APPROVED'
           WHEN remark = 'not-recommended' THEN 'REJECTED'
           WHEN remark = 'hold' THEN 'HOLD'
           WHEN remark = 'underwriter' THEN 'UNDERWRITER' END as remark,
       comment,
       c.inspection_link,
       c.download_key,
       c.callback_url,
       c.inspection_by,
       c.app_version,
       c.qc_time,
       c.communication
FROM cases c
         left join online_cases o on c.id = o.case_id
WHERE c.id = ? AND c.insurance_company_id = ? AND c.app_version = 'v2'"#;
