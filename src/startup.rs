use axum::{
    routing::IntoMakeService,
    routing::{get, post},
    Router, Server,
};
use hyper::{server::conn::AddrIncoming, Result};
use std::net::TcpListener;

pub type App = Server<AddrIncoming, IntoMakeService<Router>>;

pub fn run(listener: TcpListener) -> Result<App> {
    let app = Router::new()
        .route("/health_check", get(crate::routes::health_check))
        .route("/subscriptions", post(crate::routes::subscribe));

    Ok(Server::from_tcp(listener)?.serve(app.into_make_service()))
}
