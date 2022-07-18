use sqlx::Row;

use crate::callback::callback_details::CallbackDetails;
use crate::DB;
use crate::error::Error;

impl DB {
    #[tracing::instrument(name = "db::get_callback_details", skip(self))]
    pub async fn get_callback_details(&self, case_id: i32) -> Result<CallbackDetails, Error> {
        let photos_on_hold = self.get_photos_on_hold(case_id).await.unwrap_or(vec![]);

        sqlx::query(r#"
            SELECT c.inspection_number,
                   quote_number,
                CASE
                    WHEN remark = 'recommended' THEN 'APPROVED'
                    WHEN remark = 'not-recommended' THEN 'REJECTED'
                    WHEN remark = 'hold' THEN 'HOLD'
                    WHEN remark = 'underwriter' THEN 'UNDERWRITER' END                                AS remark,
                CASE
                    WHEN current_stage = 3 THEN 'SCHEDULED'
                    WHEN current_stage = -1 THEN 'CLOSED'
                    WHEN current_stage = 4 THEN 'QC'
                    WHEN current_stage = 5 THEN 'COMPLETED' END                                       AS status,
                    callback_url,
                    comment
             FROM cases AS c JOIN online_cases ON c.id = online_cases.case_id
             WHERE c.id = ? and c.app_version = 'v2'"#)
            .bind(case_id)
            .fetch_one(&self.0)
            .await
            .and_then(|r| {
                let remarks: Option<String> = r.get("remark");

                let photos_on_hold = match remarks.as_ref() {
                    None => vec![],
                    Some(remarks) => match remarks.as_str() {
                        "hold" | "HOLD" => photos_on_hold,
                        _ => vec![],
                    }
                };

                Ok(CallbackDetails {
                    id: case_id,
                    inspection_number: r.get("inspection_number"),
                    quote_number: r.get("quote_number"),
                    remarks,
                    status: r.get("status"),
                    comment: r.get("comment"),
                    photos_on_hold,
                    url: r.get("callback_url"),
                })
            })
            .map_err(|e| {
                error!("Error getting callback details: {:?}", e);
                e.into()
            })
    }
}
