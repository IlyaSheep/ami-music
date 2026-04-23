use std::{
    fs::{self, create_dir_all},
    path::{Path, PathBuf},
};

use anyhow::Result;
use serde::Deserialize;

use crate::APP_NAME;

#[derive(Deserialize)]
pub struct Config {
    pub library: LibraryConfig,
}

#[derive(Deserialize)]
pub struct LibraryConfig {
    pub directories: Vec<PathBuf>,
}

impl Config {
    pub fn load() -> Result<Self> {
        let path = dirs::config_dir()
            .unwrap()
            .join(format!("{}.toml", APP_NAME));

        if !path.exists() {
            Self::write_default_config(&path)?;
        }
        let text = std::fs::read_to_string(&path)?;
        Ok(toml::from_str(&text)?)
    }

    fn write_default_config(path: &Path) -> Result<()> {
        let audio_dir = dirs::audio_dir().unwrap();
        let config = format!(
            r#"[library]
        directories = [
            "{}",
        ]
            "#,
            audio_dir.display()
        );

        if let Some(parent) = path.parent() {
            create_dir_all(parent)?;
        }
        fs::write(path, config)?;

        Ok(())
    }
}
