use rocket_db_pools::sqlx;
use rocket_db_pools::sqlx::Row;

use crate::db::DB;
use crate::error::Error;

impl DB {
    #[tracing::instrument(name = "db::save_url", skip(self))]
    pub async fn save_url(&self, url: &str) -> Result<u64, Error> {
        sqlx::query("INSERT INTO wssr (long_url) VALUES (?)")
            .bind(url)
            .execute(&self.0)
            .await
            .and_then(|r| Ok(r.last_insert_id()))
            .map_err(|e| e.into())
    }

    #[tracing::instrument(name = "db::get_url", skip(self))]
    pub async fn get_url(&self, id: i32) -> Result<String, Error> {
        sqlx::query("SELECT long_url FROM wssr WHERE id = ?")
            .bind(id)
            .fetch_one(&self.0)
            .await
            .and_then(|r| Ok(r.get("long_url")))
            .map_err(|e| {
                tracing::error!("Failed to get url: {:?}", e);
                e.into()
            })
    }
}
