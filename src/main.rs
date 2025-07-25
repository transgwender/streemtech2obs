use crate::models::Config;
use std::env;
use std::net::IpAddr;
use std::{fs, process};

mod models;
mod handlers;
mod routes;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    let config: Config = parse_config(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing configuration: {err}");
        process::exit(1);
    });

    server(config).await;
}

async fn server(config: Config) {
    let routes = routes::routes();

    let addr = config.ip.parse::<IpAddr>().unwrap_or_else(|err| {
        eprintln!("Problem parsing IP address: {err}");
        process::exit(1);
    });
    println!("Server starting at http://{}:{}", addr, config.port);
    warp::serve(routes).run((addr, config.port)).await;
}


fn parse_config(args: &[String]) -> Result<Config, &str> {
    if args.len() < 2 {
        return Err("not enough arguments");
    }

    let file_path = &args[1];

    let contents = fs::read_to_string(file_path).map_err(|_| "cannot read file")?;
    let config: Config = serde_json::from_str(&contents).map_err(|_| "cannot deserialize")?;

    Ok(config)
}