pub mod file;

use std::{fs, path::PathBuf};

use crate::{
    cfg::file::GroupFileConfigurationHelper, checker::check_configuration, env::get_depository_dir,
    error::Error,
};

use self::file::is_file_same;

pub fn update_file(config: &dyn GroupFileConfigurationHelper) -> Result<(), Error> {
    if config.is_link() {
        return Ok(());
    }
    let depository_path = get_depository_dir().join(config.get_depository_path().unwrap());
    if !depository_path.parent().unwrap().exists() {
        fs::create_dir_all(depository_path.parent().unwrap()).unwrap();
    }
    let local_path = match config.get_local_path() {
        None => Err(Error::err(format!(
            "File {:?} does not exists",
            depository_path
        )))?,
        Some(p) => PathBuf::from(p),
    };
    match fs::copy(local_path, depository_path) {
        Ok(_) => Ok(()),
        Err(err) => Err(Error::err(err.to_string())),
    }
}

pub fn is_file_updatable(config: &dyn GroupFileConfigurationHelper) -> Result<bool, Error> {
    if let Some(err) = check_configuration(config) {
        return Err(err);
    }
    if config.is_link() {
        return Ok(true);
    }

    let depository_path = get_depository_dir().join(config.get_depository_path().unwrap());
    let local_path = match config.get_local_path() {
        None => Err(Error::err(format!(
            "{0:?} has not been registered on {1}(missing {1} field)",
            depository_path,
            std::env::consts::OS
        )))?,
        Some(p) => PathBuf::from(p),
    };

    if !depository_path.exists() || !is_file_same(depository_path, local_path)? {
        return Ok(true);
    }

    Ok(false)
}
