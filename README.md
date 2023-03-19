| Goalie Scorekeeper Service |
| --- |
| Goalie is a scorekeeper service designed to track scores for various games. |

### Features

- [ ] RESTful API for game management and score tracking
- [ ] GraphQL endpoint for querying and retrieving scores
- [ ] Integration with various game engines
- [ ] User authentication and authorization
- [ ] Pagination and search functionality for scoreboards
- [ ] Admin dashboard for game management and data analytics

### TODOS

1.  Generate code to create SQL from DBML
2.  Generate server code
3.  Write tests for client -> server communications
4.  Deploy
5.  Add test games to deployed database
6.  Set up database backups
7.  Write documentation a. Use inline comments for generated documentation b. Use a documentation generator (e.g., Swagger, Slate)
8.  Add privacy and terms page
9.  Add a free trial tier with user permissions and roles a. Control the number of games users can add based on their subscription
10.  Assign unique IDs and secret keys to each game a. Allow refreshing and revoking secret keys
11.  Add a billing and subscription management area a. Allow users to pause or cancel their subscription b. Consider using a subscription billing service/API (e.g., Braintree, Stripe)
12.  Create a pricing info page a. Offer 1 game for free b. Charge $10/month for each additional game

### Details

To create a performant GraphQL server in Rust, you can follow these steps:

1.  Set up the Rust development environment: Install Rust and its package manager, Cargo, by following the official installation guide: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

2.  Create a new Rust project: Run `cargo new graphql_server` to create a new Rust project. Change into the project directory by running `cd graphql_server`.

3.  Add required dependencies: Open the `Cargo.toml` file in the project directory and add the necessary dependencies for building a GraphQL server. Some popular crates to use include `actix-web`, `async-graphql`, and `sqlx`. Example:

```toml
[dependencies]

actix-web = "4.0.0-beta.8"
async-graphql = "3.0.14"
async-graphql-actix-web = "3.0.14"
sqlx = { version = "0.5", features = ["postgres", "runtime-tokio-rustls"] }
tokio = { version = "1", features = ["full"] }
```

4.  Implement the data model: Create Rust structs and database functions to represent the data model and interact with the database. You can use the `sqlx` crate to establish a connection and perform database queries.

5.  Define the GraphQL schema: Use the `async-graphql` crate to define the GraphQL schema, types, resolvers, and mutations that correspond to your API definition.

6.  Implement the server: Use the `actix-web` crate to create a web server that handles incoming GraphQL requests. Register the `async-graphql-actix-web` integration middleware to route the requests to the GraphQL schema.

7.  Optimize performance: Rust is already a fast language, but to further optimize your server for millions of requests per second, consider using caching strategies, connection pooling, and benchmarking your server to find bottlenecks and areas for improvement.

8.  Test the server: Write test cases for the GraphQL queries, mutations, and other functionalities to ensure that everything works as expected.

9.  Build the server: Run `cargo build --release` to build the server in release mode, which applies optimizations for better performance.

10.  Deploy the server: Deploy the compiled server binary to a hosting provider or cloud platform, making sure to configure it for secure communications, proper performance, and database connectivity.


Following these steps will help you create a performant GraphQL server in Rust, which can handle a high load of requests per second.


# Building / Running

`cargo install diesel_cli --no-default-features --features sqlite`

# AI Pull Request Requests:

1.  `touch sqlite.db` #TODO put into generate.sh and remove from readme TODO list

