use backend::configuration::get_configuration;
use backend::startup::run;
use std::net::{SocketAddr, TcpListener};

#[tokio::main]
async fn main() -> hyper::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration");
    let addr = SocketAddr::from(([127, 0, 0, 1], configuration.application_port));
    let listener = TcpListener::bind(&addr).unwrap();
    run(listener)?.await
}
