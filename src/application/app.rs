use crate::api::router;
use crate::application::{config, state::AppState};
use crate::infrastructure::{database, redis};
use tokio::sync::oneshot;
use tower_http::cors::{Any, CorsLayer};



pub async fn start_server(api_ready: oneshot::Sender<()>) {
    config::Config::load();
    let config = config::Config::get();
    let socket_addr = config.service_socket_addr();

    let db_pool = database::create_pool(config)
        .await
        .expect("failed to create Database pool");

    let redis_pool = redis::connection::create_pool(&config.redis_url)
    .await
    .expect("Failed to create Redis pool");

    let app_state = AppState::new(db_pool, redis_pool);

    let app = router::router()
        .layer(CorsLayer::new().allow_origin(Any))
        .with_state(app_state);

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
    let ctrl_c = async {
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
