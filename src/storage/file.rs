use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

use sha2::{Sha256, Digest};

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

pub fn hash_file<P:AsRef<Path>>(path: P) -> Result<String, Error>{
  let mut hasher = Sha256::new();
  let file = File::open(path).map_err(|v| Error::err(v.to_string()))?;
  let mut reader = BufReader::new(file);
  let mut buf = [0; 10000];
  loop{
    let n = reader.read(&mut buf).unwrap();
    if n == 0 {
      break;
    }
    hasher.update(&buf[0..n]);
  };
  let value = hasher.finalize();
  Ok(format!("{:X}", value))
}