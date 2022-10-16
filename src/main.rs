use backend::startup::run;
use std::net::{SocketAddr, TcpListener};

#[tokio::main]
async fn main() -> hyper::Result<()> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    let listener = TcpListener::bind(&addr).unwrap();
    run(listener)?.await
}
