use crate::helpers::resource_location::ResourceLocation;
use java_string::JavaString;
use world_transmuter_engine::{
    DataVersion, JCompound, JValue, JValueMut, MapDataHook, ValueDataHook,
};

pub(crate) struct DataHookValueTypeEnforceNamespaced;

impl ValueDataHook for DataHookValueTypeEnforceNamespaced {
    fn pre_hook(&self, data: &mut JValueMut, _from_version: DataVersion, _to_version: DataVersion) {
        if let JValueMut::String(str) = data {
            if let Ok(replacement) = ResourceLocation::parse(str) {
                **str = replacement.to_java_string();
            }
        }
    }

    fn post_hook(
        &self,
        _data: &mut JValueMut,
        _from_version: DataVersion,
        _to_version: DataVersion,
    ) {
    }
}

pub(crate) struct DataHookEnforceNamespacedId {
    id: JavaString,
}

impl DataHookEnforceNamespacedId {
    pub(crate) fn id() -> Self {
        Self::new("id")
    }

    pub(crate) fn new(id: impl Into<JavaString>) -> Self {
        Self { id: id.into() }
    }
}

impl MapDataHook for DataHookEnforceNamespacedId {
    fn pre_hook(&self, data: &mut JCompound, _from_version: DataVersion, _to_version: DataVersion) {
        if let Some(JValue::String(str)) = data.get(&self.id[..]) {
            if let Ok(replacement) = ResourceLocation::parse(str) {
                data.insert(self.id.clone(), replacement.to_java_string());
            }
        }
    }

    fn post_hook(
        &self,
        _data: &mut JCompound,
        _from_version: DataVersion,
        _to_version: DataVersion,
    ) {
    }
}
