use directories::BaseDirs;
use miette::{IntoDiagnostic, Result};
use rust_i18n::t;
use std::path::{Path, PathBuf};
use std::{env, fs};

use crate::error::DMError;

fn env_option_to_result<T>(result: Option<T>) -> Result<T> {
    result
        .ok_or(DMError::EnvError {
            msg: t!("error.env.dir_not_certain.msg"),
            advice: Some(t!("error.env.dir_not_certain.advice")),
        })
        .into_diagnostic()
}

pub fn get_app_data_dir() -> Result<PathBuf> {
    let path = env::var("DM_DATA")
        .ok()
        .map(|p| PathBuf::from(p))
        .or(BaseDirs::new().map(|q| q.data_local_dir().to_path_buf().join("dm")));
    if let Some(path) = &path{
        if !path.exists() {
            fs::create_dir_all(path).into_diagnostic()?;
        }
    }
    env_option_to_result(path)
}

pub fn get_app_config_file() -> Result<PathBuf> {
    let path = env::var("DM_CONFIG_FILE")
        .ok()
        .map(|p| PathBuf::from(p))
        .or(BaseDirs::new().map(|q| q.config_dir().to_path_buf().join("dm.toml")));
    env_option_to_result(path)
}

pub fn get_group_dir(group_name: &str) -> Result<PathBuf> {
    Ok(get_app_data_dir()?.join(format!("{}/{}", "depository", group_name)))
}

pub fn to_depositiory_path<P: AsRef<Path>>(path: P) -> PathBuf {
    let path = dunce::canonicalize(path).unwrap();
    let path = path.to_str().unwrap();
    if path.starts_with("/") {
        // Unix path
        PathBuf::from("ROOT/").join(path.split_at(0).1)
    } else if path.starts_with("\\\\?\\") {
        // MSDOS path
        let filepath = &path[4..];
        let (disk, path) = filepath.split_once(":\\").unwrap();
        PathBuf::from(format!("{}\\{}", disk, path))
    } else {
        panic!("Unsupported filesystem")
    }
}
