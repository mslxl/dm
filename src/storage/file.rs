use std::{
    fs::{self, File},
    io::{BufReader, Read},
    path::{Path, PathBuf},
};

use crate::{cfg::file::GroupFileConfigurationHelper, env::get_depository_dir, platform::symlink_file_specify};

use super::StorageError;

fn compare_file<P: AsRef<Path>, Q: AsRef<Path>>(
    local_file: P,
    depositiory_file: Q,
) -> Result<bool, StorageError> {
    let local_file = File::open(local_file).map_err(StorageError::from)?;
    let depositiory_file = File::open(depositiory_file).map_err(StorageError::from)?;

    if local_file.metadata().unwrap().len() != depositiory_file.metadata().unwrap().len() {
        return Ok(false);
    }

    let mut reader1 = BufReader::new(local_file);
    let mut reader2 = BufReader::new(depositiory_file);

    let mut buf1 = [0; 10000];
    let mut buf2 = [0; 10000];

    loop {
        let n1 = reader1.read(&mut buf1).map_err(StorageError::from)?;
        let n2 = reader2.read(&mut buf2).map_err(StorageError::from)?;
        if n1 == 0 && n2 == 0 {
            return Ok(true);
        }
        if n1 != n2 && buf1 != buf2 {
            return Ok(false);
        }
    }
}


pub fn updatable(config: &dyn GroupFileConfigurationHelper) -> Result<bool, StorageError> {
    let depository_path = get_depository_dir().join(config.get_depository_path().unwrap());
    let local_path = PathBuf::from(config.get_local_path().unwrap());
    Ok(!depository_path.try_exists().map_err(StorageError::from)?
        || !compare_file(&local_path, &depository_path)?)
}

pub fn update_file(config: &dyn GroupFileConfigurationHelper) -> Result<(), StorageError> {
    if config.is_encrypt() && config.is_hard_link() {
        return Err(StorageError::new(String::from(
            "Can't use encrypt and hard-link at same time",
        )));
    }
    if config.is_soft_link() && config.is_hard_link() {
        return Err(StorageError::new(String::from(
            "Can't use soft-link and hard-link at same time",
        )));
    }
    let depository_path = get_depository_dir().join(config.get_depository_path().unwrap());
    let parent = depository_path.parent().unwrap();
    if !parent.try_exists().map_err(StorageError::from)? {
        fs::create_dir_all(parent).map_err(StorageError::from)?;
    }
    let local_path = PathBuf::from(config.get_local_path().unwrap());

    if config.is_hard_link() {
        if !depository_path.try_exists().map_err(StorageError::from)? {
            fs::hard_link(local_path, depository_path).map_err(StorageError::from)?;
        }
        return Ok(());
    }

    if config.is_soft_link() {
        if !depository_path.try_exists().map_err(StorageError::from)? {
            symlink_file_specify(local_path, depository_path).map_err(StorageError::from)?;
        }
        return Ok(());
    }

    if !depository_path.try_exists().map_err(StorageError::from)?
        || !compare_file(&local_path, &depository_path)?
    {
        if depository_path.exists() {
            fs::remove_file(&depository_path).map_err(StorageError::from)?;
        }
        fs::copy(local_path, depository_path).map_err(StorageError::from)?;
    }

    return Ok(());
}
