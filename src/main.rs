use mjolnir::application::{app, state::AppState};
use mjolnir::infrastructure::{database, redis};
use tokio::sync::oneshot;


#[tokio::main]
async fn main() {
    let _ = tracing_subscriber::fmt().try_init();

    let (tx, rx) = oneshot::channel();

    tokio::spawn(async move {
        app::start_server(tx).await;
    });

    let _ = rx.await;

    tokio::signal::ctrl_c()
        .await
        .expect("Failed to listen for ctrl+c");
    println!("Shutting down...");
}
