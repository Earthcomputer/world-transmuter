use crate::helpers::resource_location::ResourceLocation;
use crate::{static_string_mc_map, types};
use world_transmuter_engine::{value_data_converter_func, JValueMut};

const VERSION: u32 = 3084;

static_string_mc_map! {
    game_event_renames = {
        "block_press" => "minecraft:block_activate",
        "block_switch" => "minecraft:block_activate",
        "block_unpress" => "minecraft:block_deactivate",
        "block_unswitch" => "minecraft:block_deactivate",
        "drinking_finish" => "minecraft:drink",
        "elytra_free_fall" => "minecraft:elytra_glide",
        "entity_damaged" => "minecraft:entity_damage",
        "entity_dying" => "minecraft:entity_die",
        "entity_killed" => "minecraft:entity_die",
        "mob_interact" => "minecraft:entity_interact",
        "ravager_roar" => "minecraft:entity_roar",
        "ring_bell" => "minecraft:block_change",
        "shulker_close" => "minecraft:container_close",
        "shulker_open" => "minecraft:container_open",
        "wolf_shaking" => "minecraft:entity_shake",
    }
}

pub(crate) fn register() {
    types::game_event_name_mut().add_structure_converter(
        VERSION,
        value_data_converter_func(|data, _from_version, _to_version| {
            if let JValueMut::String(data) = data {
                let corrected_data = ResourceLocation::make_correct(data);
                if let Some(new_name) = game_event_renames().get(&corrected_data[..]).copied() {
                    **data = new_name.to_owned();
                }
            }
        }),
    );
}
