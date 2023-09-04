use crate::helpers::mc_namespace_map::McNamespaceMap;
use crate::helpers::resource_location::ResourceLocation;
use crate::types::MinecraftTypesMut;
use std::sync::OnceLock;
use valence_nbt::value::ValueMut;
use world_transmuter_engine::value_data_converter_func;

const VERSION: u32 = 3084;

static GAME_EVENT_RENAMES: OnceLock<McNamespaceMap<&str>> = OnceLock::new();

fn game_event_renames() -> &'static McNamespaceMap<'static, &'static str> {
    GAME_EVENT_RENAMES.get_or_init(|| {
        let mut map = McNamespaceMap::new();
        map.insert_mc("block_press", "minecraft:block_activate");
        map.insert_mc("block_switch", "minecraft:block_activate");
        map.insert_mc("block_unpress", "minecraft:block_deactivate");
        map.insert_mc("block_unswitch", "minecraft:block_deactivate");
        map.insert_mc("drinking_finish", "minecraft:drink");
        map.insert_mc("elytra_free_fall", "minecraft:elytra_glide");
        map.insert_mc("entity_damaged", "minecraft:entity_damage");
        map.insert_mc("entity_dying", "minecraft:entity_die");
        map.insert_mc("entity_killed", "minecraft:entity_die");
        map.insert_mc("mob_interact", "minecraft:entity_interact");
        map.insert_mc("ravager_roar", "minecraft:entity_roar");
        map.insert_mc("ring_bell", "minecraft:block_change");
        map.insert_mc("shulker_close", "minecraft:container_close");
        map.insert_mc("shulker_open", "minecraft:container_open");
        map.insert_mc("wolf_shaking", "minecraft:entity_shake");
        map
    })
}

pub(crate) fn register(types: MinecraftTypesMut) {
    types
        .game_event_name()
        .borrow_mut()
        .add_structure_converter(
            VERSION,
            value_data_converter_func(|data, _from_version, _to_version| {
                if let ValueMut::String(data) = data {
                    let corrected_data = data
                        .parse::<ResourceLocation>()
                        .map_or_else(|_| (*data).clone(), |rl| rl.to_string());
                    if let Some(new_name) = game_event_renames().get(&corrected_data[..]).copied() {
                        **data = new_name.to_owned();
                    }
                }
            }),
        );
}
