use server::start_server;
use tracing_subscriber::EnvFilter;

mod config;
mod models;
mod routes;
mod server;
mod sysctl;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    start_server().await.unwrap();
}
