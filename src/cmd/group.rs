use crate::group_cfg::{with_group_cfg_mut, GroupConfigurationReader, GroupConfigurationWriter};
use std::{process, fs, path::PathBuf, env};

use super::GroupCommands;

fn group_new(name: &str, desc: Option<&str>) {
    with_group_cfg_mut(|cfg| {
        if cfg.group_exists(name) {
            println!("Group already exists");
            process::exit(0);
        }
        cfg.group_add(name);
        if let Some(desc) = desc {
            cfg.group_setfield(name, "description", desc)
        }
    });
}

fn group_set_enable(name: &str, enable: bool) {
    with_group_cfg_mut(|cfg| {
        if !cfg.group_exists(name) {
            println!("Group not exists");
            process::exit(-1);
        }
        cfg.group_setfield(name, "enable", enable);
    });
}

pub fn group_commands(command: GroupCommands) {
    match command {
        GroupCommands::New { name, desc } => group_new(&name, desc.as_deref()),
        GroupCommands::Enable { name } => group_set_enable(&name, true),
        GroupCommands::Disable { name } => group_set_enable(&name, false),
        _ => todo!(),
    }
}

pub fn group_addfile(group: &str, encrypt: bool, file: &str){
    with_group_cfg_mut(|cfg| {
        let mut path = PathBuf::from(&file);
        if !path.exists() {
            panic!("{} not exists", file);
        }
        if path.is_relative() {
            path = env::current_dir().unwrap().join(path);
        }
        cfg.group_addfile(group, path.to_str().unwrap().to_owned());
    });
}
