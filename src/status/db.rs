use std::time::SystemTime;
use rocket_db_pools::sqlx;
use rocket_db_pools::sqlx::Row;
use sqlx::types::chrono::NaiveDateTime;

use crate::db::DB;
use crate::error::Error;
use crate::status::part_status::{PartStatus, QuoteStatus, VehicleStatus};

impl DB {
    #[tracing::instrument(name = "db::get_status", skip(self))]
    pub async fn get_part_status(&self, case_id: i32) -> Result<Vec<PartStatus>, Error> {
        sqlx::query_as::<_, PartStatus>(
            r#"
        SELECT qq.value, qo.value
        FROM cases_qc_answers AS qa
         JOIN cases_qc_questions AS qq ON qa.question_id = qq.id
         JOIN cases_qc_question_options AS qo ON qa.answer_id = qo.id
        WHERE case_id = ?
        "#,
        )
        .bind(case_id)
        .fetch_all(&self.0)
        .await
        .map_err(|e| {
            tracing::error!("Error fetching part status {:?}", e);
            e.into()
        })
    }

    #[tracing::instrument(name = "db::get_vehicle_status", skip(self))]
    pub async fn get_vehicle_status(&self, vehicle_number: &str) -> Result<VehicleStatus, Error> {
        sqlx::query("SELECT id, current_stage, remark, inspection_submit_time FROM cases WHERE vehicle_number = ? ORDER BY id DESC")
            .bind(vehicle_number)
            .fetch_one(&self.0)
            .await
            .and_then(|r| {
                let remark: Option<String> = r.get("remark");
                let current_stage: i32 = r.get("current_stage");

                let status = match remark.unwrap_or_default().as_str() {
                    "recommended" => "recommended",
                    "not-recommended" => "not-recommended",
                    _ => match current_stage {
                        -1 => "closed",
                        3 => "scheduled",
                        4 => "in_quality_check",
                        _ => "inspection_not_done",
                    },
                };

                let inspection_time: Option<NaiveDateTime> = r.get("inspection_submit_time");

                Ok(VehicleStatus {
                    case_id: r.get("id"),
                    status: status.to_string(),
                    inspection_time: inspection_time.map(|t| t.to_string()),
                })
            })
            .map_err(|e| {
                tracing::error!("Error fetching vehicle status {:?}", e);
                e.into()
            })
    }

    #[tracing::instrument(name = "db::get_quote_status", skip(self))]
    pub async fn get_quote_status(&self, quote_number: &str) -> Result<QuoteStatus, Error> {
        sqlx::query(
            r#"
        SELECT current_stage, remark, qc_time, close_time
        FROM online_cases AS o
         LEFT JOIN cases AS c on c.id = o.case_id
        WHERE quote_number = ?
        ORDER BY updated_date DESC
        "#,
        )
        .bind(quote_number)
        .fetch_one(&self.0)
        .await
        .and_then(|r| {
            let current_stage: i32 = r.get("current_stage");
            let remark: Option<String> = r.get("remark");

            let status = match remark.unwrap_or_default().as_str() {
                "recommended" => "APPROVED",
                "not-recommended" => "REJECTED",
                _ => match current_stage {
                    -1 => "CLOSED",
                    _ => "INSPECTION_NOT_DONE",
                },
            };

            let updated_at: Option<NaiveDateTime> = match status {
                "APPROVED" | "REJECTED" => r.get("qc_time"),
                "CLOSED" => r.get("close_time"),
                _ => Some(NaiveDateTime::from_timestamp(SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as i64, 0)),
            };

            Ok(QuoteStatus {
                status: status.to_string(),
                updated_at: updated_at.map(|t| t.to_string()),
            })
        })
        .map_err(|e| {
            tracing::error!("Error fetching quote status {:?}", e);
            e.into()
        })
    }
}
