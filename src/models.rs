use serde::Deserialize;

pub mod errors;

#[derive(Debug, Deserialize)]
pub struct AirVpnIpResponse {
    pub ip: String,
    pub ipv4: bool,
    pub ipv6: bool,
    pub airvpn: bool,
    pub geo: AirVpnGeo,
    pub result: AirVpnRespRes,
}

#[derive(Debug, Deserialize)]
pub struct AirVpnGeo {
    pub code: String,
    pub name: String,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AirVpnRespRes {
    Ok,
    Error,
}
