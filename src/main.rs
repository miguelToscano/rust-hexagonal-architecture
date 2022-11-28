pub mod adapters;
pub mod application;
pub mod domain;
use dotenv::dotenv;

#[tokio::main]
pub async fn main() -> Result<(), std::io::Error> {
    dotenv().ok();
    match std::env::var("MODE")  {
        Ok(mode) => {
            match mode.as_str() {
                "REST_SERVER" => adapters::inbound::rest_server::run().await,
                "CLI" => panic!("CLI mode not implemented yet"),
                _ => panic!("could not read MODE env var"),
            }
        },
        _ => Ok(()),
    }
}
