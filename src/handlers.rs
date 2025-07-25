use super::models::{NotConfigured, OBSConfig, OBSConnectionTimeout, Post};
use obws::client::ConnectConfig;
use obws::requests::EventSubscription;
use obws::responses::general::Version;
use obws::Client;
use std::sync::OnceLock;

static OBS_CONFIG: OnceLock<OBSConfig> = OnceLock::new();
pub fn set_obs_config(obs_config: OBSConfig) {
    OBS_CONFIG.get_or_init(|| obs_config);
}

// A function to handle GET requests at /posts/{id}
pub async fn get_post(id: u64) -> Result<impl warp::Reply, warp::Rejection> {
    // For simplicity, let's say we are returning a static post
    let post = Post {
        id,
        title: String::from("Hello,  Warp!"),
        body: String::from("This is a post about Warp."),
    };
    Ok(warp::reply::json(&post))
}

// A function to handle GET requests at /version
pub async fn get_version() -> Result<impl warp::Reply, warp::Rejection> {
    let config = OBS_CONFIG.get().ok_or_else(|| warp::reject::custom(NotConfigured))?;

    let obs_config = ConnectConfig {
        host: config.host.clone(),
        port: config.port.clone(),
        dangerous: None,
        password: Some(config.password.clone()),
        event_subscriptions: Some(EventSubscription::ALL),
        broadcast_capacity: config.broadcast_capacity.clone(),
        connect_timeout: config.connect_timeout.clone(),
    };

    let mut obs_client = Client::connect_with_config(obs_config).await.or_else(|_| Err(warp::reject::custom(OBSConnectionTimeout)))?;

    // Get version information of OBS and obs-websocket.
    let version: Version = obs_client.general().version().await.or_else(|_| Err(warp::reject::custom(OBSConnectionTimeout)))?;

    obs_client.disconnect().await;

    Ok(warp::reply::json(&version))
}