pub mod domain;
pub mod adapters;
pub mod application;

#[tokio::main]
pub async fn main() -> std::io::Result<()> {
    adapters::inbound::rest_server::run().await
}
