use java_string::{JavaStr, JavaString};
use std::collections::btree_map::BTreeMap;
use world_transmuter_engine::{JCompound, JValue};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub(crate) struct BlockStateOwned {
    pub(crate) name: JavaString,
    pub(crate) properties: BTreeMap<JavaString, JavaString>,
}

impl BlockStateOwned {
    pub(crate) fn to_nbt(&self) -> JCompound {
        let mut nbt = JCompound::new();
        nbt.insert("Name", self.name.clone());
        if !self.properties.is_empty() {
            let mut props = JCompound::new();
            for (key, val) in &self.properties {
                props.insert(key, val);
            }
            nbt.insert("Properties", props);
        }
        nbt
    }

    pub(crate) fn get_property(&self, key: &(impl AsRef<JavaStr> + ?Sized)) -> Option<&JavaStr> {
        self.properties.get(key.as_ref()).map(|value| &value[..])
    }
}

#[macro_export]
macro_rules! block_state_owned {
    ($name:expr; $([$($prop_name:expr => $prop_value:expr),+])?) => {
        $crate::helpers::block_state::BlockStateOwned {
            name: $name.into(),
            properties: {
                #[allow(unused_mut)]
                let mut map = ::std::collections::BTreeMap::new();
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
    pub(crate) name: &'a JavaStr,
    pub(crate) properties: BTreeMap<&'a JavaStr, &'a JavaStr>,
}

impl<'a> From<&'a BlockStateOwned> for BlockState<'a> {
    fn from(value: &'a BlockStateOwned) -> BlockState<'a> {
        let mut props = BTreeMap::new();
        for (key, val) in value.properties.iter() {
            props.insert(&key[..], &val[..]);
        }
        BlockState {
            name: &value.name[..],
            properties: props,
        }
    }
}

impl<'a> BlockState<'a> {
    pub(crate) fn get_property(&self, key: &(impl AsRef<JavaStr> + ?Sized)) -> Option<&'a JavaStr> {
        self.properties.get(key.as_ref()).copied()
    }

    pub(crate) fn set_property(
        &mut self,
        key: &'a (impl AsRef<JavaStr> + ?Sized),
        value: &'a (impl AsRef<JavaStr> + ?Sized),
    ) -> Option<&'a JavaStr> {
        self.properties.insert(key.as_ref(), value.as_ref())
    }

    pub(crate) fn from_nbt(nbt: &'a JCompound) -> Option<Self> {
        let Some(JValue::String(name)) = nbt.get("Name") else {
            return None;
        };
        let mut properties = BTreeMap::new();
        if let Some(JValue::Compound(props)) = nbt.get("Properties") {
            for (key, value) in props {
                let JValue::String(value) = value else {
                    return None;
                };
                properties.insert(&key[..], &value[..]);
            }
        }

        Some(Self { name, properties })
    }

    pub(crate) fn to_nbt(&self) -> JCompound {
        let mut nbt = JCompound::new();
        nbt.insert("Name", self.name);
        if !self.properties.is_empty() {
            let mut props = JCompound::new();
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
            props.insert((*key).to_owned(), (*val).to_owned());
        }
        BlockStateOwned {
            name: self.name.to_owned(),
            properties: props,
        }
    }
}

#[macro_export]
macro_rules! block_state {
    ($name:literal $([$($prop_name:literal = $prop_value:expr),+])?) => {
        $crate::helpers::block_state::BlockState {
            name: $name.as_ref(),
            properties: {
                #[allow(unused_mut)]
                let mut map = ::std::collections::BTreeMap::new();
                $(
                    $(
                        map.insert(::java_string::JavaStr::from_str($prop_name), ::java_string::JavaStr::from_str($prop_value));
                    )+
                )?
                map
            }
        }
    }
}
