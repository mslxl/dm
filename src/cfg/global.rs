use std::{path::PathBuf, iter::empty};

use toml_edit::{Document, Table, ArrayOfTablesIter, array, value};

use crate::error::Error;

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
    pub fn add_group(&mut self, name: &str) -> Result<(), Error> {
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
    pub fn rm_group(&mut self, name: &str) -> Result<(), Error> {
        let doc = &mut self.document;
        if !doc.contains_array_of_tables(Self::GROUP_SECTION_KEY) {
            return Err(Error::err(format!("{} does not exists when program tries to remove group", name)));
        }
        let array = doc[Self::GROUP_SECTION_KEY]
            .as_array_of_tables_mut()
            .unwrap();
        let target = array.iter().enumerate().find(|(_, group)| {
            group.contains_key("name") && group["name"].as_str().unwrap() == name
        });
        match target {
            None => Err(Error::err(format!("{} does not exists when program tries to remove group", name))),
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
