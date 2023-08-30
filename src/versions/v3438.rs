use crate::helpers::rename::{rename_item, rename_tile_entity};
use crate::types::MinecraftTypes;
use rust_dataconverter_engine::{map_data_converter_func, rename_key};

const VERSION: u32 = 3438;

pub(crate) fn register<'a>(types: &'a MinecraftTypes<'a>) {
    // brushable block rename
    types.tile_entity.borrow_mut().copy_walkers(
        VERSION,
        "minecraft:suspicious_sand",
        "minecraft:brushable_block",
    );

    rename_tile_entity(types, VERSION, |name| {
        if name == "minecraft:suspicious_sand" {
            Some("minecraft:brushable_block".to_owned())
        } else {
            None
        }
    });

    types.tile_entity.borrow_mut().add_converter_for_id(
        "minecraft:brushable_block",
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            rename_key(data, "loot_table", "LootTable");
            rename_key(data, "loot_table_seed", "LootTableSeed");
        }),
    );

    rename_item(types, VERSION, |name| match name {
        "minecraft:pottery_shard_archer" => Some("minecraft:archer_pottery_shard".to_owned()),
        "minecraft:pottery_shard_prize" => Some("minecraft:prize_pottery_shard".to_owned()),
        "minecraft:pottery_shard_arms_up" => Some("minecraft:arms_up_pottery_shard".to_owned()),
        "minecraft:pottery_shard_skull" => Some("minecraft:skull_pottery_shard".to_owned()),
        _ => None,
    });
}
