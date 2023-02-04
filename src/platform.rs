use std::io;
use std::os;
use std::path::Path;

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
