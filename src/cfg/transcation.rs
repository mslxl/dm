use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;
use std::path::PathBuf;
use std::{fs, iter::empty};
use toml_edit::{array, table, value, ArrayOfTablesIter, Document, Table};

use super::CfgError;

pub struct Transcation {
    depository_path: PathBuf,
    global_cfg: GlobalConfiguration,
    group_cfg_map: RefCell<HashMap<String, GroupConfiguration>>,
}

impl Transcation {
    pub fn new(path: PathBuf) -> Self {
        let global_cfg_path = path.join("dm.toml");
        let global_doc = if global_cfg_path.exists() {
            fs::read_to_string(&global_cfg_path)
                .expect("Fail to read global configuration file")
                .parse::<Document>()
                .expect("Invalid global configuration file")
        } else {
            Document::new()
        };
        Self {
            depository_path: path,
            global_cfg: GlobalConfiguration::new(global_cfg_path, global_doc),
            group_cfg_map: RefCell::new(HashMap::new()),
        }
    }
    pub fn global(&self) -> &GlobalConfiguration {
        &self.global_cfg
    }
    pub fn global_mut(&mut self) -> &mut GlobalConfiguration {
        &mut self.global_cfg
    }

    pub fn new_group(&mut self, name: String) -> Result<(), CfgError> {
        let path = self
            .depository_path
            .join("depository")
            .join(&name)
            .join("config.toml");
        if path.exists() {
            return Err(CfgError::new(format!("group {} already exists", name)));
        }
        let cfg = GroupConfiguration::empty(path, &name);
        self.group_cfg_map.borrow_mut().insert(name, cfg);
        Ok(())
    }

    fn ensure_group_in_map(&self, name: String) -> Option<()> {
        let path = self.depository_path.join("depository").join(&name);
        if path.exists() {
            if !self.group_cfg_map.borrow().contains_key(&name) {
                let doc = fs::read_to_string(&path)
                    .expect(&format!(
                        "Fail to read {:?} group configuration file",
                        &path
                    ))
                    .parse::<Document>()
                    .expect(&format!("Invalid {:?} group configuration file", &path));

                self.group_cfg_map
                    .borrow_mut()
                    .insert(name.clone(), GroupConfiguration::new(path, doc));
            }
            Some(())
        } else {
            None
        }
    }

    pub fn group_mut(&mut self, name: String) -> Option<RefMut<GroupConfiguration>> {
        self.ensure_group_in_map(name.clone())?;

        Some(RefMut::map(self.group_cfg_map.borrow_mut(), |it| {
            it.get_mut(&name).unwrap()
        }))
    }

    pub fn group(&self, name: String) -> Option<Ref<GroupConfiguration>> {
        self.ensure_group_in_map(name.clone())?;

        Some(Ref::map(self.group_cfg_map.borrow(), |it| {
            it.get(&name).unwrap()
        }))
    }

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
}
