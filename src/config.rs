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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_default_config() {
        let config = AppConfig::default();
        assert_eq!(config.storage_type, StorageType::Local);
        assert_eq!(config.local_config.path, "~/.quill/storage/todos.json");
        assert_eq!(config.mongo_config.database, "quill");
    }

    #[test]
    fn test_storage_type_default() {
        let storage_type = StorageType::default();
        assert_eq!(storage_type, StorageType::Local);
    }

    #[test]
    fn test_expand_local_path_with_tilde() {
        let mut config = AppConfig::default();
        config.local_config.path = "~/test/path".to_string();
        let expanded = config.expand_local_path();
        assert!(!expanded.starts_with("~"));
    }

    #[test]
    fn test_expand_local_path_without_tilde() {
        let mut config = AppConfig::default();
        config.local_config.path = "/absolute/path".to_string();
        let expanded = config.expand_local_path();
        assert_eq!(expanded, "/absolute/path");
    }

    #[test]
    fn test_config_serialization() {
        let config = AppConfig::default();
        let json = serde_json::to_string(&config).unwrap();
        let deserialized: AppConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(config.storage_type, deserialized.storage_type);
    }

    #[test]
    fn test_save_and_load_config() {
        let temp_dir = TempDir::new().unwrap();
        
        // Set HOME environment variable and ensure it's used
        let original_home = std::env::var("HOME").ok();
        std::env::set_var("HOME", temp_dir.path());
        
        let mut original_config = AppConfig::default();
        original_config.mongo_config.database = "test_db".to_string();
        
        // Save the config using the save method
        original_config.save().unwrap();
        
        let loaded_config = AppConfig::load().unwrap();
        assert_eq!(loaded_config.mongo_config.database, "test_db");
        
        // Restore original HOME if it existed
        if let Some(home) = original_home {
            std::env::set_var("HOME", home);
        } else {
            std::env::remove_var("HOME");
        }
    }
}