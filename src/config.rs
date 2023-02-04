
use miette::{IntoDiagnostic, Result};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use crate::env::get_xdg_config_dir;

#[derive(Serialize, Deserialize)]
pub struct DMConfiguration {
    pub using_profile: String,
}

impl Default for DMConfiguration {
    fn default() -> Self {
        Self {
            using_profile: String::from("default"),
        }
    }
}

impl DMConfiguration {
    pub fn save(&self) -> Result<()> {
        let config_file = get_xdg_config_dir().join("dm.toml");
        std::fs::write(
            config_file,
            toml_edit::ser::to_string_pretty(self).into_diagnostic()?,
        )
        .into_diagnostic()
    }
}

fn read_config_file() -> Result<DMConfiguration> {
    let config_file = get_xdg_config_dir().join("dm.toml");
    if !config_file.exists() {
        Ok(DMConfiguration::default())
    } else {
        toml_edit::de::from_str(&std::fs::read_to_string(config_file).into_diagnostic()?)
            .into_diagnostic()
    }
}

pub static config: Lazy<Mutex<DMConfiguration>> = Lazy::new(|| Mutex::new(read_config_file().unwrap()));
