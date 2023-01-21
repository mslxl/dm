use std::io;
use std::os;
use std::path::Path;
use std::path::PathBuf;

#[cfg(target_family = "windows")]
pub fn symlink_file_specify<P: AsRef<Path>, Q: AsRef<Path>>(
    original: P,
    link: Q,
) -> io::Result<()> {
    os::windows::fs::symlink_file(original, link)
}

#[cfg(target_family = "unix") ]
pub fn symlink_file_specify<P: AsRef<Path>, Q: AsRef<Path>>(
    original: P,
    link: Q,
) -> io::Result<()> {
    os::unix::fs::symlink(original, link)
}

pub fn local_path_to_depositiory_path<P: AsRef<Path>>(group_name: &str, path: P) -> PathBuf{
    let mut path = path.as_ref().canonicalize().unwrap();
    if path.is_relative() {
        path = path.canonicalize().unwrap();
    }
    let path = path.to_str().unwrap();
    if path.starts_with("/") {
        // Unix path
        PathBuf::from(format!("depository/{}/ROOT/", group_name)).join(path.split_at(0).1)
    } else if path.starts_with("\\\\?\\") {
        // MSDOS path
        let filepath = &path[4..];
        let (disk, path) = filepath.split_once(":\\").unwrap();
        PathBuf::from(format!("depository\\{}\\{}\\{}", group_name, disk, path))
    } else {

        panic!("Unsupported filesystem")
    }
}