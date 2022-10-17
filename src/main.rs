use backend::{configuration::get_configuration, startup::run};
use sqlx::PgPool;
use std::net::{SocketAddr, TcpListener};

#[tokio::main]
async fn main() -> hyper::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration");

    let addr = SocketAddr::from(([127, 0, 0, 1], configuration.application_port));

    let pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    let listener = TcpListener::bind(&addr).unwrap();

    run(listener, pool)?.await
}
