use toml_edit::{table, value, Document, Value};

pub struct GroupConfiguration<'a>(&'a Document);

impl<'a> From<&'a Document> for GroupConfiguration<'a> {
    fn from(doc: &'a Document) -> Self {
        Self(doc)
    }
}

pub struct GroupConfigurationMut<'a>(&'a mut Document);

impl<'a> From<&'a mut Document> for GroupConfigurationMut<'a> {
    fn from(doc: &'a mut Document) -> Self {
        Self(doc)
    }
}

impl<'a> Into<&'a Document> for GroupConfiguration<'a> {
    fn into(self) -> &'a Document {
        self.0
    }
}
pub trait GroupTomlDocument {
    fn doc_deref<'a>(&'a self) -> &'a Document;
}
pub trait GroupTomlDocumentMut: GroupTomlDocument {
    fn doc_deref_mut<'a>(&'a mut self) -> &'a mut Document;
}

impl GroupTomlDocument for GroupConfiguration<'_> {
    fn doc_deref<'a>(&'a self) -> &'a Document {
        self.0
    }
}

impl GroupTomlDocument for GroupConfigurationMut<'_> {
    fn doc_deref<'a>(&'a self) -> &'a Document {
        self.0
    }
}

impl GroupTomlDocumentMut for GroupConfigurationMut<'_> {
    fn doc_deref_mut<'a>(&'a mut self) -> &'a mut Document {
        self.0
    }
}

pub trait GroupConfigurationReader {
    fn group_exists(&self, name: &str) -> bool;
}
pub trait GroupConfigurationWriter: GroupConfigurationReader {
    fn group_add(&mut self, name: &str);
    fn group_setfield<T>(&mut self, name: &str, fieldname: &str, v: T)
    where
        T: Into<Value>;
}

impl<T> GroupConfigurationReader for T
where
    T: GroupTomlDocument,
{
    fn group_exists(&self, name: &str) -> bool {
        self.doc_deref()
            .contains_table(name)
    }
}

impl<T> GroupConfigurationWriter for T
where
    T: GroupTomlDocumentMut,
{
    fn group_add(&mut self, name: &str) {
        let r = self.doc_deref_mut();
        if ! r.contains_table(name){
            r[name] = table();
        }
        r[name]["enable"] = value(true);
    }

    fn group_setfield<V>(&mut self, name: &str, fieldname: &str, v: V)
    where
        V: Into<Value>,
    {
        let r = self.doc_deref_mut();
        r[name][fieldname] = value(v);
    }
}
