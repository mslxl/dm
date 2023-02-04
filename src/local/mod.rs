use miette::Result;
use miette::{Context, IntoDiagnostic};
use rust_i18n::t;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

use crate::{env::get_depository_dir, error::DMError};

pub mod group;
pub mod profile;

struct Transcation {
    group: HashMap<String, TomlGroup>,
    global: TomlGlobal,
}

impl Transcation {
    fn lock() -> Result<()> {
        let lock_file = get_depository_dir().join(".lock");
        if lock_file.exists() {
            Err(DMError::LockError {
                msg: t!("error.transcation.lock.msg"),
                advice: t!(
                    "error.transcation.lock.advice",
                    lock = lock_file.to_str().unwrap()
                ),
            }
            .into())
        } else {
            std::fs::write(lock_file, t!("lock.content")).into_diagnostic()?;
            Ok(())
        }
    }

    fn unlock() -> Result<()> {
        let lock_file = get_depository_dir().join(".lock");
        if lock_file.exists() {
            std::fs::remove_file(lock_file).into_diagnostic()?;
        }
        Ok(())
    }

    fn start() -> Result<Self> {
        Self::lock().wrap_err(t!("error.ctx.transcation.init"))?;
        let global_toml_path = get_global_toml_path();
        let global = if !global_toml_path.exists() {
            TomlGlobal::default()
        } else {
            let toml = std::fs::read_to_string(global_toml_path)
                .into_diagnostic()
                .wrap_err(t!("error.ctx.transcation.init"))?;
            toml_edit::de::from_str(&toml)
                .into_diagnostic()
                .wrap_err(t!("error.ctx.transcation.init"))?
        };
        Ok(Self {
            group: HashMap::new(),
            global,
        })
    }

    fn global(&mut self) -> &mut TomlGlobal {
        &mut self.global
    }

    fn commit(self) -> Result<()> {
        let global_toml_path = get_global_toml_path();
        std::fs::write(
            global_toml_path,
            toml_edit::ser::to_string_pretty(&self.global)
                .into_diagnostic()
                .wrap_err(t!("error.ctx.transcation.commit"))?,
        )
        .into_diagnostic()
        .wrap_err(t!("error.ctx.transcation.commit"))?;

        Self::unlock().wrap_err(t!("error.ctx.transcation.commit"))?;
        Ok(())
    }
}

impl Drop for Transcation {
    fn drop(&mut self) {
        Self::unlock().unwrap();
    }
}

#[derive(Serialize, Deserialize)]
struct TomlGlobalProfileEntry {
    name: String,
    group: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct TomlGlobalGeneral {
    profile: Vec<TomlGlobalProfileEntry>,
    group: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct TomlGlobal {
    general: TomlGlobalGeneral,
}

impl TomlGlobalProfileEntry {
    pub fn new(name: String) -> Self {
        Self {
            name,
            group: vec![],
        }
    }
}
impl Default for TomlGlobalGeneral {
    fn default() -> Self {
        Self {
            profile: vec![TomlGlobalProfileEntry::new(String::from("default"))],
            group: vec![],
        }
    }
}

impl Default for TomlGlobal {
    fn default() -> Self {
        Self {
            general: TomlGlobalGeneral::default(),
        }
    }
}

fn get_global_toml_path() -> PathBuf {
    get_depository_dir().join("dm.toml")
}

#[derive(Serialize, Deserialize)]
struct TomlGroup {}
