use std::path::PathBuf;
use toml_edit::{array, table, value, Document, Table};

use crate::util::rel_to_depositiory_path;

use super::file::{
    GroupFileConfiguration, GroupFileConfigurationHelper, GroupFileConfigurationHelperMut,
    GroupFileConfigurationMut,
};
use super::CfgError;
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

    pub fn files(&self) -> impl Iterator<Item = GroupFileConfiguration> {
        self.doc["files"]
            .as_array_of_tables()
            .unwrap()
            .iter()
            .map(GroupFileConfiguration::from)
    }

    pub fn files_mut(&mut self) -> impl Iterator<Item = GroupFileConfigurationMut> {
        self.doc["files"]
            .as_array_of_tables_mut()
            .unwrap()
            .iter_mut()
            .map(GroupFileConfigurationMut::from)
    }

    fn get_file_table_by_rel(&self, rel_path: &PathBuf) -> Option<GroupFileConfiguration> {
        if !self.doc.contains_array_of_tables("files") {
            return None;
        }
        let rel_path = rel_path.to_str().unwrap();
        self.files().find(|helper| {
            helper
                .get_depository_path()
                .map(|v| v == rel_path)
                .unwrap_or(false)
        })
    }

    fn get_file_table_by_rel_mut(
        &mut self,
        rel_path: &PathBuf,
    ) -> Option<GroupFileConfigurationMut> {
        if !self.doc.contains_array_of_tables("files") {
            return None;
        }
        let rel_path = rel_path.to_str().unwrap();
        self.files_mut().find(|helper| {
            helper
                .get_depository_path()
                .map(|v| v == rel_path)
                .unwrap_or(false)
        })
    }

    fn get_file_table_by_abs(&self, abs: PathBuf) -> Option<GroupFileConfiguration> {
        self.get_file_table_by_rel(&rel_to_depositiory_path(self.get_name().unwrap(), abs))
    }
    fn get_file_table_by_abs_mut(&mut self, abs: PathBuf) -> Option<GroupFileConfigurationMut> {
        self.get_file_table_by_rel_mut(&rel_to_depositiory_path(self.get_name().unwrap(), abs))
    }

    pub fn add_file(
        &mut self,
        local_path: PathBuf,
    ) -> Result<GroupFileConfigurationMut<'_>, CfgError> {
        // Wether the file ready exists
        let record_path = rel_to_depositiory_path(self.get_name().unwrap(), local_path.clone());
        if let Some(_) = self.get_file_table_by_rel(&record_path) {
            return Err(CfgError::new(format!(
                "File '{:?}' already exists in {}",
                local_path,
                self.get_name().unwrap()
            )));
        }
        // Create files section
        if !self.doc.contains_array_of_tables("files") {
            self.doc["files"] = array();
        }

        // Add configuration
        let array = self.doc["files"].as_array_of_tables_mut().unwrap();
        let mut table = Table::new();
        let mut helper = GroupFileConfigurationMut::from(&mut table);
        helper.set_depository_path(record_path.to_str().unwrap());
        helper.set_local_path(local_path.canonicalize().unwrap().to_str().unwrap());
        array.push(table);
        let table = array.get_mut(array.len() - 1).unwrap();
        Ok(GroupFileConfigurationMut::from(table))
    }
}
