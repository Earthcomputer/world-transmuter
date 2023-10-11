use java_string::JavaStr;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct McNamespaceMap<'a, V>(BTreeMap<&'a JavaStr, V>);

impl<V> Default for McNamespaceMap<'_, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, V> McNamespaceMap<'a, V> {
    pub fn new() -> Self {
        McNamespaceMap(BTreeMap::new())
    }

    pub fn insert_mc(&mut self, key: &'a (impl AsRef<JavaStr> + ?Sized), value: V) -> Option<V> {
        self.0.insert(key.as_ref(), value)
    }

    pub fn contains_key(&self, key: &JavaStr) -> bool {
        key.strip_prefix("minecraft:")
            .map(|key| self.0.contains_key(key))
            == Some(true)
    }

    pub fn get(&self, key: &JavaStr) -> Option<&V> {
        key.strip_prefix("minecraft:")
            .and_then(|key| self.get_mc(key))
    }

    pub fn get_mc(&self, key: &JavaStr) -> Option<&V> {
        self.0.get(key)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn iter_mc_to_value(&self) -> std::collections::btree_map::Iter<&'a JavaStr, V> {
        self.0.iter()
    }
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct McNamespaceSet<'a>(BTreeSet<&'a JavaStr>);

impl<'a> McNamespaceSet<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert_mc(&mut self, value: &'a str) {
        self.0.insert(JavaStr::from_str(value));
    }

    pub fn contains(&self, value: &JavaStr) -> bool {
        value
            .strip_prefix("minecraft:")
            .map(|value| self.0.contains(value))
            == Some(true)
    }
}
