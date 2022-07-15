#[macro_use]
extern crate rocket;

use lambda_web::{is_running_on_lambda, launch_rocket_on_lambda, LambdaError};
use rocket_db_pools::Database;

use crate::db::DB;

mod db;
pub mod error;
mod wssr;

pub async fn run() -> Result<(), LambdaError> {
    let rocket = rocket::build()
        .attach(DB::init())
        .mount(
            "/wssr/api/v1/create-short",
            routes![wssr::resource::save_url],
        )
        .mount("/wssr", routes![wssr::resource::get_url]);

    if is_running_on_lambda() {
        launch_rocket_on_lambda(rocket).await?;
    } else {
        let _ = rocket.launch().await?;
    }

    Ok(())
}
