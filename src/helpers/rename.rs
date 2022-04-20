use rust_dataconverter_engine::{data_converter_func, DataVersion, MapType, ObjectType, Types};
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
