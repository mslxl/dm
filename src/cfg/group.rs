use std::{path::PathBuf, str::FromStr};

use toml_edit::{array, table, value, Document, Table, TomlError, Value};

use crate::util::to_depositiory_path;

pub struct GroupConfiguration(Document);

impl FromStr for GroupConfiguration {
    type Err = TomlError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse::<Document>()?))
    }
}

impl ToString for GroupConfiguration {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl From<Document> for GroupConfiguration {
    fn from(doc: Document) -> Self {
        Self(doc)
    }
}
impl Into<Document> for GroupConfiguration {
    fn into(self) -> Document {
        self.0
    }
}

impl GroupConfiguration {
    pub fn group<'a>(&'a self, name: &'a str) -> GroupConfigurationItem<'a> {
        GroupConfigurationItem(name, self)
    }
    pub fn group_mut<'a>(&'a mut self, name: &'a str) -> GroupConfigurationItemMut<'a> {
        GroupConfigurationItemMut(name, self)
    }
}

pub struct GroupConfigurationItem<'a>(&'a str, &'a GroupConfiguration);
pub struct GroupConfigurationItemMut<'a>(&'a str, &'a mut GroupConfiguration);
trait DerefGroupConfig {
    fn deref<'a>(&'a self) -> (&'a str, &'a GroupConfiguration);
}
trait DerefGroupConfigMut: DerefGroupConfig {
    fn deref_mut<'a>(&'a mut self) -> (&'a str, &'a mut GroupConfiguration);
}
impl DerefGroupConfig for GroupConfigurationItem<'_> {
    fn deref<'a>(&'a self) -> (&'a str, &'a GroupConfiguration) {
        (self.0, self.1)
    }
}
impl DerefGroupConfig for GroupConfigurationItemMut<'_> {
    fn deref<'a>(&'a self) -> (&'a str, &'a GroupConfiguration) {
        (self.0, self.1)
    }
}
impl DerefGroupConfigMut for GroupConfigurationItemMut<'_> {
    fn deref_mut<'a>(&'a mut self) -> (&'a str, &'a mut GroupConfiguration) {
        (self.0, self.1)
    }
}

pub trait GroupConfigurationItemReader {
    fn exists(&self) -> bool;
}

impl<T> GroupConfigurationItemReader for T
where
    T: DerefGroupConfig,
{
    fn exists(&self) -> bool {
        let (group_name, doc) = self.deref();
        doc.0.contains_table(group_name)
    }
}

pub trait GroupConfigurationItemWriter: GroupConfigurationItemReader {
    fn create(&mut self) -> Result<(), ()>;
    fn set_field<T>(&mut self, fieldname: &str, v: T) -> Result<(), ()>
    where
        T: Into<Value>;

    fn add_file(&mut self, filename: &PathBuf, encrypt: bool) -> Result<String, ()>;
}

impl<T> GroupConfigurationItemWriter for T
where
    T: DerefGroupConfigMut + GroupConfigurationItemReader,
{
    fn create(&mut self) -> Result<(), ()> {
        let (group_name, doc) = self.deref_mut();
        if !doc.0.contains_table(group_name) {
            doc.0[group_name] = table();
            doc.0[group_name]["enable"] = value(true);
            Ok(())
        } else {
            Err(())
        }
    }

    fn add_file(&mut self, filename: &PathBuf, encrypt: bool) -> Result<String, ()> {
        let (group_name, doc) = self.deref_mut();

        let group = match doc.0[group_name].as_table_mut() {
            None => return Err(()),
            Some(group) => group,
        };

        if !group.contains_key("files") {
            group["files"] = array();
        }

        let mut table = Table::new();
        let depositiory_path = to_depositiory_path(filename.clone())
            .to_str()
            .unwrap()
            .to_owned();

        table.insert("file", value(depositiory_path.clone()));
        table.insert(
            std::env::consts::OS,
            value(filename.to_str().unwrap().to_owned()),
        );
        table.insert("encrypt", value(encrypt));

        group["files"].as_array_of_tables_mut().unwrap().push(table);
        Ok(depositiory_path)
    }

    fn set_field<V>(&mut self, fieldname: &str, v: V) -> Result<(), ()>
    where
        V: Into<Value>,
    {
        let (group_name, doc) = self.deref_mut();

        let group = match doc.0[group_name].as_table_mut() {
            None => return Err(()),
            Some(group) => group,
        };
        group[fieldname] = value(v);
        Ok(())
    }
}
