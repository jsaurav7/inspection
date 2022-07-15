use lambda_web::LambdaError;

use inspection::run;

#[rocket::main]
async fn main() -> Result<(), LambdaError> {
    tracing_subscriber::fmt().without_time().init();

    run().await
}
