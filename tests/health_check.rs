use hyper::{header, StatusCode};
use std::net::TcpListener;
use test_case::test_case;

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");

    let addr = listener.local_addr().unwrap();

    let server = backend::startup::run(listener).expect("Failed to bind to address");

    tokio::spawn(server);

    addr.to_string()
}

#[tokio::test]
async fn health_check_works() {
    let addr = spawn_app();

    let client = reqwest::Client::new();

    let resp = client
        .get(format!("http://{addr}/health_check"))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(resp.status().is_success());
    assert_eq!(Some(0), resp.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_data() {
    // Arrange
    let addr = spawn_app();

    let client = reqwest::Client::new();

    // Act
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(&format!("http://{addr}/subscriptions"))
        .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(response.status(), StatusCode::OK);
}

#[test_case("name=le%20guin"; "when it is missing the email")]
#[test_case("email=ursula_le_guin%gmail.com"; "when it is missing the name")]
#[test_case(""; "when it is missing both the email and the name")]
#[tokio::test]
async fn subscribe_returns_a_422_for_invalid_data(invalid_body: &'static str) {
    // Arrange
    let addr = spawn_app();
    let client = reqwest::Client::new();
    // Act
    let response = client
        .post(&format!("http://{addr}/subscriptions"))
        .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(invalid_body)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY)
}
