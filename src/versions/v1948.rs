use rust_dataconverter_engine::{data_converter_func, MapType, ObjectType, Types};
use crate::helpers::json_parser;
use crate::MinecraftTypesMut;

const VERSION: u32 = 1948;

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    types.item_stack.borrow_mut().add_converter_for_id("minecraft:white_banner", VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        let _: Option<_> = try {
            let display = data.get_map_mut("tag")?.get_map_mut("display")?;
            let prev_name = display.get_string("Name")?;
            if let Ok(mut json) = json_parser::parse_map::<T>(prev_name) {
                if json.get_string("translate") == Some("block.minecraft.illager_banner") {
                    json.set("translate", T::Object::create_string("block.minecraft.ominous_banner".to_owned()));
                    display.set("Name", T::Object::create_string(json_parser::stringify_map::<T>(json)));
                }
            }
        };
    }));
}
