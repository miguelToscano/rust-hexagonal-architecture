use std::net::TcpListener;

#[tokio::main]
pub async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind 8080 port");
    newsletter::server::run(listener)?.await
}
