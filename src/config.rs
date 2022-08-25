use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub redis: Redis,
    pub port: u32,
}

#[derive(Deserialize)]
pub struct Redis {
    pub host: String,
    pub password: String,
    pub port: u32,
    pub username: String,
}
