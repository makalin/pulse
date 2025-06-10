use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use dirs::config_dir;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub api_url: String,
    pub theme: String,
    pub notifications_enabled: bool,
    pub auto_encrypt: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            api_url: "http://localhost:8080".to_string(),
            theme: "System".to_string(),
            notifications_enabled: true,
            auto_encrypt: true,
        }
    }
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = Self::get_config_path()?;
        
        if !config_path.exists() {
            return Ok(Self::default());
        }

        let config_str = fs::read_to_string(config_path)?;
        Ok(serde_json::from_str(&config_str)?)
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = Self::get_config_path()?;
        
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let config_str = serde_json::to_string_pretty(self)?;
        fs::write(config_path, config_str)?;
        
        Ok(())
    }

    fn get_config_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
        let mut path = config_dir().ok_or("Could not find config directory")?;
        path.push("pulse");
        path.push("config.json");
        Ok(path)
    }
} 