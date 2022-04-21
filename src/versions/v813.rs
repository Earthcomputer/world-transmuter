use rust_dataconverter_engine::{data_converter_func, MapType, ObjectType, Types};
use crate::MinecraftTypesMut;

const VERSION: u32 = 813;

const SHULKER_ID_BY_COLOR: [&str; 16] = [
    "minecraft:white_shulker_box",
    "minecraft:orange_shulker_box",
    "minecraft:magenta_shulker_box",
    "minecraft:light_blue_shulker_box",
    "minecraft:yellow_shulker_box",
    "minecraft:lime_shulker_box",
    "minecraft:pink_shulker_box",
    "minecraft:gray_shulker_box",
    "minecraft:silver_shulker_box",
    "minecraft:cyan_shulker_box",
    "minecraft:purple_shulker_box",
    "minecraft:blue_shulker_box",
    "minecraft:brown_shulker_box",
    "minecraft:green_shulker_box",
    "minecraft:red_shulker_box",
    "minecraft:black_shulker_box",
];

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    types.item_stack.borrow_mut().add_converter_for_id("minecraft:shulker_box", VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        let _: Option<_> = try {
            let tag = data.get_map_mut("tag")?;
            let block_entity = tag.get_map_mut("BlockEntityTag")?;

            let color = block_entity.remove("Color")?.as_i64()?;
            data.set("id", T::Object::create_string(SHULKER_ID_BY_COLOR[color.rem_euclid(SHULKER_ID_BY_COLOR.len() as i64) as usize].to_owned()));
        };
    }));

    types.tile_entity.borrow_mut().add_converter_for_id("minecraft:shulker_box", VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        data.remove("Color");
    }));
}
