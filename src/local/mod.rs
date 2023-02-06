use miette::IntoDiagnostic;
use miette::Result;
use rust_i18n::t;
use serde::{Deserialize, Serialize};
use std::cell::{Ref, RefCell, RefMut};
use std::{collections::HashMap, path::PathBuf};

use crate::env::get_app_data_dir;
use crate::env::get_group_dir;
use crate::error::DMError;
use crate::error::GroupErrorKind;

pub mod file;
pub mod group;
pub mod profile;

struct Transaction {
    group: RefCell<HashMap<String, TomlGroup>>,
    global: TomlGlobal,
}

impl Transaction {
    fn lock() -> Result<()> {
        let lock_file = get_app_data_dir()?.join(".lock");
        if lock_file.exists() {
            Err(DMError::LockError {
                msg: t!("error.transcation.lock.msg"),
                advice: t!(
                    "error.transcation.lock.advice",
                    lock = lock_file.to_str().unwrap()
                ),
            }
            .into())
        } else {
            std::fs::write(lock_file, t!("lock.content")).into_diagnostic()?;
            Ok(())
        }
    }

    fn unlock() -> Result<()> {
        let lock_file = get_app_data_dir()?.join(".lock");
        if lock_file.exists() {
            std::fs::remove_file(lock_file).into_diagnostic()?;
        }
        Ok(())
    }

    pub fn start() -> Result<Self> {
        Self::lock()?;
        let global_toml_path = get_global_toml_path()?;
        let global = if !global_toml_path.exists() {
            TomlGlobal::default()
        } else {
            let toml = std::fs::read_to_string(global_toml_path).into_diagnostic()?;
            toml_edit::de::from_str(&toml).into_diagnostic()?
        };
        Ok(Self {
            group: RefCell::new(HashMap::new()),
            global,
        })
    }

    pub fn global(&self) -> &TomlGlobal {
        &self.global
    }

    pub fn global_mut(&mut self) -> &mut TomlGlobal {
        &mut self.global
    }

    fn load_group_toml(&self, name: String) -> Result<()> {
        let dir = get_group_dir(&name)?;
        if !dir.exists() {
            self.group
                .borrow_mut()
                .insert(name.clone(), TomlGroup::new(name));
        } else {
            let file = dir.join("manifest.toml");
            if file.exists() {
                let ins = toml_edit::de::from_str::<TomlGroup>(
                    &std::fs::read_to_string(file).into_diagnostic()?,
                )
                .into_diagnostic()?;
                self.group.borrow_mut().insert(name, ins);
            } else {
                self.group
                    .borrow_mut()
                    .insert(name.clone(), TomlGroup::new(name));
            }
        }
        Ok(())
    }

    pub fn group(&self, name: &str) -> Result<Option<Ref<TomlGroup>>> {
        let borrow = self.group.borrow();
        if !borrow.contains_key(name) {
            self.load_group_toml(name.to_string())?;
        }

        if !borrow.contains_key(name) {
            return Ok(None);
        }
        let r = Ref::map(borrow, |map| map.get(name).unwrap());
        Ok(Some(r))
    }

    pub fn group_mut(&mut self, name: &str) -> Result<Option<RefMut<TomlGroup>>> {
        let borrow = self.group.borrow_mut();
        if !borrow.contains_key(name) {
            self.load_group_toml(name.to_string())?;
        }

        if !borrow.contains_key(name) {
            return Ok(None);
        }
        let r = RefMut::map(borrow, |map| map.get_mut(name).unwrap());
        Ok(Some(r))
    }

    pub fn create_group(&mut self, name: &str) -> Result<RefMut<TomlGroup>> {
        let mut borrow = self.group.borrow_mut();
        if borrow.contains_key(name) || self.global.registery.group.contains(&name.to_string()) {
            Err(DMError::GroupError {
                kind: GroupErrorKind::DuplicateCreate,
                msg: t!("error.group.duplicate.msg", name = name),
                advice: None,
            })
            .into_diagnostic()?;
        }
        borrow.insert(name.to_string(), TomlGroup::new(name.to_string()));
        self.global.registery.group.push(name.to_string());

        Ok(RefMut::map(borrow, |map| map.get_mut(name).unwrap()))
    }

    pub fn commit(mut self) -> Result<()> {
        let global_toml_path = get_global_toml_path()?;
        // Save global configuration
        std::fs::write(
            global_toml_path,
            toml_edit::ser::to_string_pretty(&self.global).into_diagnostic()?,
        )
        .into_diagnostic()?;
        // Save group manifest
        for (name, v) in self.group.borrow().iter() {
            let value = toml_edit::ser::to_string_pretty(v).into_diagnostic()?;
            let dir = get_group_dir(name)?;
            if !dir.exists() {
                std::fs::create_dir_all(&dir).into_diagnostic()?;
            }
            let manifest_path = dir.join("manifest.toml");
            std::fs::write(manifest_path, value).into_diagnostic()?;
        }
        Self::unlock()?;
        Ok(())
    }
}

impl Drop for Transaction {
    fn drop(&mut self) {
        Self::unlock().unwrap();
    }
}

#[derive(Serialize, Deserialize)]
struct TomlGlobalProfileEntry {
    name: String,
    group: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct TomlGlobalRegistery {
    profile: Vec<TomlGlobalProfileEntry>,
    group: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct TomlGlobal {
    registery: TomlGlobalRegistery,
}

impl TomlGlobalProfileEntry {
    pub fn new(name: String) -> Self {
        Self {
            name,
            group: vec![],
        }
    }
}
impl Default for TomlGlobalRegistery {
    fn default() -> Self {
        Self {
            profile: vec![TomlGlobalProfileEntry::new(String::from("default"))],
            group: vec![],
        }
    }
}

impl Default for TomlGlobal {
    fn default() -> Self {
        Self {
            registery: TomlGlobalRegistery::default(),
        }
    }
}

fn get_global_toml_path() -> Result<PathBuf> {
    Ok(get_app_data_dir()?.join("dm.toml"))
}

#[derive(Serialize, Deserialize)]
enum DMPath {
    Normal(String),
    Dynamic(Vec<String>)
}

#[derive(Serialize, Deserialize)]
enum TomlItemEntry {
    File {
        path: String,
        manaul: bool,
        install: HashMap<String, DMPath>,
    },
    Dir {
        path: String,
        manaul: bool,
        install: HashMap<String, DMPath>,
    },
}

#[derive(Serialize, Deserialize)]
struct TomlGroup {
    name: String,
    description: Option<String>,
    files: Vec<TomlItemEntry>,
}

impl TomlGroup {
    pub fn new(name: String) -> Self {
        Self {
            name,
            description: None,
            files: vec![],
        }
    }
}
