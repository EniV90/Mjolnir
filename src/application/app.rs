
use tokio::sync::oneshot;
use tower_http::cors::{Any, CorsLayer};
use crate::application::config;
use crate::api::router;


pub async fn start_server(api_ready: oneshot::Sender<()>) {
  config::Config::load();
  let config = config::Config::get();
  let socket_addr = config.service_socket_addr();

  let app = router::router()
    .layer(CorsLayer::new().allow_origin(Any));

  let listener = tokio::net::TcpListener::bind(&socket_addr).await.unwrap();
  tracing::info!("Server running on Port: {}", socket_addr);

  let _ = api_ready.send(());

  axum::serve(listener, app)
    .with_graceful_shutdown(shutdown_signal())
    .await
    .unwrap();

    tracing::info!("Server shutdown successfully.")
  
}

async fn shutdown_signal() {
  let ctrl_c = async{
    tokio::signal::ctrl_c()
    .await
    .expect("Failed to install Ctrc+C handler");
  };

  #[cfg(unix)]
  let terminate = async {
    tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
    .expect("Failed to install signal handler")
    .recv()
    .await;
  };

  #[cfg(not(unix))]
  let terminate = std::future::pending::<()>();

  tokio::select! {
    _ = ctrl_c => {},
    _ = terminate => {},

  }
tracing::info!("Signal received, starting graceful shutdown");
  
}