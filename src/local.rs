use std::path::PathBuf;
use std::{env, fs};
use toml_edit::Document;

use crate::group_cfg::{GroupConfiguration, GroupConfigurationMut};

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

pub fn get_local_config_filename() -> PathBuf {
    let file = get_xdg_config_dir().join("dm.toml");
    if !file.exists() {
        fs::File::create(&file).unwrap();
    }
    file
}

pub fn get_depository_config_filename() -> PathBuf {
    let file = get_depository_dir().join("depository.toml");
    if !file.exists() {
        fs::File::create(&file).unwrap();
    }
    file
}

pub fn with_toml_cfg<F>(path: PathBuf, block: F)
where
    F: FnOnce(&Document),
{
    let cfg = fs::read_to_string(&path)
        .expect(&format!(
            "Error occured when reading {}",
            path.to_str().unwrap()
        ))
        .parse::<Document>()
        .expect(&format!("Invalid config file {}", path.to_str().unwrap()));
    block(&cfg);
}

pub fn with_toml_cfg_mut<F>(path: PathBuf, block: F)
where
    F: FnOnce(&mut Document),
{
    let mut cfg = fs::read_to_string(&path)
        .expect(&format!(
            "Error occured when reading {}",
            path.to_str().unwrap()
        ))
        .parse::<Document>()
        .expect(&format!("Invalid config file {}", path.to_str().unwrap()));
    block(&mut cfg);
    cfg.fmt();
    fs::write(&path, cfg.to_string()).expect(&format!(
        "Error occured when writing {}",
        path.to_str().unwrap()
    ));
}

pub fn with_group_cfg<F>(block: F)
where
    F: FnOnce(&GroupConfiguration),
{
    with_toml_cfg(get_depository_config_filename(), |doc|{
        let cfg = GroupConfiguration::from(doc);
        block(&cfg)
    });
}

pub fn with_group_cfg_mut<F>(block: F)
where
    F: FnOnce(&mut GroupConfigurationMut),
{
    with_toml_cfg_mut(get_depository_config_filename(), |doc| {
        let mut cfg = GroupConfigurationMut::from(doc);
        block(&mut cfg);
    })
}
