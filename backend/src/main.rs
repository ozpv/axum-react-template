use axum::{
    extract::{Request, State},
    handler::HandlerWithoutStateExt,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use std::path::PathBuf;
use std::time::Duration;
use tokio::net::TcpListener;
use tower::util::ServiceExt;
use tower_http::{
    compression::CompressionLayer,
    services::{ServeDir, ServeFile},
    timeout::TimeoutLayer,
    trace::TraceLayer,
};

use backend::state::AppState;

struct Env {
    site_addr: String,
    dist_dir: PathBuf,
}

impl Env {
    fn get_or_default() -> Self {
        let site_addr = std::env::var("SITE_ADDR").unwrap_or_else(|_| "127.0.0.1:3000".to_string());
        let dist_dir = std::env::var("DIST_DIR")
            .unwrap_or_else(|_| format!("{}/../frontend/dist", env!("CARGO_MANIFEST_DIR")))
            .into();

        Self {
            site_addr,
            dist_dir,
        }
    }
}

async fn not_found() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, Html("<h1>404 Not Found</h1>"))
}

// serves static files with GET /assets/*
async fn static_file_server(State(state): State<AppState>, req: Request) -> impl IntoResponse {
    ServeDir::new(state.get_dist_dir().as_ref())
        .precompressed_br()
        .precompressed_gzip()
        .fallback(not_found.into_service())
        .oneshot(req)
        .await
        .map_err(|_| (StatusCode::NOT_FOUND, Html("<h1>404 Not Found</h1>")))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let Env {
        site_addr,
        dist_dir,
    } = Env::get_or_default();

    let index_path = dist_dir.join("index.html");

    let listener = TcpListener::bind(&site_addr).await?;

    tracing::info!("Listening on http://{site_addr}/");
    tracing::info!("Serving files in {}", dist_dir.display());

    let router = Router::new()
        // test api
        .route("/healthcheck", get(|| async { "routing works" }))
        // serve the frontend statically
        // serving the index file allows for react router to resume the routing
        .fallback_service(ServeFile::new(index_path))
        .route("/assets/{*any}", get(static_file_server))
        .layer(CompressionLayer::new().gzip(true))
        .layer(TimeoutLayer::new(Duration::from_secs(30)))
        .layer(TraceLayer::new_for_http())
        .with_state(AppState::new(dist_dir));

    axum::serve(listener, router).await?;
    Ok(())
}
