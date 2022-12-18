use std::{fs, sync::Mutex};

use once_cell::sync::Lazy;

use crate::env::get_depository_config_filename;

use self::group::GroupConfiguration;

pub mod group;

static GROUP_CONFIG_INIT: Mutex<bool> = Mutex::new(false);

pub static GROUP_CONFIG: Lazy<Mutex<GroupConfiguration>> = Lazy::new(|| {
    let path = get_depository_config_filename();
    let cfg = fs::read_to_string(&path)
        .expect(&format!("Error occured when reading {:?}", &path))
        .parse::<GroupConfiguration>()
        .expect(&format!("Invalid config file {:?}", &path));

    *GROUP_CONFIG_INIT.lock().unwrap() = true;
    Mutex::new(cfg)
});

pub fn save_config() {
    if *GROUP_CONFIG_INIT.lock().unwrap() {
        let path = get_depository_config_filename();
        fs::write(path, GROUP_CONFIG.lock().unwrap().to_string())
            .expect("Error occured when saving depository config");
    }
}
