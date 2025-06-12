use axum::{
    Router,
    http::StatusCode,
    response::Html,
    routing::get,
};

use http_body_util::BodyExt;
use hyper::{Request, client};
use mjolnir::application::config;
use serde_json::{Value, json};
use tokio::net::TcpListener;

struct TestConfig {
    host: String,
    port: u16,
}
impl TestConfig {
    fn new() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 0,
        }
    }

    fn base_url(&self, port: u16) -> String {
        format!("http://{}:{}", self.host, port)
    }
}

async fn create_test_app() -> Router {
    Router::new().route("/", get(root_handler))
}

async fn root_handler() -> Html<&'static str> {
    Html("<h1>Mjolnir<h1>")
}

async fn spawn_test_server() -> (String, tokio::task::JoinHandle<()>) {
    let config = TestConfig::new();
    let listner = TcpListener::bind(format!("{}:{}", config.host, config.port))
        .await
        .expect("Failed to bind to address");

    let addr = listner.local_addr().expect("Failed to get local address");
    let base_url = config.base_url(addr.port());

    let app = create_test_app().await;

    let handle = tokio::spawn(async move {
        axum::serve(listner, app)
            .await
            .expect("Failed to start test server");
    });

    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    (base_url, handle)
}

async fn make_request(url: &str) -> Result<(StatusCode, String), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client.get(url).send().await?;
    let status = response.status();
    let body = response.text().await?;
    Ok((status.into(), body))
}

#[tokio::test]
async fn test_root_endpoint() {
  let (base_url, _handle) = spawn_test_server().await;

  let (status, body) = make_request(&base_url).await.unwrap();

  assert_eq!(status, StatusCode::OK);
  assert_eq!(body, "<h1>Mjolnir<h1>");
}