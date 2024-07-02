use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub api_id: String,
    pub api_hash: String,
    pub phone_number: String,
    pub dc: String,
    pub session_path: String,
    pub boto_tokeno: String, 
}

impl Config {
    pub fn load() -> Self {
        let config_path = "config.toml";
        
        if !Path::new(config_path).exists() {
            panic!("Configuration file not found: {}", config_path);
        }
        
        let config_str = fs::read_to_string(config_path).expect("Failed to read config.toml");
        let config: toml::Value = toml::from_str(&config_str).expect("Failed to parse configuration");
        
        let prod_config = config.get("prod").expect("Missing [prod] section in config.toml").clone();
        let config: Config = prod_config.try_into().expect("Failed to deserialize [prod] section");
        
        println!("Loaded configuration: {:?}", config);
        config
    }
}