pub mod file;

use std::{
    ffi::OsStr,
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
    ui::get_ui,
};

use self::file::{decrypt_file, encrypt_file, hash_file, is_file_same};

pub fn install_file(config: &dyn GroupFileConfigurationHelper) -> Result<(), Error> {
    let depository_path = get_depository_dir().join(config.get_depository_path().unwrap());
    let local_path = match config.get_local_path() {
        None => Err(Error::err(format!(
            "File local location {:?} does not register",
            depository_path
        )))?,
        Some(p) => PathBuf::from(p),
    };

    get_ui().msgbox_str(&format!("Install {:?}...", &depository_path));

    // Do nothing if use link
    if config.is_link() {
        get_ui().msgbox_str("Skip link file...");
        return Ok(());
    }

    // Decompress (decrypt if required)
    if config.is_compress() {
        get_ui().msgbox_str("Decompress from zstd file...");
        if config.is_encrypt() {
            // Decrypt
            let src = local_path.parent().unwrap().join(".dm_decrypt.zst");
            fs::copy(&depository_path, &src).unwrap();
            get_ui().msgbox_str("Decrypt file by invoking GPG...");
            decrypt_file(&src);

            // Decompress
            let src_file_stream = File::open(&src).unwrap();
            let local_file_stream = File::create(&local_path).unwrap();
            zstd::stream::copy_decode(src_file_stream, local_file_stream).unwrap();

            fs::remove_file(src).unwrap();
        } else {
            // Only Decompress
            let depository_file_stream = File::open(&depository_path).unwrap();
            let local_file_stream = File::create(&local_path).unwrap();
            zstd::stream::copy_decode(depository_file_stream, local_file_stream).unwrap();
        }
        return Ok(());
    }

    match fs::copy(depository_path, &local_path) {
        Ok(_) => {
            if config.is_encrypt() {
                get_ui().msgbox_str("Decrypt file by invoking GPG...");
                decrypt_file(&local_path);
            }

            Ok(())
        }
        Err(err) => Err(Error::err(err.to_string())),
    }
}

pub fn update_file(config: &dyn GroupFileConfigurationHelper) -> Result<(), Error> {
    if let Some(err) = check_configuration(config) {
        return Err(err);
    }
    // Gain resource path
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

    // Create link
    if config.is_link() {
        get_ui().msgbox_str("Create link...");
        if !depository_path.exists() {
            fs::rename(&local_path, &depository_path).unwrap();
            symlink_file_specify(&depository_path, &local_path).unwrap();
        }
        return Ok(());
    }

    // Compress file (encrypt if required)
    if config.is_compress() {
        get_ui().msgbox_str("Compress to zstd file...");
        let local_file_stream = File::open(&local_path).unwrap();

        // Record sha256 for update check
        let sha256 = hash_file(&local_path).unwrap();
        insert_or_update_file_sha256(&local_path, &sha256).unwrap();
        if !depository_path.parent().unwrap().exists() {
            fs::create_dir_all(depository_path.parent().unwrap()).unwrap();
        }
        let depository_file_stream = File::create(&depository_path).unwrap();
        zstd::stream::copy_encode(local_file_stream, depository_file_stream, 13).unwrap();

        if config.is_encrypt() {
            get_ui().msgbox_str("Encrypt compress file by invoking gpg...");
            encrypt_file(&depository_path, config.get_encrypt_recipient().unwrap());
        }

        return Ok(());
    }

    get_ui().msgbox_str("Copy file...");
    match fs::copy(&local_path, &depository_path) {
        Ok(_) => {
            if config.is_encrypt() {
                get_ui().msgbox_str("Encrypt compress file by invoking gpg...");
                // Record sha256 for update check
                let sha256 = hash_file(&local_path).unwrap();
                insert_or_update_file_sha256(&local_path, &sha256).unwrap();
                encrypt_file(&depository_path, config.get_encrypt_recipient().unwrap());
            }
            Ok(())
        }
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

    if config.is_compress() || config.is_encrypt() {
        return if let Some(hash) = cache::query_file_sha256(config.get_local_path().unwrap()) {
            let current_hash = hash_file(config.get_local_path().unwrap())?;
            Ok(hash != current_hash)
        } else {
            Ok(true)
        }
    }

    if !is_file_same(depository_path, local_path)? {
        return Ok(true);
    }
    Ok(false)
}
