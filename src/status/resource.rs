use rocket::serde::json::Json;

use crate::error::Error;
use crate::DB;
use crate::status::part_status::{PartStatus, QuoteStatus, VehicleStatus};
use crate::status::service;

#[tracing::instrument(skip(db))]
#[get("/<case_id>")]
pub async fn get_part_status(db: &DB, case_id: i32) -> Result<Json<Vec<PartStatus>>, Error> {
    Ok(Json(service::get_part_status(db, case_id).await?))
}

#[tracing::instrument(skip(db))]
#[get("/<vehicle_number>")]
pub async fn get_vehicle_status(db: &DB, vehicle_number: &str) -> Result<Json<VehicleStatus>, Error> {
    Ok(Json(service::get_vehicle_status(db, vehicle_number).await?))
}

#[tracing::instrument(skip(db))]
#[get("/<quote_number>")]
pub async fn get_quote_status(db: &DB, quote_number: &str) -> Result<Json<QuoteStatus>, Error> {
    Ok(Json(service::get_quote_status(db, quote_number).await?))
}
