use crate::helpers::rename::{rename_advancement, simple_rename};
use crate::{static_string_mc_set, types};
use java_string::JavaString;
use world_transmuter_engine::{map_data_converter_func, JValue};

const VERSION: u32 = 2503;

static_string_mc_set! {
    wall_blocks = {
        "andesite_wall",
        "brick_wall",
        "cobblestone_wall",
        "diorite_wall",
        "end_stone_brick_wall",
        "granite_wall",
        "mossy_cobblestone_wall",
        "mossy_stone_brick_wall",
        "nether_brick_wall",
        "prismarine_wall",
        "red_nether_brick_wall",
        "red_sandstone_wall",
        "sandstone_wall",
        "stone_brick_wall",
    }
}

pub(crate) fn register() {
    types::block_state_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            let Some(JValue::String(name)) = data.get("Name") else {
                return;
            };
            if wall_blocks().contains(name) {
                if let Some(JValue::Compound(properties)) = data.get_mut("Properties") {
                    for side in ["east", "west", "north", "south"] {
                        if let Some(JValue::String(value)) = properties.get_mut(side) {
                            let new_value = if value == "true" { "low" } else { "none" };
                            *value = JavaString::from(new_value);
                        }
                    }
                }
            }
        }),
    );

    rename_advancement(
        VERSION,
        simple_rename(
            "minecraft:recipes/misc/composter",
            "minecraft:recipes/decorations/composter",
        ),
    );
}
