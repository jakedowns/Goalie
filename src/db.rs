use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv::dotenv;
use std::env;

pub type DbConn = Pool<ConnectionManager<SqliteConnection>>;

pub fn establish_connection() -> DbConn {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    Pool::new(manager).expect("Failed to create the connection pool")
}
