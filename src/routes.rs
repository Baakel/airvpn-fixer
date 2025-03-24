use std::time::Duration;

use axum::{Json, response::IntoResponse};
use tracing::{debug, error, warn};

use crate::{
    models::{AirVpnIpResponse, AirVpnRespRes},
    sysctl::renew_bluetit,
};

// use crate::config::CONFIG;

pub async fn get_connected_servers() -> impl IntoResponse {
    let client = reqwest::Client::new();

    let res = match client
        .post("https://airvpn.org/api/whatismyip/")
        // .bearer_auth(&CONFIG.air_api_key)
        .timeout(Duration::from_secs(4))
        .send()
        .await
    {
        Ok(res) => res,
        Err(e) => {
            error!(error = ?e, "Error while getting request");
            if e.is_timeout() {
                renew_bluetit().await.unwrap();
            }
            return Json({});
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
    Json({})
}
