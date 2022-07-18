use rocket_db_pools::sqlx;
use rocket_db_pools::sqlx::MySqlPool;
pub use rocket_db_pools::Database;
use sqlx::Row;

use crate::error::Error;
use crate::models::inspector::Inspector;
use crate::models::notification_users::NotificationUsers;
use crate::models::photo_on_hold::PhotoOnHold;

#[derive(Database)]
#[database("production")]
pub struct DB(pub MySqlPool);

impl DB {
    #[tracing::instrument(name = "db::get_case_inspectors", skip(self))]
    pub async fn get_case_inspectors(&self, case_id: i32) -> Result<Vec<Inspector>, Error> {
        sqlx::query_as::<_, Inspector>("SELECT phone_number, inspection_link FROM cases_user_allocation c LEFT JOIN users u ON c.user_id = u.username WHERE case_id=? AND stage=3")
            .bind(case_id)
            .fetch_all(&self.0)
            .await
            .map_err(|e| {
                error!("Error fetching case inspectors: {:?}", e);
                e.into()
            })
    }

    #[tracing::instrument(name = "db::get_photos_on_hold", skip(self))]
    pub async fn get_photos_on_hold(&self, case_id: i32) -> Result<Vec<PhotoOnHold>, Error> {
        sqlx::query_as::<_, PhotoOnHold>("SELECT photo_type_id, comment FROM cases_photos_req WHERE case_id = ? and enabled = 1")
            .bind(case_id)
            .fetch_all(&self.0)
            .await
            .map_err(|e| {
                error!("Error fetching photos on hold: {:?}", e);
                e.into()
            })
    }

    #[tracing::instrument(name = "db::get_notification_users", skip(self))]
    pub async fn get_notification_users(&self, case_id: i32) -> Result<NotificationUsers, Error> {
        sqlx::query(
            "SELECT notification_to, notification_type FROM cases_notifications WHERE case_id = ?",
        )
            .bind(case_id)
            .fetch_all(&self.0)
            .await
            .and_then(|rows| {
                Ok(rows
                    .into_iter()
                    .map(|row| (row.get("notification_to"), row.get("notification_type")))
                    .fold(
                        NotificationUsers::default(),
                        |mut acc, (notification_to, notification_type): (String, String)| {
                            if notification_type == "EMAIL" {
                                acc.emails.push(notification_to);
                            } else {
                                acc.phones.push(notification_to);
                            }

                            acc
                        },
                    ))
            })
            .map_err(|e| {
                tracing::error!("Failed to get notification users: {:?}", e);
                e.into()
            })
    }
}
