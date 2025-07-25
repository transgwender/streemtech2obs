use warp::Filter;
use crate::models::{handle_rejection, OBSConfig};
use super::handlers;

// A function to build our routes
pub fn routes(obs_config: OBSConfig) -> impl Filter<Extract = impl warp::Reply> + Clone {
    handlers::set_obs_config(obs_config);
    get_post().or(get_version()).recover(handle_rejection)
}

// A route to handle GET requests for a specific post
fn get_post() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("post" / u64)
        .and(warp::get())
        .and_then(handlers::get_post)
}

// A route to handle GET requests for OBS version
fn get_version() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("version")
        .and(warp::get())
        .and_then(handlers::get_version)
}