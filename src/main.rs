pub mod adapters;
pub mod application;
pub mod domain;

#[tokio::main]
pub async fn main() -> std::io::Result<()> {
    adapters::inbound::rest_server::run().await
}
