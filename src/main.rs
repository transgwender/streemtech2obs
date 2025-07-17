use std::{fs, process};
use std::env;
use std::net::IpAddr;
use crate::models::Config;

mod models;
mod handlers;
mod routes;

#[tokio::main]
async fn main() {
    let routes = routes::routes();

    let args: Vec<String> = env::args().collect();

    let file_path = parse_config(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let config: Config = serde_json::from_str(&contents).unwrap();

    println!("Server starting at http://{}:{}", config.ip, config.port);
    let addr = config.ip.parse::<IpAddr>().unwrap();
    warp::serve(routes).run((addr, config.port)).await;
}

fn parse_config(args: &[String]) -> Result<&str, &'static str> {
    if args.len() < 2 {
        return Err("not enough arguments");
    }

    let file_path = &args[1];

    Ok(file_path)
}