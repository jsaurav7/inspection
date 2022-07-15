use rocket::response::Redirect;
use rocket::serde::json::Json;

use crate::error::Error;
use crate::wssr::service;
use crate::DB;

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveUrlRequest {
    pub long_url: String,
}

#[tracing::instrument(skip(db, save_url_req))]
#[post("/", data = "<save_url_req>")]
pub async fn save_url(db: &DB, save_url_req: Json<SaveUrlRequest>) -> Result<String, Error> {
    service::save_url(db, &save_url_req.long_url).await
}

#[tracing::instrument(skip(db))]
#[get("/<id>")]
pub async fn get_url(db: &DB, id: &str) -> Result<Redirect, Error> {
    Ok(Redirect::to(service::get_url(db, id).await?))
}
