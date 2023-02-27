use miette::{IntoDiagnostic, Result, Context};
use once_cell::sync::Lazy;
use rust_i18n::t;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use crate::env::get_app_config_file;

#[derive(Serialize, Deserialize)]
pub struct DMConfiguration {
    pub using_profile: String,
    pub locale: String,
}

impl Default for DMConfiguration {
    fn default() -> Self {
        Self {
            using_profile: String::from("default"),
            locale: String::from("en"),
        }
    }
}

impl DMConfiguration {
    pub fn save(&self) -> Result<()> {
        let config_file = get_app_config_file()?;
        std::fs::write(
            config_file,
            toml_edit::ser::to_string_pretty(self).into_diagnostic()?,
        )
        .into_diagnostic()
    }
}

fn read_config_file() -> Result<DMConfiguration> {
    let config_file = get_app_config_file()?;
    if !config_file.exists() {
        Ok(DMConfiguration::default())
    } else {
        toml_edit::de::from_str(&std::fs::read_to_string(config_file).into_diagnostic()?)
            .into_diagnostic()
            .wrap_err(t!("error.ctx.serde.deserializing"))
    }
}

pub static CONFIG: Lazy<Mutex<DMConfiguration>> =
    Lazy::new(|| Mutex::new(read_config_file().unwrap()));
