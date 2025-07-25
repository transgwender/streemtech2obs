use crate::models::{Config, OBSConfig, ServerConfig};
use std::env;
use std::net::IpAddr;
use std::{fs, process};
use std::thread::sleep;
use std::time::Duration;
use obws::Client;
use obws::client::{ConnectConfig, DEFAULT_BROADCAST_CAPACITY, DEFAULT_CONNECT_TIMEOUT};
use obws::requests::EventSubscription;

mod models;
mod handlers;
mod routes;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    let (server_config, obs_config) : (ServerConfig, OBSConfig) = parse_config(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing configuration: {err}");
        process::exit(1);
    });

    // let obs_config = ConnectConfig {
    //     host: config.obs_ip.clone(),
    //     port: config.obs_port,
    //     dangerous: None,
    //     password: Some(config.obs_password.clone()),
    //     event_subscriptions: Some(EventSubscription::ALL),
    //     broadcast_capacity: config.broadcast_capacity,
    //     connect_timeout: config.connect_timeout,
    // };
    //
    // let mut obs_client = Client::connect_with_config(obs_config).await.unwrap();

    // tokio::time::sleep(Duration::from_secs(5)).await;
    //
    // // Get and print out version information of OBS and obs-websocket.
    // let version = obs_client.general().version().await.unwrap();
    // println!("{version:#?}");
    //
    // // Get a list of available scenes and print them out.
    // let scene_list = obs_client.scenes().list().await.unwrap();
    // println!("{scene_list:#?}");
    //
    // obs_client.disconnect().await;

    server(server_config, obs_config).await;
}

async fn server(server_config: ServerConfig, obs_config: OBSConfig) {
    let routes = routes::routes(obs_config);

    println!("Server starting at http://{}:{}", server_config.addr, server_config.port);
    warp::serve(routes).run((server_config.addr, server_config.port)).await;
}


fn parse_config(args: &[String]) -> Result<(ServerConfig, OBSConfig), &str> {
    if args.len() < 2 {
        return Err("not enough arguments");
    }

    let file_path = &args[1];

    let contents = fs::read_to_string(file_path).map_err(|_| "cannot read file")?;
    let config: Config = serde_json::from_str(&contents).map_err(|_| "cannot deserialize")?;

    let addr = config.ip.parse::<IpAddr>().map_err(|_| "cannot parse ip")?;

    let server_config = ServerConfig {
        addr,
        port: config.port,
    };

    let obs_config = OBSConfig {
        host: config.obs_ip,
        port: config.obs_port,
        password: config.obs_password,
        broadcast_capacity: config.broadcast_capacity,
        connect_timeout: config.connect_timeout,
    };

    Ok((server_config, obs_config))
}