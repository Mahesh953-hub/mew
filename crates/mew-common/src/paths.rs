use anyhow::{anyhow, Result};
use directories::ProjectDirs;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct MewPaths {
    pub config_dir: PathBuf,
    pub config_file: PathBuf,
    pub data_dir: PathBuf,
    pub cache_dir: PathBuf,
}

impl MewPaths {
    pub fn discover() -> Result<Self> {
        let dirs = ProjectDirs::from("dev", "mew", "mew")
            .ok_or_else(|| anyhow!("could not discover user directories"))?;

        let config_dir = dirs.config_dir().to_path_buf();
        let data_dir = dirs.data_dir().to_path_buf();
        let cache_dir = dirs.cache_dir().to_path_buf();

        Ok(Self {
            config_file: config_dir.join("config.toml"),
            config_dir,
            data_dir,
            cache_dir,
        })
    }
}
