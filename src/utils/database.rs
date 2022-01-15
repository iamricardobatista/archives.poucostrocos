use rocket_sync_db_pools::database;
use rocket_sync_db_pools::postgres;

#[database("persistent_storage")]
pub struct Db(postgres::Client);
