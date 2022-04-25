use rust_dataconverter_engine::{data_converter_func, DataType, DataVersion, MapType, ObjectType, Types};
use crate::MinecraftTypesMut;

pub(crate) fn rename_entity<'a, T: Types + ?Sized>(
    types: &MinecraftTypesMut<'a, T>,
    version: impl Into<DataVersion>,
    renamer: impl 'a + Copy + Fn(&str) -> Option<String>
) {
    let version = version.into();
    types.entity.borrow_mut().add_structure_converter(version, data_converter_func::<T::Map, _>(move |data, _from_version, _to_version| {
        if let Some(new_id) = data.get_string("id").and_then(renamer) {
            data.set("id", T::Object::create_string(new_id));
        }
    }));
    types.entity_name.borrow_mut().add_structure_converter(version, data_converter_func::<T::Object, _>(move |data, _from_version, _to_version| {
        if let Some(new_id) = data.as_string().and_then(renamer) {
            *data = T::Object::create_string(new_id);
        }
    }));
}

pub(crate) fn rename_block<'a, T: Types + ?Sized>(
    types: &MinecraftTypesMut<'a, T>,
    version: impl Into<DataVersion>,
    renamer: impl 'a + Copy + Fn(&str) -> Option<String>
) {
    let version = version.into();
    types.block_state.borrow_mut().add_structure_converter(version, data_converter_func::<T::Map, _>(move |data, _from_version, _to_version| {
        if let Some(new_name) = data.get_string("Name").and_then(renamer) {
            data.set("Name", T::Object::create_string(new_name));
        }
    }));
    types.block_name.borrow_mut().add_structure_converter(version, data_converter_func::<T::Object, _>(move |data, _from_verison, _to_version| {
        if let Some(new_id) = data.as_string().and_then(renamer) {
            *data = T::Object::create_string(new_id);
        }
    }));
}

pub(crate) fn rename_item<'a, T: Types + ?Sized>(
    types: &MinecraftTypesMut<'a, T>,
    version: impl Into<DataVersion>,
    renamer: impl 'a + Copy + Fn(&str) -> Option<String>
) {
    types.item_name.borrow_mut().add_structure_converter(version, data_converter_func::<T::Object, _>(move |data, _from_version, _to_version| {
        if let Some(new_name) = data.as_string().and_then(renamer) {
            *data = T::Object::create_string(new_name);
        }
    }));
}

pub(crate) fn rename_keys_in_map<T: Types + ?Sized>(typ: impl DataType<T::Object>, owning_map: &mut T::Map, key: &str, from_version: DataVersion, to_version: DataVersion) {
    if let Some(map) = owning_map.get_map_mut(key) {
        rename_keys::<T>(typ, map, from_version, to_version);
    }
}

pub(crate) fn rename_keys<T: Types + ?Sized>(typ: impl DataType<T::Object>, map: &mut T::Map, from_version: DataVersion, to_version: DataVersion) {
    map.rename_keys(move |key| {
        let mut new_key = T::Object::create_string(key.to_owned());
        typ.convert(&mut new_key, from_version, to_version);
        if let Some(new_key) = new_key.into_string() {
            if new_key != key {
                return Some(new_key);
            }
        }
        None
    });
}
