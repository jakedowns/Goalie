use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use graphql_client::{GraphQLQuery, Response};
use std::collections::HashMap;

type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

fn create_pool() -> Pool {
    // Set up an in-memory SQLite database for testing.
    let manager = ConnectionManager::<SqliteConnection>::new(":memory:");
    let pool = r2d2::Pool::builder().build(manager).unwrap();

    // Run migrations.
    let conn = pool.get().unwrap();
    embedded_migrations::run(&conn).unwrap();

    pool
}

// Define our test query.
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.graphql",
    query_path = "tests/resolver_test.graphql",
    response_derives = "Debug"
)]
struct TestQuery;

#[tokio::test]
async fn test_resolver() {
    // Create a test pool and get a connection.
    let pool = create_pool();
    let conn = pool.get().unwrap();

    // Insert some test data into the database.
    // ...

    // Define our query variables.
    let variables = HashMap::new();

    // Execute the query and parse the response.
    let response_body = surf::post("http://localhost:8000/graphql")
        .body(json!({
            "query": TestQuery::build_query(variables),
            "variables": variables
        }))
        .await
        .unwrap()
        .body_string()
        .await
        .unwrap();

    let response: Response<test_query::ResponseData> =
        serde_json::from_str(&response_body).unwrap();

    // Assert that the response is what we expect.
    // ...
}
