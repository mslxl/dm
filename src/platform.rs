use std::io;
use std::os;
use std::path::Path;

#[cfg(target_os = "windows")]
pub fn symlink_file_specify<P: AsRef<Path>, Q: AsRef<Path>>(
    original: P,
    link: Q,
) -> io::Result<()> {
    os::windows::fs::symlink_file(original, link)
}

#[cfg(target_os = "linux")]
pub fn symlink_file_specify<P: AsRef<Path>, Q: AsRef<Path>>(
    original: P,
    link: Q,
) -> io::Result<()> {
    os::unix::fs::symlink(original, link)
}

#[cfg(target_os = "macos")]
pub fn symlink_file_specify<P: AsRef<Path>, Q: AsRef<Path>>(
    original: P,
    link: Q,
) -> io::Result<()> {
    os::unix::fs::symlink(original, link)
}

#[cfg(target_os = "android")]
pub fn symlink_file_specify<P: AsRef<Path>, Q: AsRef<Path>>(
    original: P,
    link: Q,
) -> io::Result<()> {
    os::unix::fs::symlink(original, link)
}

#[cfg(target_os = "freebsd")]
pub fn symlink_file_specify<P: AsRef<Path>, Q: AsRef<Path>>(
    original: P,
    link: Q,
) -> io::Result<()> {
    os::unix::fs::symlink(original, link)
}
