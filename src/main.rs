use backend::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> hyper::Result<()> {
    run(TcpListener::bind("127.0.0.1:8000").unwrap())?.await
}
