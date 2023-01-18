use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use toml_edit::{table, value, Array, Document};

use super::global::GlobalConfiguration;
use super::group::GroupConfiguration;
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
    pub fn new_group(&mut self, name: String) -> Result<RefMut<GroupConfiguration>, CfgError> {
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

        self.group_cfg_map.borrow_mut().insert(name.clone(), cfg);
        Ok(RefMut::map(self.group_cfg_map.borrow_mut(), |it| {
            it.get_mut(&name).unwrap()
        }))
    }

    pub fn groups(&self) -> impl Iterator<Item = &str> {
        self.global_cfg.document[Self::GENERAL_SECTION_KEY]
            .as_table()
            .unwrap()["depository"]
            .as_array()
            .unwrap()
            .iter()
            .map(|item| item.as_str().unwrap())
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
