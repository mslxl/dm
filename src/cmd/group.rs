use std::path::PathBuf;

use crate::cfg::group::GroupConfigurationItemWriter;
use crate::cfg::GROUP_CONFIG;

use super::GroupCommands;

fn group_new(name: &str, desc: Option<&str>) {
    let mut cfg = GROUP_CONFIG.lock().unwrap();
    let mut group = cfg.group_mut(name);
    group.create().expect("Group already exists");
    if let Some(desc) = desc {
        group.set_field("description", desc).unwrap();
    }
}

fn group_set_enable(name: &str, enable: bool) {
    let mut cfg = GROUP_CONFIG.lock().unwrap();
    let mut group = cfg.group_mut(name);
    group.set_field("enable", enable).expect("Group not exists");
}

pub fn group_commands(command: GroupCommands) {
    match command {
        GroupCommands::New { name, desc } => group_new(&name, desc.as_deref()),
        GroupCommands::Enable { name } => group_set_enable(&name, true),
        GroupCommands::Disable { name } => group_set_enable(&name, false),
        _ => todo!(),
    }
}

pub fn group_addfile(group: &str, encrypt: bool, file: &str) {
    let mut cfg = GROUP_CONFIG.lock().unwrap();
    let mut group = cfg.group_mut(group);

    let path = PathBuf::from(file)
        .canonicalize()
        .expect(&format!("{} not exists", file));
    group
        .add_file(&path, encrypt)
        .expect(&format!("Error occured when register {:?}", &path));
}
