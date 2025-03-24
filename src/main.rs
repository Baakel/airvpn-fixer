use models::{AirVpnIpResponse, AirVpnRespRes};
use server::start_server;
use sysctl::renew_bluetit;
use tracing::{debug, error, info, warn};
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
    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    info!("Starting airvpn bluetit fixer");
    'outter: loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

        let client = reqwest::Client::new();

        let res = match client
            .post("https://airvpn.org/api/whatismyip/")
            // .bearer_auth(&CONFIG.air_api_key)
            .timeout(std::time::Duration::from_secs(2))
            .send()
            .await
        {
            Ok(res) => res,
            Err(e) => {
                error!(error = ?e, "Error while getting request");
                if e.is_timeout() {
                    renew_bluetit().await.unwrap();
                }
                continue 'outter;
            }
        };

        let resp = res.json::<AirVpnIpResponse>().await.unwrap();
        if !resp.airvpn || resp.result != AirVpnRespRes::Ok {
            // if resp.airvpn || resp.result != AirVpnRespRes::Ok {
            warn!(
                current_ip = resp.ip,
                current_location = resp.geo.name,
                "AirVPN disconnected. Renewing connection..."
            );
            renew_bluetit().await.unwrap();
        };
        debug!(resp = ?resp, "We got a response!");
    }
    // start_server().await.unwrap();
}
