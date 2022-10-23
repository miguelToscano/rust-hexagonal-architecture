use sqlx::PgPool;

pub mod domain;
pub mod server;
pub mod repositories;

use newsletter::server;
use newsletter::server::configuration;
#[tokio::main]
pub async fn main() -> std::io::Result<()> {
    server::app::run().await?
}
