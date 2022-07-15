use rocket::http::Status;
use rocket::response::Responder;
use rocket::{response, Request};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    // #[error("HTTP Error {source:?}")]
    // Reqwest {
    //     #[from] source: reqwest::Error,
    // },
    #[error("SerdeJson Error {source:?}")]
    SerdeJson {
        #[from]
        source: serde_json::Error,
    },
    #[error("Sqlx Error {source:?}")]
    Sqlx {
        #[from]
        source: rocket_db_pools::sqlx::Error,
    },
    #[error("unknown data store error")]
    Unknown,
}

impl<'r, 'o: 'r> Responder<'r, 'o> for Error {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'o> {
        // log `self` to your favored error tracker, e.g.
        // sentry::capture_error(&self);

        // tracing::error!("{:?}", self);

        match self {
            // Error::Sqlx { source } => {
            //     let message = format!("{:?}", source);
            //     Response::build_from(message.respond_to(req)?)
            //         .status(Status::InternalServerError)
            //         .raw_header("Content-Type", "text/plain")
            //         .ok()
            // }
            // in our simplistic example, we're happy to respond with the default 500 responder in all cases
            _ => Status::InternalServerError.respond_to(req),
        }
    }
}
