use rust_dataconverter_engine::{data_converter_func, MapType, ObjectType, Types};
use crate::helpers::json_parser;
use crate::MinecraftTypesMut;

const VERSION: u32 = 1953;

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    types.tile_entity.borrow_mut().add_converter_for_id("minecraft:banner", VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        if let Some(name) = data.get_string("CustomName") {
            if let Ok(mut json) = json_parser::parse_map::<T>(name) {
                if json.get_string("translate") == Some("block.minecraft.illager_banner") {
                    json.set("translate", T::Object::create_string("block.minecraft.ominous_banner".to_owned()));
                    data.set("CustomName", T::Object::create_string(json_parser::stringify_map::<T>(json)))
                }
            }
        }
    }));
}
