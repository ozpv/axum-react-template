use axum::{
    handler::HandlerWithoutStateExt,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use std::path::PathBuf;
use std::time::Duration;
use tokio::net::TcpListener;
use tower_http::{
    compression::CompressionLayer, services::ServeDir, timeout::TimeoutLayer, trace::TraceLayer,
};

async fn not_found() -> impl IntoResponse {
    Html("<h1>404 Not Found</h1>")
}

async fn hello_from_axum() -> impl IntoResponse {
    (StatusCode::OK, "Working")
}

struct Env {
    site_addr: String,
    dist_dir: PathBuf,
}

async fn env_or_default() -> Env {
    let site_addr = std::env::var("SITE_ADDR").unwrap_or("127.0.0.1:3000".to_string());
    let dist_dir = std::env::var("DIST_DIR")
        .unwrap_or(format!("{}/../frontend/dist", env!("CARGO_MANIFEST_DIR")))
        .into();

    Env {
        site_addr,
        dist_dir,
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let env = env_or_default().await;

    let router = Router::new()
        // testing route
        .route("/test", get(hello_from_axum))
        // serve the frontend statically
        .fallback_service(ServeDir::new(&env.dist_dir).not_found_service(not_found.into_service()))
        .layer(CompressionLayer::new().gzip(true))
        .layer(TimeoutLayer::new(Duration::from_secs(30)))
        .layer(TraceLayer::new_for_http());

    let listener = TcpListener::bind(&env.site_addr).await?;

    tracing::info!("Listening on {}", &env.site_addr);
    tracing::info!("Serving files in {}", env.dist_dir.display());
    axum::serve(listener, router).await?;
    Ok(())
}
