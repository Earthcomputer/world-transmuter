use crate::MinecraftTypesMut;
use rust_dataconverter_engine::map_data_converter_func;
use valence_nbt::Value;

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

pub(crate) fn register(types: &MinecraftTypesMut) {
    types.item_stack.borrow_mut().add_converter_for_id(
        "minecraft:shulker_box",
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            let Some(Value::Compound(tag)) = data.get_mut("tag") else {
                return;
            };
            let Some(Value::Compound(block_entity)) = tag.get_mut("BlockEntityTag") else {
                return;
            };

            let Some(color) = block_entity.remove("Color").and_then(|v| v.as_i64()) else {
                return;
            };
            data.insert(
                "id",
                SHULKER_ID_BY_COLOR[color.rem_euclid(SHULKER_ID_BY_COLOR.len() as i64) as usize],
            );
        }),
    );

    types.tile_entity.borrow_mut().add_converter_for_id(
        "minecraft:shulker_box",
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            data.remove("Color");
        }),
    );
}
