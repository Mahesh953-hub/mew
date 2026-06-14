use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;

use crate::paths::MewPaths;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MewConfig {
    pub identity: IdentityConfig,
    pub style: StyleConfig,
    pub agent: AgentConfig,
    pub tokens: TokenConfig,
    pub providers: ProviderConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityConfig {
    pub display_name: String,
    pub persona: String,
    pub rename_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StyleConfig {
    pub theme: String,
    pub animations: bool,
    pub emoji: bool,
    pub kaomoji: bool,
    pub density: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    pub default_mode: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenConfig {
    pub mode: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub default: String,
}

impl Default for MewConfig {
    fn default() -> Self {
        Self {
            identity: IdentityConfig {
                display_name: "mew".to_string(),
                persona: "cute".to_string(),
                rename_enabled: true,
            },
            style: StyleConfig {
                theme: "crush-catppuccin".to_string(),
                animations: true,
                emoji: true,
                kaomoji: true,
                density: "cozy".to_string(),
            },
            agent: AgentConfig {
                default_mode: "ask".to_string(),
            },
            tokens: TokenConfig {
                mode: "balanced".to_string(),
            },
            providers: ProviderConfig {
                default: "openai/codex".to_string(),
            },
        }
    }
}

impl MewConfig {
    pub fn load_or_create(paths: &MewPaths) -> Result<Self> {
        if !paths.config_file.exists() {
            let cfg = Self::default();
            cfg.save(paths)?;
            return Ok(cfg);
        }

        let raw = fs::read_to_string(&paths.config_file)?;
        let cfg: Self = toml::from_str(&raw)?;
        Ok(cfg)
    }

    pub fn save(&self, paths: &MewPaths) -> Result<()> {
        fs::create_dir_all(&paths.config_dir)?;
        let raw = toml::to_string_pretty(self)?;
        fs::write(&paths.config_file, raw)?;
        Ok(())
    }
}
