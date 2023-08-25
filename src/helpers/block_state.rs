use std::collections::btree_map::BTreeMap;
use valence_nbt::{Compound, Value};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub(crate) struct BlockStateOwned {
    pub(crate) name: String,
    pub(crate) properties: BTreeMap<String, String>,
}

impl BlockStateOwned {
    pub(crate) fn to_nbt(&self) -> Compound {
        let mut nbt = Compound::new();
        nbt.insert("Name", self.name.clone());
        if !self.properties.is_empty() {
            let mut props = Compound::new();
            for (key, val) in &self.properties {
                props.insert(key, val);
            }
            nbt.insert("Properties", props);
        }
        nbt
    }
}

#[macro_export]
macro_rules! block_state_owned {
    ($name:expr; $([$($prop_name:expr => $prop_value:expr),+])?) => {
        BlockStateOwned {
            name: $name.into(),
            properties: {
                #[allow(unused_mut)]
                let mut map = std::collections::BTreeMap::new();
                $(
                    $(
                        map.insert($prop_name.into(), $prop_value.into());
                    )+
                )?
                map
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub(crate) struct BlockState<'a> {
    pub(crate) name: &'a str,
    pub(crate) properties: BTreeMap<&'a str, &'a str>,
}

impl<'a> From<&'a BlockStateOwned> for BlockState<'a> {
    fn from(value: &'a BlockStateOwned) -> BlockState<'a> {
        let mut props = BTreeMap::new();
        for (key, val) in value.properties.iter() {
            props.insert(key.as_str(), val.as_str());
        }
        BlockState {
            name: value.name.as_str(),
            properties: props,
        }
    }
}

impl<'a> BlockState<'a> {
    pub(crate) fn from_nbt(nbt: &'a Compound) -> Option<Self> {
        let Some(Value::String(name)) = nbt.get("Name") else {
            return None;
        };
        let mut properties = BTreeMap::new();
        if let Some(Value::Compound(props)) = nbt.get("Properties") {
            for (key, value) in props {
                let Value::String(value) = value else {
                    return None;
                };
                properties.insert(key.as_str(), value.as_str());
            }
        }

        Some(Self { name, properties })
    }

    pub(crate) fn to_nbt(&self) -> Compound {
        let mut nbt = Compound::new();
        nbt.insert("Name", self.name);
        if !self.properties.is_empty() {
            let mut props = Compound::new();
            for (key, val) in &self.properties {
                props.insert(*key, *val);
            }
            nbt.insert("Properties", props);
        }
        nbt
    }

    pub(crate) fn to_owned(&self) -> BlockStateOwned {
        let mut props = BTreeMap::new();
        for (key, val) in &self.properties {
            props.insert(key.to_string(), val.to_string());
        }
        BlockStateOwned {
            name: self.name.to_string(),
            properties: props,
        }
    }
}

#[macro_export]
macro_rules! block_state {
    ($name:literal $([$($prop_name:literal = $prop_value:expr),+])?) => {
        BlockState {
            name: $name,
            properties: {
                #[allow(unused_mut)]
                let mut map = std::collections::BTreeMap::new();
                $(
                    $(
                        map.insert($prop_name, $prop_value);
                    )+
                )?
                map
            }
        }
    }
}
