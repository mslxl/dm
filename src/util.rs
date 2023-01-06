use std::path::PathBuf;

pub fn rel_to_depositiory_path(path: PathBuf) -> PathBuf {
    let mut path = path;
    if path.is_relative() {
        path = path.canonicalize().unwrap();
    }
    let path = path.to_str().unwrap();
    if path.starts_with("/") {
        // Unix path
        PathBuf::from("ROOT/").join(path.split_at(0).1)
    } else if path.starts_with("\\\\?\\"){
        // MSDOS path
        let filepath = &path[4..];
        let (disk, path) = filepath.split_once(":\\").unwrap();
        PathBuf::from(format!("{}\\{}", disk, path))
    }else{
      todo!("Unsupported filesystem")
    }
}
