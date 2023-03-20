use actix_web::{App, web, test};
use crate::db::DbConn;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::sqlite::SqliteConnection;

pub async fn setup_test_db() -> web::Data<DbConn> {
    // Set up test environment
    let manager = ConnectionManager::<SqliteConnection>::new("sqlite://test_db.sqlite3");
    let pool = Pool::builder().build(manager).expect("Failed to create test pool.");
    let conn: PooledConnection<ConnectionManager<SqliteConnection>> = pool.get().expect("Failed to get a connection from the pool.");
    let db = web::Data::new(conn);
    return db;
}



#[actix_rt::test]
pub async fn setup_app(db: web::Data<DbConn>) -> test::TestApp<actix_web::App<web::Data<DbConn>>> {
    // Create an instance of the app with the submit_move_scope
    let app = test::init_service(App::new().data(db.clone()).service(submit_move_scope())).await;
    // Return app
    return app;
}

#[actix_rt::test]
pub async fn setup_for_test() -> (actix_web::test<TestApp>, web::Data<DbConn>) {
    let db = setup_test_db().await;
    let app = setup_app(db).await;

    // Return app and db
    (app, db)
}

#[actix_web::test]
async fn test_index_ok() {
    let (app, db) = setup_for_test().await;
    let req = test::TestRequest::default()
        .insert_header(http::header::ContentType::plaintext())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), http::StatusCode::OK);
}

#[actix_web::test]
async fn test_index_not_ok() {
    let (app, db) = setup_for_test().await;
    let req = test::TestRequest::default().to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
}
