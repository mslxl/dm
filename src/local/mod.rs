use miette::Context;
use miette::IntoDiagnostic;
use miette::Result;
use rust_i18n::t;
use serde::de::Visitor;
use serde::{Deserialize, Serialize};
use std::cell::{Ref, RefCell, RefMut};
use std::{collections::HashMap, path::PathBuf};

use crate::env::get_app_data_dir;
use crate::env::get_group_dir;
use crate::env::SpecDir;
use crate::error::DMError;
use crate::error::GroupErrorKind;

pub mod profile;
pub mod file;
pub mod group;
pub mod db;
mod updater;

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
            toml_edit::de::from_str(&toml)
                .into_diagnostic()
                .wrap_err(t!("error.ctx.serde.deserializing"))?
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
                .into_diagnostic()
                .wrap_err(t!("error.ctx.serde.deserializing"))?;
                self.group.borrow_mut().insert(name, ins);
            } else {
                self.group
                    .borrow_mut()
                    .insert(name.clone(), TomlGroup::new(name));
            }
        }
        Ok(())
    }

    pub fn group(&self, name: &str) -> Result<Ref<TomlGroup>> {
        if !self.group.borrow().contains_key(name) {
            self.load_group_toml(name.to_string())?;
        }
        let borrow = self.group.borrow();

        if !borrow.contains_key(name) {
            return Err(DMError::GroupError {
                kind: GroupErrorKind::NotExists,
                msg: t!("error.group.not_exists"),
                advice: None,
            })
            .into_diagnostic();
        }
        let r = Ref::map(borrow, |map| map.get(name).unwrap());
        Ok(r)
    }

    pub fn group_mut(&mut self, name: &str) -> Result<RefMut<TomlGroup>> {
        if !self.group.borrow().contains_key(name) {
            self.load_group_toml(name.to_string())?;
        }
        let borrow = self.group.borrow_mut();

        if !borrow.contains_key(name) {
            return Err(DMError::GroupError {
                kind: GroupErrorKind::NotExists,
                msg: t!("error.group.not_exists"),
                advice: None,
            })
            .into_diagnostic();
        }
        let r = RefMut::map(borrow, |map| map.get_mut(name).unwrap());
        Ok(r)
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
            toml_edit::ser::to_string_pretty(&self.global)
                .into_diagnostic()
                .wrap_err(t!("error.ctx.serde.serializing"))?,
        )
        .into_diagnostic()?;
        // Save group manifest
        for (name, v) in self.group.borrow().iter() {
            let value = toml_edit::ser::to_string_pretty(v)
                .into_diagnostic()
                .wrap_err(t!("error.ctx.serde.serializing"))?;
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

#[derive(Debug, Clone)]
pub enum DMPath {
    Normal(String),
    Dynamic(Vec<String>),
}

impl DMPath {
    pub fn parse(&self, env: &SpecDir) -> Result<PathBuf> {
        match self {
            DMPath::Normal(dir) => Ok(PathBuf::from(dir)),
            DMPath::Dynamic(data) => {
                if data.is_empty() {
                    Err(DMError::EnvError {
                        msg: t!("error.env.empty_path"),
                        advice: None,
                    })
                    .into_diagnostic()?;
                }
                let prefix = data.first().unwrap();
                let prefix = match prefix.chars().nth(0).unwrap() {
                    '$' => PathBuf::from(std::env::var(&prefix[1..]).into_diagnostic()?),
                    '#' => PathBuf::from(env.get_path(prefix).ok_or(DMError::EnvError {
                        msg: t!("error.env.env_not_found", name = prefix),
                        advice: None,
                    })?),
                    _ => PathBuf::from(prefix),
                };
                let mut path = PathBuf::from(prefix);
                for item in &data[1..] {
                    match item.chars().nth(0).unwrap() {
                        '#' => Err(DMError::EnvError {
                            msg: t!("error.env.prefix_not_first"),
                            advice: None,
                        })
                        .into_diagnostic()?,
                        '$' => path = path.join(std::env::var(&item[1..]).into_diagnostic()?),
                        _ => path = path.join(item),
                    }
                }
                Ok(path)
            }
        }
    }
}

impl Serialize for DMPath {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            DMPath::Normal(str) => serializer.serialize_str(str),
            DMPath::Dynamic(list) => list.serialize(serializer),
        }
    }
}

struct DMPathVisitor;
impl<'de> Visitor<'de> for DMPathVisitor {
    type Value = DMPath;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string or an array of string")
    }

    fn visit_string<E>(self, v: String) -> std::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(DMPath::Normal(v))
    }

    fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(DMPath::Normal(v.to_string()))
    }
    fn visit_seq<A>(self, mut seq: A) -> std::result::Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let mut data = vec![];
        loop {
            if let Some(v) = seq.next_element().unwrap() {
                data.push(v)
            } else {
                break;
            }
        }
        Ok(DMPath::Dynamic(data))
    }
}

impl<'de> Deserialize<'de> for DMPath {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(DMPathVisitor)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ItemEntryKind {
    File,
    Dir,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct TomlItemEntry {
    /// 标明是 File 还是 Dir
    kind: ItemEntryKind,
    /// 在仓库中的路径
    path: String,
    /// 是否使用外部脚本进行同步/安装管理
    manaul: bool,
    /// 在不同平台下的安装路径
    install: HashMap<String, DMPath>,
}

impl TomlItemEntry {
    pub fn new(kind: ItemEntryKind, path: String, manaul: bool) -> Self {
        Self {
            kind,
            path,
            manaul,
            install: HashMap::new(),
        }
    }
    /// Get install path in current platform
    pub fn get_platform_install_path(&self) -> Option<&DMPath> {
        self.install.get(&std::env::consts::OS.to_string())
    }
    /// Set install path in current platform
    pub fn insert_platform_install_path(&mut self, path: DMPath) {
        self.install.insert(std::env::consts::OS.to_string(), path);
    }
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
