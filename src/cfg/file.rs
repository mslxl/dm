use std::env;
use toml_edit::{value, Table, Value};

pub struct GroupFileConfiguration<'a> {
    attr: &'a Table,
    group_name: String,
}
pub struct GroupFileConfigurationMut<'a> {
    attr: &'a mut Table,
    group_name: String,
}
trait DerefGroupFileConfiguration {
    fn deref(&self) -> &Table;
    fn get_group_name(&self) -> &str;
}

trait DerefMutGroupFileConfiguration {
    fn deref_mut(&mut self) -> &mut Table;
}
impl DerefGroupFileConfiguration for GroupFileConfiguration<'_> {
    fn deref(&self) -> &Table {
        self.attr
    }

    fn get_group_name(&self) -> &str {
        &self.group_name
    }
}

impl DerefGroupFileConfiguration for GroupFileConfigurationMut<'_> {
    fn deref(&self) -> &Table {
        self.attr
    }
    fn get_group_name(&self) -> &str {
        &self.group_name
    }
}

impl DerefMutGroupFileConfiguration for GroupFileConfigurationMut<'_> {
    fn deref_mut(&mut self) -> &mut Table {
        self.attr
    }
}
pub trait GroupFileConfigurationHelper {
    fn get_group_name(&self) -> &str;
    fn get_field(&self, key: &str) -> Option<&Value>;

    fn get_local_path(&self) -> Option<&str>{
        self.get_field(env::consts::OS)
            .and_then(|value| value.as_str())
    }
    fn get_depository_path(&self) -> Option<&str>{
        self.get_field("location").and_then(|value| value.as_str())
    }

    fn is_compress(&self) -> bool{
        self.get_field("compress")
            .and_then(|value| value.as_bool())
            .unwrap_or(false)
    }
    fn is_encrypt(&self) -> bool{
        self.get_field("encrypt")
            .and_then(|value| value.as_bool())
            .unwrap_or(false)
    }

    fn is_hard_link(&self) -> bool{
        self.get_field("hard_link")
            .and_then(|value| value.as_bool())
            .unwrap_or(false)
    }
    fn is_soft_link(&self) -> bool{
        self.get_field("soft_link")
            .and_then(|value| value.as_bool())
            .unwrap_or(true)
    }
    fn is_link(&self) -> bool {
        return self.is_hard_link() || self.is_soft_link();
    }
}

impl<T> GroupFileConfigurationHelper for T
where
    T: DerefGroupFileConfiguration,
{
    fn get_group_name(&self) -> &str {
        DerefGroupFileConfiguration::get_group_name(self)
    }
    fn get_field(&self, key: &str) -> Option<&Value> {
        self.deref().get(key).and_then(|item| item.as_value())
    }
}
pub trait GroupFileConfigurationHelperMut: GroupFileConfigurationHelper {
    fn set_field<V: Into<Value>>(&mut self, key: &str, v: V);

    fn set_local_path(&mut self, path: &str){
        self.set_field(env::consts::OS, path)
    }
    fn set_depository_path(&mut self, path: &str){
        self.set_field("location", path)
    }

    fn set_encrypt(&mut self, v: bool){
        self.set_field("encrypt", v)
    }
    fn set_hard_link(&mut self, v: bool){
        self.set_field("hard_link", v)
    }
    fn set_soft_link(&mut self, v: bool){
        self.set_field("soft_link", v)
    }
    fn set_compress(&mut self, v:bool){
        self.set_field("compress", v)
    }
}

impl<T> GroupFileConfigurationHelperMut for T
where
    T: DerefMutGroupFileConfiguration + GroupFileConfigurationHelper + DerefGroupFileConfiguration,
{
    fn set_field<V: Into<Value>>(&mut self, key: &str, v: V) {
        self.deref_mut()[key] = value(v)
    }
}

impl<'a> GroupFileConfigurationMut<'a> {
    pub fn new(attr: &'a mut Table, group_name: String) -> Self {
        Self { attr, group_name }
    }
}

impl<'a> GroupFileConfiguration<'a> {
    pub fn new(attr: &'a Table, group_name: String) -> Self {
        Self { attr, group_name }
    }
}
