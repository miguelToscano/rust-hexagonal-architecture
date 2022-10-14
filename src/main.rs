use sqlx::PgPool;

use newsletter::server;
use newsletter::server::configuration;

#[tokio::main]
pub async fn main() -> std::io::Result<()> {
    let configuration = configuration::get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&configuration.database.get_connection_string()).await.expect(&format!("Failed to connect to {}.", configuration.database.name));
    let address = format!("{}:{}", configuration.host, configuration.port);

    let listener = std::net::TcpListener::bind(address).expect(&format!("Failed to bind {} port", configuration.port));
    server::app::run(listener, connection_pool)?.await
}
