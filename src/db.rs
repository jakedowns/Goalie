use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use r2d2_sqlite::SqliteConnectionManager;
use diesel::result::Error;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::SqliteConnection;

pub type DbConn = Pool<SqliteConnectionManager>;

pub type SqlitePool = Pool<ConnectionManager<SqliteConnection>>;

pub fn establish_connection() -> PooledConnection<ConnectionManager<SqliteConnection>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    pool.get().expect("Failed to get connection from pool.")
}

pub fn establish_connection_and_execute<T, R>(operation: T) -> Result<R, Error>
where
    T: FnOnce(&mut SqliteConnection) -> Result<R, Error>,
{
    let connection = establish_connection();
    let mut conn = connection; // Get a connection from the pool
    operation(&mut conn) // Pass the connection to the operation function
}
