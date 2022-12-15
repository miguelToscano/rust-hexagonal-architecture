extern crate argon2;
pub mod adapters;
pub mod domain;
pub mod ports;

use dotenv::dotenv;

#[tokio::main]
pub async fn main() -> Result<(), std::io::Error> {
    dotenv().ok();

    let run_mode = std::env::var("RUN_MODE").unwrap_or_else(|_| "REST_SERVER".to_string());

    match run_mode.as_str() {
        "REST_SERVER" => adapters::inbound::rest_server::run().await,
        "GRAPHQL_SERVER" => panic!("GraphQL Server not implemented"),
        "CLI" => panic!("CLI mode not implemented"),
        _ => panic!("Invalid RUN_MODE env var"),
    }
}
