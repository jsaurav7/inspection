use crate::error::Error;
use crate::wssr::base62;
use crate::DB;

#[tracing::instrument(name = "service::save_url", skip(db))]
pub async fn save_url(db: &DB, url: &str) -> Result<String, Error> {
    let insert_id = db.save_url(url).await?;

    Ok(base62::encode(insert_id as i32))
}

#[tracing::instrument(name = "service::get_url", skip(db))]
pub async fn get_url(db: &DB, id: &str) -> Result<String, Error> {
    db.get_url(base62::decode(id) as i32).await
}
