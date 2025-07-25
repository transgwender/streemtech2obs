use std::net::IpAddr;
use std::time::Duration;
use obws::client::{DEFAULT_BROADCAST_CAPACITY, DEFAULT_CONNECT_TIMEOUT};
use obws::requests::EventSubscription;
use serde::{Deserialize, Serialize};
use warp::http::StatusCode;

#[derive(Deserialize, Debug)]
pub struct Config {

    #[serde(default = "_default_ip")]
    pub ip: String,

    #[serde(default = "_default_port")]
    pub port: u16,

    pub token: String,
    pub obs_ip: String,
    pub obs_port: u16,
    pub obs_password: String,

    #[serde(default = "_default_broadcast_capability")]
    pub broadcast_capacity: usize,
    #[serde(default = "_default_connect_timeout")]
    pub connect_timeout: Duration,
}

fn _default_ip() -> String { String::from("127.0.0.1") }
fn _default_port() -> u16 { 8000 }
fn _default_broadcast_capability() -> usize { DEFAULT_BROADCAST_CAPACITY }
fn _default_connect_timeout() -> Duration { DEFAULT_CONNECT_TIMEOUT }

pub struct ServerConfig {
    pub addr: IpAddr,
    pub port: u16,
}

pub struct OBSConfig {
    pub host: String,
    pub port: u16,
    pub password: String,
    pub broadcast_capacity: usize,
    pub connect_timeout: Duration,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Post {
    pub id: u64,
    pub title: String,
    pub body: String,
}

#[derive(Debug)]
pub struct NotConfigured;

#[derive(Debug)]
pub struct OBSConnectionTimeout;

impl warp::reject::Reject for NotConfigured {}

impl warp::reject::Reject for OBSConnectionTimeout {}

/// An API error serializable to JSON.
#[derive(Serialize)]
struct ErrorMessage {
    code: u16,
    message: String,
}

// This function receives a `Rejection` and tries to return a custom
// value, otherwise simply passes the rejection along.
pub async fn handle_rejection(err: warp::reject::Rejection) -> Result<impl warp::Reply, std::convert::Infallible> {
    let code;
    let message;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "NOT_FOUND";
    } else if let Some(NotConfigured) = err.find() {
        code = StatusCode::SERVICE_UNAVAILABLE;
        message = "NOT_CONFIGURED";
    } else if let Some(OBSConnectionTimeout) = err.find() {
        code = StatusCode::GATEWAY_TIMEOUT;
        message = "OBS_CONNECTION_TIMEOUT";
    } else {
        // We should have expected this... Just log and say its a 500
        eprintln!("unhandled rejection: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "UNHANDLED_REJECTION";
    }

    let json = warp::reply::json(&ErrorMessage {
        code: code.as_u16(),
        message: message.into(),
    });

    Ok(warp::reply::with_status(json, code))
}