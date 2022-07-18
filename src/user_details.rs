use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;
use sqlx::Error;

use crate::DB;

#[derive(sqlx::FromRow)]
pub struct User {
    pub company_name: String,
    pub branch_id: i32,
}

#[derive(Debug)]
pub enum UserError {
    NotFound,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = UserError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let api_key = match request.headers().get_one("X-WIM-TOKEN") {
            None => return Outcome::Failure((Status::Unauthorized, UserError::NotFound)),
            Some(key) => key,
        };

        let db = request.guard::<&DB>().await.unwrap();

        match db.get_branch_user(api_key).await {
            Ok(user) => Outcome::Success(user),
            Err(_) => return Outcome::Failure((Status::Unauthorized, UserError::NotFound)),
        }
    }
}

impl DB {
    #[tracing::instrument(name = "db::get_branch_user", skip(self))]
    pub async fn get_branch_user(&self, api_key: &str) -> Result<User, Error> {
        sqlx::query_as::<_, User>("SELECT id as branch_id, company as company_name FROM company_branch_division WHERE api_key = ?")
            .bind(api_key)
            .fetch_one(&self.0)
            .await
            .map_err(|e| {
                tracing::error!("Failed to get branch user: {:?}", e);
                e.into()
            })
    }

    #[tracing::instrument(name = "db::get_company_user", skip(self))]
    pub async fn get_company_user(&self, api_key: &str) -> Result<User, Error> {
        sqlx::query_as::<_, User>("SELECT id as company_name, default_branch as branch_id FROM insurance_companies WHERE api_key = ?")
            .bind(api_key)
            .fetch_one(&self.0)
            .await
            .map_err(|e| {
                tracing::error!("Failed to get company user: {:?}", e);
                e.into()
            })
    }
}
