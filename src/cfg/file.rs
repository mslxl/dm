use std::env;
use toml_edit::{value, Table, Value};

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
    fn get_field(&self, key: &str) -> Option<&Value>;

    fn get_local_path(&self) -> Option<&str>;
    fn get_depository_path(&self) -> Option<&str>;

    fn is_encrypt(&self) -> bool;
    fn is_hard_link(&self) -> bool;
    fn is_soft_link(&self) -> bool;
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
    fn is_hard_link(&self) -> bool {
        self.get_field("hard_link")
            .and_then(|value| value.as_bool())
            .unwrap_or(true)
    }

    fn is_soft_link(&self) -> bool {
        self.get_field("soft_link")
            .and_then(|value| value.as_bool())
            .unwrap_or(true)
    }
    fn get_local_path(&self) -> Option<&str> {
        self.get_field(env::consts::OS)
            .and_then(|value| value.as_str())
    }

    fn get_depository_path(&self) -> Option<&str> {
        self.get_field("location").and_then(|value| value.as_str())
    }
}
pub trait GroupFileConfigurationHelperMut: GroupFileConfigurationHelper {
    fn set_field<V: Into<Value>>(&mut self, key: &str, v: V);

    fn set_local_path(&mut self, path: &str);
    fn set_depository_path(&mut self, path: &str);

    fn set_encrypt(&mut self, v: bool);
    fn set_hard_link(&mut self, v: bool);
    fn set_soft_link(&mut self, v: bool);
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
    fn set_hard_link(&mut self, v: bool) {
        self.set_field("hard_link", v)
    }

    fn set_soft_link(&mut self, v: bool) {
        self.set_field("soft_link", v)
    }
    fn set_local_path(&mut self, path: &str) {
        self.set_field(env::consts::OS, path)
    }

    fn set_depository_path(&mut self, path: &str) {
        self.set_field("location", path)
    }
}

impl<'a> From<&'a mut Table> for GroupFileConfigurationMut<'a> {
    fn from(attr: &'a mut Table) -> Self {
        Self { attr }
    }
}

impl<'a> From<&'a Table> for GroupFileConfiguration<'a> {
    fn from(attr: &'a Table) -> Self {
        Self { attr }
    }
}
