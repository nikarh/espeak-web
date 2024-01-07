use std::{env, net::Ipv4Addr, sync::Arc};

use axum::{extract::State, routing::post, Router};
use tokio::{process, sync::Mutex};
use tracing::{info, warn};

#[derive(Default, Clone)]
struct App {
    lock: Arc<Mutex<()>>,
}

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let port = env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8585);

    info!("Starting server on port {port}");

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", post(espeak))
        .with_state(App::default());

    let listener = tokio::net::TcpListener::bind((Ipv4Addr::UNSPECIFIED, port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn espeak(app: State<App>, body: String) -> () {
    info!("Speaking `{body}`");

    // Espeak can not parallelize
    let _guard = app.lock.lock().await;

    if let Err(err) = process::Command::new("espeak").arg(body).output().await {
        warn!("Espeak failed: {err}");
    }
}
