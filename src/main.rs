
use tokio::sync::oneshot;
use mjolnir::application::app;


#[tokio::main]
async fn main() {
  let _ = tracing_subscriber::fmt().try_init();

  let(tx, rx) = oneshot::channel();

  tokio::spawn(async move {
    app::start_server(tx).await;
  });

  let _ = rx.await;

  tokio::signal::ctrl_c().await.expect("Failed to listen for ctrc+c");
  println!("Shutting down...");
}