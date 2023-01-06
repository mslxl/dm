use std::path::PathBuf;
use std::{env, fs};

pub fn get_xdg_data_dir() -> PathBuf {
    let path = env::var("DM_DATA_HOME")
        .or(env::var("XDG_DATA_HOME"))
        .or(env::var("APPDATA"))
        .unwrap();
    PathBuf::from(path)
}

pub fn get_xdg_config_dir() -> PathBuf {
    let path = env::var("DM_CONFIG_HOME")
        .or(env::var("XDG_CONFIG_HOME"))
        .or(env::var("LOCALAPPDATA"))
        .unwrap();
    PathBuf::from(path)
}

pub fn get_cache_dir() -> PathBuf {
    let path = env::var("XDG_CACHE_HOME").or(env::var("TEMP")).unwrap();
    PathBuf::from(path)
}

pub fn get_depository_dir() -> PathBuf {
    let dir = get_xdg_data_dir();
    let dir = dir.join("dm");
    if !dir.exists() {
        fs::create_dir(&dir).expect("Can't create depository directory");
    }
    dir
}

