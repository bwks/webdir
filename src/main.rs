use std::{net::SocketAddr, path::PathBuf, str::FromStr};

use anyhow::{Context, Result};
use axum::{routing::get_service, Router};
use clap::Parser;
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Directory to serve
    #[arg(short, long)]
    dir: PathBuf,

    /// IP address to listen on
    #[arg(short = '4', long, default_value = "127.0.0.1")]
    ipv4: String,

    /// Port to listen on
    #[arg(short, long, default_value_t = 13337)]
    port: u16,

    /// Log level (error, warn, info, debug, trace)
    #[arg(short, long, default_value = "info")]
    log_level: String,
}

/// Setup logging.
fn setup_logging(log_level: &str) -> Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| log_level.to_string()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .try_init()
        .context("Failed to set up logging")
}

/// Build the application with directory to serve.
fn build_app(dir: PathBuf) -> Router {
    let serve_dir = ServeDir::new(dir);
    Router::new()
        .nest_service("/", get_service(serve_dir))
        .layer(TraceLayer::new_for_http())
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Initialize tracing
    setup_logging(&args.log_level)?;

    // Check if directory exists
    if !args.dir.exists() {
        anyhow::bail!("Directory '{:?}' does not exist!", args.dir);
    }
    let canonical_path = args
        .dir
        .canonicalize()
        .with_context(|| format!("Failed to canonicalize path: {:?}", args.dir))?;
    tracing::info!("Serving directory: {:?}", canonical_path);

    // List contents of the directory
    match std::fs::read_dir(&canonical_path) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => tracing::debug!("Found file: {:?}", entry.path()),
                    Err(e) => tracing::warn!("Error reading directory entry: {}", e),
                }
            }
        }
        Err(e) => tracing::warn!("Error reading directory: {}", e),
    }

    // Build our application with a route and logging
    let app = build_app(args.dir);

    // Parse the IP address
    let ipv4 = std::net::IpAddr::from_str(&args.ipv4)
        .with_context(|| format!("Invalid IP address: {}", args.ipv4))?;

    // Run it
    let addr = SocketAddr::new(ipv4, args.port);
    tracing::info!("Listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .context("Server error")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    use anyhow::Result;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tempfile::TempDir;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_build_app() {
        let temp_dir = tempfile::tempdir().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        fs::write(&file_path, "Hello, world!").unwrap();

        let app = build_app(temp_dir.path().to_path_buf());

        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .uri("/test.txt")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        assert_eq!(&body[..], b"Hello, world!");
    }

    #[test]
    fn test_setup_logging() {
        assert!(setup_logging("debug").is_ok());
    }

    #[tokio::test]
    async fn test_serve_file() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let file_path = temp_dir.path().join("test.txt");
        fs::write(&file_path, "Hello, world!")?;

        let app = build_app(temp_dir.path().to_path_buf());

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/test.txt")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await?;

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await?;
        assert_eq!(&body[..], b"Hello, world!");

        Ok(())
    }

    #[tokio::test]
    async fn test_not_found() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let app = build_app(temp_dir.path().to_path_buf());

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/nonexistent.txt")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await?;

        assert_eq!(response.status(), StatusCode::NOT_FOUND);

        Ok(())
    }
}
