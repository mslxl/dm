use std::path::{PathBuf, Path};
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

pub fn get_depository_dir() -> PathBuf {
    let dir = get_xdg_data_dir();
    let dir = dir.join("dm");
    if !dir.exists() {
        fs::create_dir(&dir).expect("Can't create depository directory");
    }
    dir
}

pub fn get_group_dir(group_name: &str) -> PathBuf {
    get_depository_dir().join(format!("{}/{}", "depository", group_name))
}

pub fn to_depositiory_path<P: AsRef<Path>>(path: P) -> PathBuf{
    let path = dunce::canonicalize(path).unwrap();
    let path = path.to_str().unwrap();
    if path.starts_with("/") {
        // Unix path
        PathBuf::from("ROOT/").join(path.split_at(0).1)
    } else if path.starts_with("\\\\?\\") {
        // MSDOS path
        let filepath = &path[4..];
        let (disk, path) = filepath.split_once(":\\").unwrap();
        PathBuf::from(format!("{}\\{}",disk, path))
    } else {
        panic!("Unsupported filesystem")
    }
}
