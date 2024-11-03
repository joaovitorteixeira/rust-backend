use std::env;

use lazy_static::lazy_static;
use diesel::r2d2::ConnectionManager;
use diesel::SqliteConnection;
use log::info;

use crate::api_error::ApiError;


type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<SqliteConnection>>;

lazy_static! {
    static ref POOL: Pool = {
        let db_url = env::var("DATABASE_URL").expect("Database url not set");
        let manager = ConnectionManager::<SqliteConnection>::new(db_url);
        Pool::new(manager).expect("Failed to create db pool")
    };
}

pub fn init() {
    info!("Initializing DB");
    lazy_static::initialize(&POOL);
}

pub fn connection() -> Result<DbConnection, ApiError> {
    POOL.get()
        .map_err(|e| ApiError::new(500, format!("Failed getting db connection: {}", e)))
}