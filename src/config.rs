use std::fs;
use serde::Deserialize;

const CONFIG_PATH: &'static str = "./config/config.toml";

#[derive(Deserialize)]
pub struct Config {
    pub application_id: u64,
    pub administrator_id: u64,
    pub db_path: String,
    pub role_ids: RoleIds,
}

#[derive(Deserialize)]
pub struct RoleIds {
    pub twitch: u64,
}

pub fn read_config() -> Config {
    fs::read(CONFIG_PATH)
        .as_deref()
        .map(toml::from_slice)
        .expect("Error while reading config file")
        .expect("Error while parsing config file")
}