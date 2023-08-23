use std::borrow::Borrow;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct McNamespaceMap<'a, V>(BTreeMap<&'a str, V>);

impl<V> Default for McNamespaceMap<'_, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, V> McNamespaceMap<'a, V> {
    pub fn new() -> Self {
        McNamespaceMap(BTreeMap::new())
    }

    pub fn insert_mc(&mut self, key: &'a str, value: V) {
        self.0.insert(key, value);
    }

    pub fn contains_key(&self, key: &str) -> bool {
        key.strip_prefix("minecraft:")
            .map(|key| self.0.contains_key(key))
            == Some(true)
    }

    pub fn get(&self, key: &str) -> Option<&V> {
        key.strip_prefix("minecraft:")
            .and_then(|key| self.get_mc(key))
    }

    pub fn get_mc<Q: ?Sized>(&self, key: &Q) -> Option<&V>
    where
        &'a str: Borrow<Q>,
        Q: Ord,
    {
        self.0.get(key)
    }

    pub fn iter_mc_to_value(&self) -> std::collections::btree_map::Iter<&'a str, V> {
        self.0.iter()
    }
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct McNamespaceSet<'a>(BTreeSet<&'a str>);

impl<'a> McNamespaceSet<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert_mc(&mut self, value: &'a str) {
        self.0.insert(value);
    }

    pub fn contains(&self, value: &str) -> bool {
        value
            .strip_prefix("minecraft:")
            .map(|value| self.0.contains(value))
            == Some(true)
    }
}
