use std::{
    fs::{File, self},
    io::{BufReader, Read},
    path::Path, ffi::OsStr,
};
use dunce::simplified;

use sha2::{Digest, Sha256};

use crate::error::Error;

pub fn is_file_same<P: AsRef<Path>, Q: AsRef<Path>>(
    local_file: P,
    depositiory_file: Q,
) -> Result<bool, Error> {
    let local_file = match File::open(local_file) {
        Err(err) => Err(Error::err(err.to_string()))?,
        Ok(value) => value,
    };
    let depositiory_file = match File::open(depositiory_file) {
        Err(err) => Err(Error::err(err.to_string()))?,
        Ok(value) => value,
    };

    if local_file.metadata().unwrap().len() != depositiory_file.metadata().unwrap().len() {
        return Ok(false);
    }

    let mut reader1 = BufReader::new(local_file);
    let mut reader2 = BufReader::new(depositiory_file);

    let mut buf1 = [0; 10000];
    let mut buf2 = [0; 10000];

    loop {
        let n1 = reader1.read(&mut buf1).unwrap();
        let n2 = reader2.read(&mut buf2).unwrap();
        if n1 == 0 && n2 == 0 {
            return Ok(true);
        }
        if n1 != n2 && buf1 != buf2 {
            return Ok(false);
        }
    }
}

pub fn hash_file<P: AsRef<Path>>(path: P) -> Result<String, Error> {
    let mut hasher = Sha256::new();
    let file = File::open(path).map_err(|v| Error::err(v.to_string()))?;
    let mut reader = BufReader::new(file);
    let mut buf = [0; 10000];
    loop {
        let n = reader.read(&mut buf).unwrap();
        if n == 0 {
            break;
        }
        hasher.update(&buf[0..n]);
    }
    let value = hasher.finalize();
    Ok(format!("{:X}", value))
}

pub fn encrypt_file<P: AsRef<Path>>(path: P, recipient: &str) {
    let origin = path.as_ref();
    let parent = origin.parent().unwrap();
    let temp = simplified(parent.join(".dm_decrypt").as_path()).to_path_buf();
    fs::rename(&origin, &temp).unwrap();

    std::process::Command::new("gpg")
        .args([
            OsStr::new("--output"),
            origin.as_os_str().clone(),
            OsStr::new("--encrypt"),
            OsStr::new("--recipient"),
            OsStr::new(recipient),
            temp.as_os_str().clone(),
        ])
        .spawn()
        .expect("Encrypt fail: can't spawn gpg")
        .wait()
        .expect("Encrypt fail!");
    fs::remove_file(temp).unwrap();
}

pub fn decrypt_file<P: AsRef<Path>>(path: P) {
    let origin = path.as_ref();
    let parent = origin.parent().unwrap();
    let temp = simplified(parent.join(".dm_decrypt").as_path()).to_path_buf();
    fs::rename(&origin, &temp).unwrap();

    let exit_code = std::process::Command::new("gpg")
        .args([
            OsStr::new("--output"),
            origin.as_os_str().clone(),
            OsStr::new("--decrypt"),
            temp.as_os_str().clone(),
        ])
        .spawn()
        .expect("Decrypt fail: can't spawn gpg")
        .wait()
        .expect("Decrypt fail!");
    if exit_code.code().unwrap_or(0)  != 0{
        panic!("Decrypt fail!");
    }
    fs::remove_file(temp).unwrap();
}
