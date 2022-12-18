use std::{path::PathBuf, os};

pub fn to_depositiory_path(path: PathBuf) -> PathBuf{
  let mut path = path;
  if path.is_relative() {
    path = path.canonicalize().unwrap();
  }
  let path =path.to_str().unwrap();
  if path.starts_with("/") {
    // Unix path
    PathBuf::from("root/").join(path.split_at(0).1)
  } else {
    // MSDOS path
    let (disk, path) = path.split_once(":\\").unwrap();
    PathBuf::from(format!("{}\\{}",disk, path))
  }
}
