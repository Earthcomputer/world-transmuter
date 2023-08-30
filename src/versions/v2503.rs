use crate::helpers::mc_namespace_map::McNamespaceSet;
use crate::helpers::rename::{rename_advancement, simple_rename};
use crate::MinecraftTypes;
use rust_dataconverter_engine::map_data_converter_func;
use std::sync::OnceLock;
use valence_nbt::Value;

const VERSION: u32 = 2503;

static WALL_BLOCKS: OnceLock<McNamespaceSet> = OnceLock::new();

fn wall_blocks() -> &'static McNamespaceSet<'static> {
    WALL_BLOCKS.get_or_init(|| {
        let mut map = McNamespaceSet::new();
        map.insert_mc("andesite_wall");
        map.insert_mc("brick_wall");
        map.insert_mc("cobblestone_wall");
        map.insert_mc("diorite_wall");
        map.insert_mc("end_stone_brick_wall");
        map.insert_mc("granite_wall");
        map.insert_mc("mossy_cobblestone_wall");
        map.insert_mc("mossy_stone_brick_wall");
        map.insert_mc("nether_brick_wall");
        map.insert_mc("prismarine_wall");
        map.insert_mc("red_nether_brick_wall");
        map.insert_mc("red_sandstone_wall");
        map.insert_mc("sandstone_wall");
        map.insert_mc("stone_brick_wall");
        map
    })
}

pub(crate) fn register<'a>(types: &'a MinecraftTypes<'a>) {
    types.block_state.borrow_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            let Some(Value::String(name)) = data.get("Name") else {
                return;
            };
            if wall_blocks().contains(name) {
                if let Some(Value::Compound(properties)) = data.get_mut("Properties") {
                    for side in ["east", "west", "north", "south"] {
                        if let Some(Value::String(value)) = properties.get_mut(side) {
                            let new_value = if value == "true" { "low" } else { "none" };
                            *value = new_value.to_owned();
                        }
                    }
                }
            }
        }),
    );

    rename_advancement(
        types,
        VERSION,
        simple_rename(
            "minecraft:recipes/misc/composter",
            "minecraft:recipes/decorations/composter",
        ),
    );
}
