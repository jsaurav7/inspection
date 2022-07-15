use rocket_db_pools::sqlx::MySqlPool;
pub use rocket_db_pools::Database;

#[derive(Database)]
#[database("production")]
pub struct DB(pub MySqlPool);
