use std::marker::PhantomData;
use rust_dataconverter_engine::{DataHook, DataVersion, MapType, ObjectType, Types};
use crate::helpers::resource_location::ResourceLocation;

pub(crate) struct DataHookValueTypeEnforceNamespaced<T: Types + ?Sized> {
    _phantom: PhantomData<T>,
}

impl<T: Types + ?Sized> DataHookValueTypeEnforceNamespaced<T> {
    pub(crate) fn new() -> Self {
        Self { _phantom: PhantomData }
    }
}

impl<T: Types + ?Sized> DataHook<T::Object> for DataHookValueTypeEnforceNamespaced<T> {
    fn pre_hook(&self, data: &mut T::Object, _from_version: DataVersion, _to_version: DataVersion) {
        if let Some(str) = data.as_string() {
            if let Ok(replacement) = str.parse::<ResourceLocation>() {
                *data = T::Object::create_string(replacement.to_string());
            }
        }
    }

    fn post_hook(&self, _data: &mut T::Object, _from_version: DataVersion, _to_version: DataVersion) {
    }
}

pub(crate) struct DataHookEnforceNamespacedId<T: Types + ?Sized> {
    _phantom: PhantomData<T>,
    id: String,
}

impl<T: Types + ?Sized> DataHookEnforceNamespacedId<T> {
    pub(crate) fn id() -> Self {
        Self::new("id")
    }

    pub(crate) fn new(id: impl Into<String>) -> Self {
        Self { _phantom: PhantomData, id: id.into() }
    }
}

impl<T: Types + ?Sized> DataHook<T::Map> for DataHookEnforceNamespacedId<T> {
    fn pre_hook(&self, data: &mut T::Map, _from_version: DataVersion, _to_version: DataVersion) {
        if let Some(str) = data.get_string(&self.id) {
            if let Ok(replacement) = str.parse::<ResourceLocation>() {
                data.set(self.id.clone(), T::Object::create_string(replacement.to_string()));
            }
        }
    }

    fn post_hook(&self, _data: &mut T::Map, _from_version: DataVersion, _to_version: DataVersion) {
    }
}
