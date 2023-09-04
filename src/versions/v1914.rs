use crate::MinecraftTypesMut;
use valence_nbt::Value;
use world_transmuter_engine::map_data_converter_func;

const VERSION: u32 = 1914;

pub(crate) fn register(types: MinecraftTypesMut) {
    types.tile_entity().borrow_mut().add_converter_for_id(
        "minecraft:chest",
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            if let Some(Value::String(loot_table)) = data.get_mut("LootTable") {
                if loot_table == "minecraft:chests/village_blacksmith" {
                    *loot_table = "minecraft:chests/village/village_weaponsmith".to_owned();
                }
            }
        }),
    );
}
