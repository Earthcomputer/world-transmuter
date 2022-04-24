use std::collections::BTreeMap;
use rust_dataconverter_engine::{MapType, ObjectType, Types};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub(crate) struct BlockStateOwned {
    pub(crate) name: String,
    pub(crate) properties: BTreeMap<String, String>,
}

impl BlockStateOwned {
    pub(crate) fn to_nbt<T: Types + ?Sized>(&self) -> T::Map {
        let mut nbt = T::Map::create_empty();
        nbt.set("Name", T::Object::create_string(self.name.clone()));
        if !self.properties.is_empty() {
            let mut props = T::Map::create_empty();
            for (key, val) in &self.properties {
                props.set(key, T::Object::create_string(val.clone()));
            }
            nbt.set("Properties", T::Object::create_map(props));
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

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
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
            properties: props
        }
    }
}

impl<'a> BlockState<'a> {
    pub(crate) fn from_nbt<T: Types + ?Sized>(nbt: &'a T::Map) -> Option<Self> {
        let name = nbt.get_string("Name")?;
        let mut properties = BTreeMap::new();
        if let Some(props) = nbt.get_map("Properties") {
            for key in props.keys() {
                let value = props.get_string(key)?;
                properties.insert(key.as_str(), value);
            }
        }

        Some(Self { name, properties })
    }

    pub(crate) fn to_nbt<T: Types + ?Sized>(&self) -> T::Map {
        let mut nbt = T::Map::create_empty();
        nbt.set("Name", T::Object::create_string(self.name.to_owned()));
        if !self.properties.is_empty() {
            let mut props = T::Map::create_empty();
            for (key, val) in &self.properties {
                props.set(*key, T::Object::create_string(val.to_string()));
            }
            nbt.set("Properties", T::Object::create_map(props));
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
            properties: props
        }
    }
}

#[macro_export]
macro_rules! block_state {
    ($name:literal $([$($prop_name:literal = $prop_value:literal),+])?) => {
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