#[macro_use]
extern crate rocket;

use lambda_web::{is_running_on_lambda, launch_rocket_on_lambda, LambdaError};
use rocket_db_pools::Database;

use clients::AppState;

use crate::db::DB;

mod callback;
mod case_details;
mod clients;
mod create_case;
mod db;
mod error;
mod events;
mod media;
mod metadata;
mod models;
mod status;
mod upload;
mod user_details;
mod wssr;

pub async fn run() -> Result<(), LambdaError> {
    dotenv::dotenv().ok();

    let state = AppState::new().await;

    let rocket = rocket::build()
        .attach(DB::init())
        .manage(state)
        .mount(
            "/wssr/api/v1/create-short",
            routes![wssr::resource::save_url],
        )
        .mount("/wssr", routes![wssr::resource::get_url])
        .mount("/v1/event", routes![events::resource::handle_event])
        .mount(
            "/v2/case/callback/trigger",
            routes![callback::resource::send_callback],
        )
        .mount("/v2/case", routes![case_details::resource::get_case_info])
        .mount("/v2/case", routes![create_case::resource::create_case])
        .mount(
            "/v2/case/metadata",
            routes![metadata::resource::get_case_metadata],
        )
        .mount(
            "/v2/case/download/report",
            routes![media::resource::get_report],
        )
        .mount(
            "/v2/case/download/photos",
            routes![media::resource::get_photos],
        )
        .mount(
            "/v2/case/vehicle/photos/details",
            routes![media::resource::get_photos_details],
        )
        .mount(
            "/v2/case/vehicle/parts/status",
            routes![status::resource::get_part_status],
        )
        .mount(
            "/v2/case/status",
            routes![status::resource::get_vehicle_status],
        )
        .mount(
            "/v2/document/upload-zip",
            routes![upload::resource::upload_zip],
        )
        .mount(
            "/v2/case/status-by-quote",
            routes![status::resource::get_quote_status],
        );

    if is_running_on_lambda() {
        launch_rocket_on_lambda(rocket).await?;
    } else {
        let _ = rocket.launch().await?;
    }

    Ok(())
}
