use rust_dataconverter_engine::{data_converter_func, MapType, ObjectType, Types};
use crate::MinecraftTypesMut;

const VERSION: u32 = 2518;

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    types.tile_entity.borrow_mut().add_converter_for_id("minecraft:jigsaw", VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        let _: Option<_> = try {
            let typ = data.remove("attachment_type")?.into_string()?;
            let pool = data.remove("target_pool")?.into_string()?;
            data.set("name", T::Object::create_string(typ.clone()));
            data.set("target", T::Object::create_string(typ));
            data.set("pool", T::Object::create_string(pool));
        };
    }));

    types.block_state.borrow_mut().add_structure_converter(VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        if data.get_string("Name") == Some("minecraft:jigsaw") {
            if let Some(properties) = data.get_map_mut("Properties") {
                let facing = properties.remove("facing").and_then(|o| o.into_string());
                let facing = match facing.as_deref() {
                    Some("down") => "down_south",
                    Some("up") => "up_north",
                    Some("south") => "south_up",
                    Some("west") => "west_up",
                    Some("east") => "east_up",
                    _ => "north_up",
                };
                properties.set("orientation", T::Object::create_string(facing.to_owned()));
            }
        }
    }));
}
