use crate::error::Error;
use crate::status::part_status::{PartStatus, QuoteStatus, VehicleStatus};
use crate::DB;

#[tracing::instrument(name = "service::get_part_status", skip(db))]
pub async fn get_part_status(db: &DB, case_id: i32) -> Result<Vec<PartStatus>, Error> {
    db.get_part_status(case_id).await
}

#[tracing::instrument(name = "service::get_vehicle_status", skip(db))]
pub async fn get_vehicle_status(db: &DB, vehicle_number: &str) -> Result<VehicleStatus, Error> {
    db.get_vehicle_status(vehicle_number).await
}

#[tracing::instrument(name = "service::get_quote_status", skip(db))]
pub async fn get_quote_status(db: &DB, quote_number: &str) -> Result<QuoteStatus, Error> {
    db.get_quote_status(quote_number).await
}
