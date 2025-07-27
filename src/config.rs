use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StorageType {
    Local,
    MongoDB,
}

impl Default for StorageType {
    fn default() -> Self {
        Self::Local
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalConfig {
    pub path: String,
}

impl Default for LocalConfig {
    fn default() -> Self {
        Self {
            path: "~/.quill/storage/todos.json".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MongoConfig {
    pub connection_string: String,
    pub database: String,
    pub collection: String,
}

impl Default for MongoConfig {
    fn default() -> Self {
        Self {
            connection_string: "mongodb://localhost:27017".to_string(),
            database: "quill".to_string(),
            collection: "tasks".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    #[serde(default)]
    pub storage_type: StorageType,
    #[serde(default)]
    pub local_config: LocalConfig,
    #[serde(default)]
    pub mongo_config: MongoConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            storage_type: StorageType::Local,
            local_config: LocalConfig::default(),
            mongo_config: MongoConfig::default(),
        }
    }
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        let path = Self::get_config_path()?;
        
        if path.exists() {
            let content = fs::read_to_string(&path)?;
            let config: AppConfig = serde_json::from_str(&content)?;
            Ok(config)
        } else {
            Ok(Self::default())
        }
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::get_config_path()?;
        
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        let content = serde_json::to_string_pretty(self)?;
        fs::write(&path, content)?;
        Ok(())
    }

    fn get_config_path() -> Result<PathBuf> {
        let mut path = dirs::home_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?;
        path.push(".quill");
        path.push("config.json");
        Ok(path)
    }

    pub fn expand_local_path(&self) -> String {
        if self.local_config.path.starts_with("~/") {
            if let Some(home) = dirs::home_dir() {
                return self.local_config.path.replacen("~", &home.to_string_lossy(), 1);
            }
        }
        self.local_config.path.clone()
    }
}