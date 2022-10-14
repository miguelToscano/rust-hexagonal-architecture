use newsletter::server;
use newsletter::server::configuration;

#[tokio::main]
pub async fn main() -> std::io::Result<()> {
    let configuration = configuration::get_configuration().expect("Failed to read configuration.");

    let address = format!("{}:{}", configuration.host, configuration.port);

    let listener = std::net::TcpListener::bind(address).expect(&format!("Failed to bind {} port", configuration.port));
    server::app::run(listener)?.await
}
