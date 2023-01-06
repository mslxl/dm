use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;
use std::env;
use std::path::PathBuf;
use std::{fs, iter::empty};
use toml_edit::{array, table, value, Array, ArrayOfTablesIter, Document, Table, Value};

use crate::util::rel_to_depositiory_path;

use super::CfgError;

pub struct Transcation {
    depository_path: PathBuf,
    global_cfg: GlobalConfiguration,
    group_cfg_map: RefCell<HashMap<String, GroupConfiguration>>,
}

impl Transcation {
    const GENERAL_SECTION_KEY: &'static str = "general";

    /// Init transcation
    /// It will load global configuration file (dm.toml) automatically,
    /// if the file does not exists, it will create in memory
    ///
    /// All changes will be applied after call `save()` function
    pub fn new(path: PathBuf) -> Self {
        let global_cfg_path = path.join("dm.toml");
        let global_doc = if global_cfg_path.exists() {
            fs::read_to_string(&global_cfg_path)
                .expect("Fail to read global configuration file")
                .parse::<Document>()
                .expect("Invalid global configuration file")
        } else {
            let mut doc = Document::new();
            doc[Self::GENERAL_SECTION_KEY] = table();
            doc[Self::GENERAL_SECTION_KEY].as_table_mut().unwrap()["depository"] =
                value(Array::new());
            doc
        };
        Self {
            depository_path: path,
            global_cfg: GlobalConfiguration::new(global_cfg_path, global_doc),
            group_cfg_map: RefCell::new(HashMap::new()),
        }
    }
    /// Get global configuration helper
    pub fn global(&self) -> &GlobalConfiguration {
        &self.global_cfg
    }
    /// Get mutable global configuration helper
    pub fn global_mut(&mut self) -> &mut GlobalConfiguration {
        &mut self.global_cfg
    }

    /// Create new group
    /// attention! this function only register the name and create related configuration
    ///
    /// it will fail if the group name already exists
    ///
    /// All changes will be applied after called `save` function
    pub fn new_group(&mut self, name: String) -> Result<(), CfgError> {
        let path = self
            .depository_path
            .join("depository")
            .join(&name)
            .join("config.toml");
        if path.exists() {
            return Err(CfgError::new(format!("group {} already exists", name)));
        }

        self.global_cfg.document[Self::GENERAL_SECTION_KEY]
            .as_table_mut()
            .unwrap()["depository"]
            .as_array_mut()
            .unwrap()
            .push(&name);

        let cfg = GroupConfiguration::empty(path, &name);

        self.group_cfg_map.borrow_mut().insert(name, cfg);
        Ok(())
    }

    /// If group does not loaded, the function will try to load it, else it will do nothing
    /// it will failed if configuration does not exists
    fn ensure_group_in_map(&self, name: String) -> Option<()> {
        if self.group_cfg_map.borrow().contains_key(&name) {
            Some(())
        } else {
            let path = self
                .depository_path
                .join("depository")
                .join(&name)
                .join("config.toml");

            if path.exists() {
                let doc = fs::read_to_string(&path)
                    .expect(&format!(
                        "Fail to read {:?} group configuration file",
                        &path
                    ))
                    .parse::<Document>()
                    .expect(&format!("Invalid {:?} group configuration file", &path));

                self.group_cfg_map
                    .borrow_mut()
                    .insert(name, GroupConfiguration::new(path, doc));
                Some(())
            } else {
                None
            }
        }
    }

    /// Get mutable group configuration helper
    pub fn group_mut(&mut self, name: String) -> Option<RefMut<GroupConfiguration>> {
        self.ensure_group_in_map(name.clone())?;

        Some(RefMut::map(self.group_cfg_map.borrow_mut(), |it| {
            it.get_mut(&name).unwrap()
        }))
    }

    /// Get group configuration helper
    pub fn group(&self, name: String) -> Option<Ref<GroupConfiguration>> {
        self.ensure_group_in_map(name.clone())?;

        Some(Ref::map(self.group_cfg_map.borrow(), |it| {
            it.get(&name).unwrap()
        }))
    }

    /// Apply all changes
    pub fn save(&mut self) -> Result<(), CfgError> {
        if !&self.global_cfg.path.parent().unwrap().exists() {
            fs::create_dir_all(&self.global_cfg.path.parent().unwrap())
                .map_err(|err| CfgError::new(err.to_string()))?;
        }

        fs::write(&self.global_cfg.path, self.global_cfg.document.to_string())
            .map_err(|err| CfgError::new(err.to_string()))?;

        let map = self.group_cfg_map.borrow();
        for cfg in map.values() {
            let parent = cfg.path.parent().unwrap();
            if !parent.exists() {
                fs::create_dir_all(parent).map_err(|err| CfgError::new(err.to_string()))?;
            }
            fs::write(&cfg.path, cfg.doc.to_string())
                .map_err(|err| CfgError::new(err.to_string()))?;
        }

        Ok(())
    }
}

pub struct GlobalConfiguration {
    pub path: PathBuf,
    pub document: Document,
}

impl GlobalConfiguration {
    pub fn new(path: PathBuf, doc: Document) -> Self {
        Self {
            path,
            document: doc,
        }
    }

    const GROUP_SECTION_KEY: &'static str = "group";

    /// Add group name to global configuration
    /// it does not change filesystem
    pub fn add_group(&mut self, name: &str) -> Result<(), CfgError> {
        let doc = &mut self.document;
        if !doc.contains_array_of_tables(Self::GROUP_SECTION_KEY) {
            doc[Self::GROUP_SECTION_KEY] = array()
        }
        let array = doc[Self::GROUP_SECTION_KEY]
            .as_array_of_tables_mut()
            .unwrap();

        let mut table = Table::new();
        table["name"] = value(name);
        array.push(table);
        Ok(())
    }
    /// Remove group name from global configuration
    /// it does not change filesystem
    pub fn rm_group(&mut self, name: &str) -> Result<(), CfgError> {
        let doc = &mut self.document;
        if !doc.contains_array_of_tables(Self::GROUP_SECTION_KEY) {
            return Err(CfgError::new(format!("{} does not exists", name)));
        }
        let array = doc[Self::GROUP_SECTION_KEY]
            .as_array_of_tables_mut()
            .unwrap();
        let target = array.iter().enumerate().find(|(idx, group)| {
            group.contains_key("name") && group["name"].as_str().unwrap() == name
        });
        match target {
            None => Err(CfgError::new(format!("{} does not exists", name))),
            Some((idx, _)) => {
                array.remove(idx);
                Ok(())
            }
        }
    }
    pub fn iter(&self) -> ArrayOfTablesIter {
        let doc = &self.document;
        if doc.contains_array_of_tables(Self::GROUP_SECTION_KEY) {
            doc[Self::GROUP_SECTION_KEY]
                .as_array_of_tables()
                .unwrap()
                .iter()
        } else {
            Box::new(empty())
        }
    }
    pub fn find_group(&self, name: &str) -> Option<&Table> {
        let doc = &self.document;
        if !doc.contains_array_of_tables(Self::GROUP_SECTION_KEY) {
            return None;
        }
        doc[Self::GROUP_SECTION_KEY]
            .as_array_of_tables()
            .unwrap()
            .iter()
            .find(|group| group.contains_key("name") && group["name"].as_str().unwrap() == name)
    }
    fn find_group_mut(&mut self, name: &str) -> Option<&mut Table> {
        let doc = &mut self.document;
        if !doc.contains_array_of_tables(Self::GROUP_SECTION_KEY) {
            return None;
        }
        doc[Self::GROUP_SECTION_KEY]
            .as_array_of_tables_mut()
            .unwrap()
            .iter_mut()
            .find(|group| group.contains_key("name") && group["name"].as_str().unwrap() == name)
    }
}

pub struct GroupConfiguration {
    pub path: PathBuf,
    pub doc: Document,
}

impl GroupConfiguration {
    const GENERAL_SECTION_KEY: &'static str = "general";
    pub fn empty(path: PathBuf, name: &str) -> Self {
        let mut cfg = Self::new(path, Document::new());
        cfg.doc["general"] = table();
        cfg.doc["files"] = array();
        cfg.doc["directories"] = array();
        cfg.set_name(name);
        cfg
    }
    pub fn new(path: PathBuf, doc: Document) -> Self {
        Self { path, doc }
    }
    fn set_str_field(&mut self, key: &str, v: &str) {
        self.doc[Self::GENERAL_SECTION_KEY][key] = value(v);
    }
    fn get_str_field(&self, key: &str) -> Option<&str> {
        self.doc[Self::GENERAL_SECTION_KEY][key].as_str()
    }
    pub fn set_name(&mut self, name: &str) {
        self.set_str_field("name", name);
    }
    pub fn get_name(&self) -> Option<&str> {
        self.get_str_field("name")
    }
    pub fn set_desc(&mut self, desc: &str) {
        self.set_str_field("description", desc)
    }

    pub fn get_desc(&self) -> Option<&str> {
        self.get_str_field("description")
    }

    fn get_file_table_by_rel(&self, rel_path: &PathBuf) -> Option<&Table> {
        if !self.doc.contains_array_of_tables("files") {
            return None;
        }
        let rel_path = rel_path.to_str().unwrap();
        self.doc["files"]
            .as_array_of_tables()
            .unwrap()
            .iter()
            .find(|table| {
                table.contains_key("location") && table["location"].as_str().unwrap() == rel_path
            })
    }

    fn get_file_table_by_rel_mut(&mut self, rel_path: &PathBuf) -> Option<&mut Table> {
        if !self.doc.contains_array_of_tables("files") {
            return None;
        }
        let rel_path = rel_path.to_str().unwrap();
        self.doc["files"]
            .as_array_of_tables_mut()
            .unwrap()
            .iter_mut()
            .find(|table| {
                table.contains_key("location") && table["location"].as_str().unwrap() == rel_path
            })
    }

    fn get_file_table_by_abs(&self, abs: PathBuf) -> Option<&Table> {
        self.get_file_table_by_rel(&rel_to_depositiory_path(abs))
    }
    fn get_file_table_by_abs_mut(&mut self, abs: PathBuf) -> Option<&mut Table> {
        self.get_file_table_by_rel_mut(&rel_to_depositiory_path(abs))
    }

    pub fn add_file(
        &mut self,
        local_path: PathBuf,
    ) -> Result<GroupFileConfigurationMut<'_>, CfgError> {
        let depository_root: PathBuf = self.path.parent().unwrap().into();
        let record_path = rel_to_depositiory_path(local_path.clone());
        if let Some(_) = self.get_file_table_by_rel(&record_path) {
            return Err(CfgError::new(format!(
                "File '{:?}' already exists in {}",
                local_path,
                self.get_name().unwrap()
            )));
        }
        if !self.doc.contains_array_of_tables("files") {
            self.doc["files"] = array();
        }
        let array = self.doc["files"].as_array_of_tables_mut().unwrap();
        let mut table = Table::new();
        table["location"] = value(record_path.to_str().unwrap());
        table[env::consts::OS] = value(local_path.canonicalize().unwrap().to_str().unwrap());
        array.push(table);
        let table = array.get_mut(array.len() - 1).unwrap();
        Ok(GroupFileConfigurationMut::new(table))
    }
}

pub struct GroupFileConfiguration<'a> {
    attr: &'a Table,
}
pub struct GroupFileConfigurationMut<'a> {
    attr: &'a mut Table,
}
trait DerefGroupFileConfiguration {
    fn deref(&self) -> &Table;
}
trait DerefMutGroupFileConfiguration {
    fn deref_mut(&mut self) -> &mut Table;
}
impl DerefGroupFileConfiguration for GroupFileConfiguration<'_> {
    fn deref(&self) -> &Table {
        self.attr
    }
}

impl DerefGroupFileConfiguration for GroupFileConfigurationMut<'_> {
    fn deref(&self) -> &Table {
        self.attr
    }
}

impl DerefMutGroupFileConfiguration for GroupFileConfigurationMut<'_> {
    fn deref_mut(&mut self) -> &mut Table {
        self.attr
    }
}
pub trait GroupFileConfigurationHelper {
    const GENERAL_SECTION_KEY: &'static str = "general";
    fn get_field(&self, key: &str) -> Option<&Value>;

    fn is_encrypt(&self) -> bool;
}

impl<T> GroupFileConfigurationHelper for T
where
    T: DerefGroupFileConfiguration,
{
    fn get_field(&self, key: &str) -> Option<&Value> {
        self.deref().get(key).and_then(|item| item.as_value())
    }

    fn is_encrypt(&self) -> bool {
        self.get_field("encrypt")
            .and_then(|value| value.as_bool())
            .unwrap_or(false)
    }
}
pub trait GroupFileConfigurationHelperMut: GroupFileConfigurationHelper {
    fn set_field<V: Into<Value>>(&mut self, key: &str, v: V);
    fn set_encrypt(&mut self, v: bool);
}

impl<T> GroupFileConfigurationHelperMut for T
where
    T: DerefMutGroupFileConfiguration + GroupFileConfigurationHelper + DerefGroupFileConfiguration,
{
    fn set_field<V: Into<Value>>(&mut self, key: &str, v: V) {
        self.deref_mut()[key] = value(v)
    }
    fn set_encrypt(&mut self, v: bool) {
        self.set_field("encrypt", v)
    }
}

impl<'a> GroupFileConfiguration<'a> {
    fn new(attr: &'a Table) -> Self {
        Self { attr }
    }
}
impl<'a> GroupFileConfigurationMut<'a> {
    fn new(attr: &'a mut Table) -> Self {
        Self { attr }
    }
}
