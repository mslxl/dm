pub mod file;

use std::{
    fs::{self, File},
    path::PathBuf,
};

use crate::{
    cache::{self, insert_or_update_file_sha256},
    cfg::file::GroupFileConfigurationHelper,
    checker::check_configuration,
    env::get_depository_dir,
    error::Error,
    platform::symlink_file_specify,
};

use self::file::{hash_file, is_file_same};

pub fn install_file(config: &dyn GroupFileConfigurationHelper) -> Result<(), Error> {
    let depository_path = get_depository_dir().join(config.get_depository_path().unwrap());
    let local_path = match config.get_local_path() {
        None => Err(Error::err(format!(
            "File local location {:?} does not register",
            depository_path
        )))?,
        Some(p) => PathBuf::from(p),
    };
    if config.is_link() {
        return Ok(());
    }

    if config.is_compress() {
        let depository_file_stream = File::open(&depository_path).unwrap();
        let local_file_stream = File::create(local_path).unwrap();
        zstd::stream::copy_decode(depository_file_stream, local_file_stream).unwrap();
        return Ok(());
    }

    match fs::copy(depository_path, local_path) {
        Ok(_) => Ok(()),
        Err(err) => Err(Error::err(err.to_string())),
    }
}

pub fn update_file(config: &dyn GroupFileConfigurationHelper) -> Result<(), Error> {
    let depository_path = get_depository_dir().join(config.get_depository_path().unwrap());
    let local_path = match config.get_local_path() {
        None => Err(Error::err(format!(
            "File local location {:?} does not register",
            depository_path
        )))?,
        Some(p) => PathBuf::from(p),
    };
    if !depository_path.parent().unwrap().exists() {
        fs::create_dir_all(depository_path.parent().unwrap()).unwrap();
    }

    if config.is_link() {
        if !depository_path.exists() {
            fs::rename(&local_path, &depository_path).unwrap();
            symlink_file_specify(&depository_path, &local_path).unwrap();
        }
        return Ok(());
    }

    if config.is_compress() {
        let local_file_stream = File::open(&local_path).unwrap();

        let sha256 = hash_file(&local_path).unwrap();
        insert_or_update_file_sha256(&local_path, &sha256).unwrap();
        if !depository_path.parent().unwrap().exists() {
            fs::create_dir_all(depository_path.parent().unwrap()).unwrap();
        }
        let depository_file_stream = File::create(depository_path).unwrap();
        zstd::stream::copy_encode(local_file_stream, depository_file_stream, 13).unwrap();
        return Ok(());
    }

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

    if !depository_path.exists() {
        return Ok(true);
    }

    if config.is_compress() {
        if let Some(hash) = cache::query_file_sha256(config.get_local_path().unwrap()) {
            let current_hash = hash_file(config.get_local_path().unwrap())?;
            return Ok(hash != current_hash);
        } else {
            return Ok(true);
        }
    }

    if !is_file_same(depository_path, local_path)? {
        return Ok(true);
    }
    Ok(false)
}
