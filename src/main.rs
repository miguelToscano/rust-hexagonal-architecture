pub mod domain;
pub mod adapters;

#[tokio::main]
pub async fn main() -> std::io::Result<()> {
    adapters::inbound::rest_server::run().await
}
