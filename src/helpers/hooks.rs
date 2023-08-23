use crate::helpers::resource_location::ResourceLocation;
use rust_dataconverter_engine::{DataVersion, MapDataHook, ValueDataHook};
use valence_nbt::value::ValueMut;
use valence_nbt::{Compound, Value};

pub(crate) struct DataHookValueTypeEnforceNamespaced;

impl ValueDataHook for DataHookValueTypeEnforceNamespaced {
    fn pre_hook(&self, data: &mut ValueMut, _from_version: DataVersion, _to_version: DataVersion) {
        if let ValueMut::String(str) = data {
            if let Ok(replacement) = str.parse::<ResourceLocation>() {
                **str = replacement.to_string();
            }
        }
    }

    fn post_hook(
        &self,
        _data: &mut ValueMut,
        _from_version: DataVersion,
        _to_version: DataVersion,
    ) {
    }
}

pub(crate) struct DataHookEnforceNamespacedId {
    id: String,
}

impl DataHookEnforceNamespacedId {
    pub(crate) fn id() -> Self {
        Self::new("id")
    }

    pub(crate) fn new(id: impl Into<String>) -> Self {
        Self { id: id.into() }
    }
}

impl MapDataHook for DataHookEnforceNamespacedId {
    fn pre_hook(&self, data: &mut Compound, _from_version: DataVersion, _to_version: DataVersion) {
        if let Some(Value::String(str)) = data.get(&self.id) {
            if let Ok(replacement) = str.parse::<ResourceLocation>() {
                data.insert(self.id.clone(), replacement.to_string());
            }
        }
    }

    fn post_hook(
        &self,
        _data: &mut Compound,
        _from_version: DataVersion,
        _to_version: DataVersion,
    ) {
    }
}
