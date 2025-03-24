use axum::{Router, routing::get};
use tracing::info;

use crate::{models::errors::AirError, routes::get_connected_servers};

pub async fn start_server() -> Result<(), AirError> {
    info!("Starting server on 0.0.0.0:9090");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:9090").await?;
    let router = Router::new().route("/sessions", get(get_connected_servers));

    axum::serve(listener, router).await?;
    Ok(())
}
