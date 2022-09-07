use axum::{
    http::{HeaderValue, Method},
    response::Json,
    routing::get,
    Router,
};
use serde_json::{json, Value};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/api", get(json)).layer(
        CorsLayer::new()
            .allow_origin("http://127.0.0.1:5173".parse::<HeaderValue>().unwrap())
            .allow_methods([Method::GET]),
    );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // run it with hyper on localhost:3000
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn json() -> Json<Value> {
    Json(json!({ "data": 42 }))
}
