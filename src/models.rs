use serde::{Deserialize, Serialize};

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
}

fn _default_ip() -> String { String::from("127.0.0.1") }
fn _default_port() -> u16 { 8000 }

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Post {
    pub id: u64,
    pub title: String,
    pub body: String,
}