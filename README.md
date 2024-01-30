# DM - Dotfiles Manager

~~WIP~~
I convert to NixOS, so this project will not update in future


[![Rust](https://github.com/mslxl/dm/actions/workflows/rust.yml/badge.svg)](https://github.com/mslxl/dm/actions/workflows/rust.yml)

Yet another dotfiles manager written in Rust, it named `dm` simply.

DM is designed to manager in multiply platform, and support many version control tools. In addition to above,
DM is integral with Github and WebDAV etc. web storage service, it can automatically sync files and manage it.

DM manage dotfiles, but it's not manage file directly, DM will manage group that composed by many files. It would proivde users a handy way to add, edit or just remove their configuration from DM.

## Building

1. Setup rust, you can follow [this instructions](https://rustup.rs/)
2. Clone this repository

```bash
$git clone git@github.com:mslxl/dm.git
```
3. Build

``` bash
$cargo build --release
```

4. Find the executable in `target/release/dm.exe`

- [X] Manage profile
- [ ] Manage files by group
- [ ] Basic file manage
- [ ] Basic dir manage
- [ ] Install file cross operation system
- [ ] Recongize special file
- [ ] Encrypt by gnuPGP
- [ ] Symbolic link
- [ ] Compress
- [ ] Hooks script
- [ ] Manual install script
- [ ] Template
- [ ] TUI
