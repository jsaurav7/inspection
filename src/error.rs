use aws_sdk_s3::error::GetObjectError;
use aws_sdk_s3::types::SdkError;
use rocket::http::Status;
use rocket::response::Responder;
use rocket::{response, Request};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("HTTP Error {source:?}")]
    Reqwest {
        #[from] source: reqwest::Error,
    },
    #[error("Lettre Error {source:?}")]
    Lettre {
        #[from] source: lettre::transport::smtp::Error,
    },
    #[error("SerdeJson Error {source:?}")]
    SerdeJson {
        #[from]
        source: serde_json::Error,
    },
    #[error("Sqlx Error {source:?}")]
    Sqlx {
        #[from]
        source: sqlx::Error,
    },
    #[error("S3 Error {source:?}")]
    S3 {
        #[from]
        source: SdkError<GetObjectError>,
    },
    #[error("unknown data store error")]
    Unknown,
}

impl<'r, 'o: 'r> Responder<'r, 'o> for Error {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'o> {
        Status::InternalServerError.respond_to(req)
    }
}
