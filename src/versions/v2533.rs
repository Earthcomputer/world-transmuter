use rust_dataconverter_engine::{data_converter_func, ListType, MapType, ObjectType, Types};
use crate::MinecraftTypesMut;

const VERSION: u32 = 2533;

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    types.entity.borrow_mut().add_converter_for_id("minecraft:villager", VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        if let Some(attributes) = data.get_list_mut("Attributes") {
            for attribute in attributes.iter_mut() {
                if let Some(attribute) = attribute.as_map_mut() {
                    if attribute.get_string("Name") == Some("generic.follow_range") && attribute.get_f64("Base") == Some(16.0) {
                        attribute.set("Base", T::Object::create_double(48.0));
                    }
                }
            }
        }
    }));
}
