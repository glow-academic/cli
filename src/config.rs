// config.rs — Configuration file support
//
// Priority chain (highest to lowest):
//   CLI flag > env var > config file > hardcoded default
//
// Config file location: ~/.config/glow/config.toml
// In Python terms: like a pyproject.toml or .env file
// In TS terms:     like a .rc file or config.json

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instance {
    pub url: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    pub api_url: Option<String>,
    pub glow_url: Option<String>,
    pub client_id: Option<String>,
    pub org_id: Option<String>,
    pub active_instance: Option<String>,
    #[serde(default)]
    pub instances: HashMap<String, Instance>,
}

impl Config {
    /// Load config from ~/.config/glow/config.toml.
    /// Returns default config if file doesn't exist.
    pub fn load() -> Result<Self> {
        let path = Self::config_path();
        if !path.exists() {
            return Ok(Config::default());
        }
        let contents = std::fs::read_to_string(&path)
            .with_context(|| format!("Failed to read config file: {}", path.display()))?;
        Self::parse(&contents)
            .with_context(|| format!("Failed to parse {}. Check the TOML syntax.", path.display()))
    }

    /// Parse config from a TOML string (public for testing)
    pub fn parse(toml_str: &str) -> Result<Self> {
        toml::from_str(toml_str).map_err(|e| anyhow::anyhow!("Invalid config: {}", e))
    }

    /// Save config to disk
    pub fn save(&self) -> Result<()> {
        let path = Self::config_path();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create config dir: {}", parent.display()))?;
        }
        let toml_str = toml::to_string_pretty(self).context("Failed to serialize config")?;
        std::fs::write(&path, toml_str)
            .with_context(|| format!("Failed to write config: {}", path.display()))
    }

    /// Returns the config file path
    pub fn config_path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("glow")
            .join("config.toml")
    }

    /// Resolve the active instance URL (if any)
    pub fn active_instance_url(&self) -> Option<&str> {
        let name = self.active_instance.as_deref()?;
        self.instances.get(name).map(|i| i.url.as_str())
    }
}

/// Resolve a value with priority: explicit (CLI/env) > config file > default
#[allow(dead_code)]
pub fn resolve_option(explicit: Option<&str>, config_val: Option<&str>, default: &str) -> String {
    explicit.or(config_val).unwrap_or(default).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_full_config() {
        let toml = r#"
            api_url = "https://custom.api.com"
            glow_url = "https://glow.example.com"
        "#;
        let config = Config::parse(toml).unwrap();
        assert_eq!(config.api_url.unwrap(), "https://custom.api.com");
        assert_eq!(config.glow_url.unwrap(), "https://glow.example.com");
    }

    #[test]
    fn test_parse_partial_config() {
        let toml = r#"api_url = "https://custom.api.com""#;
        let config = Config::parse(toml).unwrap();
        assert_eq!(config.api_url.unwrap(), "https://custom.api.com");
        assert!(config.glow_url.is_none());
    }

    #[test]
    fn test_parse_empty_config() {
        let config = Config::parse("").unwrap();
        assert!(config.api_url.is_none());
        assert!(config.glow_url.is_none());
    }

    #[test]
    fn test_parse_invalid_toml_returns_error() {
        let result = Config::parse("not valid [[[toml");
        assert!(result.is_err());
    }

    #[test]
    fn test_config_path_ends_with_expected() {
        let path = Config::config_path();
        assert!(path.ends_with("glow/config.toml"));
    }

    #[test]
    fn test_resolve_option_explicit_wins() {
        assert_eq!(
            resolve_option(Some("explicit"), Some("config"), "default"),
            "explicit"
        );
    }

    #[test]
    fn test_resolve_option_config_fallback() {
        assert_eq!(resolve_option(None, Some("config"), "default"), "config");
    }

    #[test]
    fn test_resolve_option_default_fallback() {
        assert_eq!(resolve_option(None, None, "default"), "default");
    }

    #[test]
    fn test_parse_config_with_instances() {
        let toml = r#"
            active_instance = "prod"

            [instances.prod]
            url = "https://glow.prod.example.com"

            [instances.staging]
            url = "https://glow.staging.example.com"
        "#;
        let config = Config::parse(toml).unwrap();
        assert_eq!(config.active_instance.as_deref(), Some("prod"));
        assert_eq!(config.instances.len(), 2);
        assert_eq!(
            config.instances["prod"].url,
            "https://glow.prod.example.com"
        );
    }

    #[test]
    fn test_active_instance_url() {
        let config = Config {
            active_instance: Some("prod".into()),
            instances: {
                let mut m = HashMap::new();
                m.insert(
                    "prod".into(),
                    Instance {
                        url: "https://glow.prod.example.com".into(),
                    },
                );
                m
            },
            ..Config::default()
        };
        assert_eq!(
            config.active_instance_url(),
            Some("https://glow.prod.example.com")
        );
    }

    #[test]
    fn test_active_instance_url_none_when_not_set() {
        let config = Config::default();
        assert_eq!(config.active_instance_url(), None);
    }

    #[test]
    fn test_config_save_and_load_roundtrip() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("config.toml");

        let config = Config {
            api_url: Some("https://api.test.com".into()),
            active_instance: Some("dev".into()),
            instances: {
                let mut m = HashMap::new();
                m.insert(
                    "dev".into(),
                    Instance {
                        url: "http://localhost:8000".into(),
                    },
                );
                m
            },
            ..Config::default()
        };

        // Save to temp path
        let toml_str = toml::to_string_pretty(&config).unwrap();
        std::fs::write(&path, &toml_str).unwrap();

        // Read back
        let contents = std::fs::read_to_string(&path).unwrap();
        let loaded = Config::parse(&contents).unwrap();
        assert_eq!(loaded.active_instance.as_deref(), Some("dev"));
        assert_eq!(loaded.instances["dev"].url, "http://localhost:8000");
    }
}
