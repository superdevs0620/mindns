use std::path::PathBuf;

use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct ServerSettings {
    pub port: u16,
    pub bind: String,
}

#[derive(Deserialize)]
pub struct MirrorSettings {
    pub enabled: bool,
    pub primary: String,
    pub secondary: Option<String>,
}

#[derive(Deserialize)]
pub struct RulesSettings {
    pub load_as: String,
    pub path: String,
}

#[derive(Deserialize)]
pub struct Config {
    pub server: ServerSettings,
    pub mirror: Option<MirrorSettings>,
    pub rules: Vec<RulesSettings>,
}

pub fn load_config(path: PathBuf) -> Config {
    let config = std::fs::read_to_string(path).unwrap();
    toml::from_str(&config).unwrap()
}

pub fn load_config_relative(path: &str) -> Config {
    let current_dir = std::env::current_dir().unwrap();
    let path = current_dir.join(path);
    load_config(path)
}