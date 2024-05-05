use crate::helpers::components::retrieve_translation_string;
use crate::{static_string_set, types};
use java_string::JavaStr;
use std::collections::BTreeSet;
use world_transmuter_engine::{
    convert_map_in_map, get_mut_multi, map_data_converter_func, map_data_walker, rename_key,
    DataVersion, DataWalkerMapTypePaths, JCompound, JList, JValue,
};

const VERSION: u32 = 3825;

static_string_set!(
    BANNER_NAMES, banner_names, {
        "block.minecraft.ominous_banner",
    }
);

static_string_set!(
    MAP_NAMES, map_names, {
        "filled_map.buried_treasure",
        "filled_map.explorer_jungle",
        "filled_map.explorer_swamp",
        "filled_map.mansion",
        "filled_map.monument",
        "filled_map.trial_chambers",
        "filled_map.village_desert",
        "filled_map.village_plains",
        "filled_map.village_savanna",
        "filled_map.village_snowy",
        "filled_map.village_taiga",
    }
);

const TRIAL_SPAWNER_NORMAL_CONFIG_KEYS: [&JavaStr; 9] = [
    JavaStr::from_str("spawn_range"),
    JavaStr::from_str("total_mobs"),
    JavaStr::from_str("simultaneous_mobs"),
    JavaStr::from_str("total_mobs_added_per_player"),
    JavaStr::from_str("simultaneous_mobs_added_per_player"),
    JavaStr::from_str("ticks_between_spawn"),
    JavaStr::from_str("spawn_potentials"),
    JavaStr::from_str("loot_tables_to_eject"),
    JavaStr::from_str("items_to_drop_when_ominous"),
];

pub(crate) fn register() {
    types::item_stack_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            let [Some(JValue::Compound(components)), Some(JValue::String(id))] =
                get_mut_multi(data, ["components", "id"])
            else {
                return;
            };
            match id.as_bytes() {
                b"minecraft:white_banner" => convert_name(components, banner_names()),
                b"minecraft:filled_map" => convert_name(components, map_names()),
                _ => {}
            }
        }),
    );
    types::tile_entity_mut().add_converter_for_id(
        "minecraft:banner",
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            let Some(JValue::String(custom_name)) = data.get("CustomName") else {
                return;
            };
            if !retrieve_translation_string(custom_name)
                .is_some_and(|translation_key| translation_key == "block.minecraft.ominous_banner")
            {
                return;
            }
            let custom_name = data.remove("CustomName").unwrap();

            let JValue::Compound(components) = data
                .entry("components")
                .and_modify(|comp| {
                    if !matches!(comp, JValue::Compound(_)) {
                        *comp = JValue::Compound(JCompound::new())
                    }
                })
                .or_insert_with(JCompound::new)
            else {
                unreachable!()
            };
            components.insert("minecraft:item_name", custom_name);
            components.insert("minecraft:hide_additional_tooltip", JCompound::new());
        }),
    );
    types::tile_entity_mut().add_walker_for_id(
        VERSION,
        "minecraft:trial_spawner",
        map_data_walker(|data, from_version, to_version| {
            fn convert_config(
                config: &mut JCompound,
                from_version: DataVersion,
                to_version: DataVersion,
            ) {
                if let Some(JValue::List(JList::Compound(spawn_potentials))) =
                    config.get_mut("spawn_potentials")
                {
                    for spawn_potential in spawn_potentials {
                        if let Some(JValue::Compound(data)) = spawn_potential.get_mut("data") {
                            convert_map_in_map(
                                types::entity_ref(),
                                data,
                                "entity",
                                from_version,
                                to_version,
                            );
                        }
                    }
                }
            }

            if let Some(JValue::Compound(normal_config)) = data.get_mut("normal_config") {
                convert_config(normal_config, from_version, to_version);
            }
            if let Some(JValue::Compound(ominous_config)) = data.get_mut("ominous_config") {
                convert_config(ominous_config, from_version, to_version);
            }

            if let Some(JValue::Compound(spawn_data)) = data.get_mut("spawn_data") {
                convert_map_in_map(
                    types::entity_ref(),
                    spawn_data,
                    "entity",
                    from_version,
                    to_version,
                );
            }
        }),
    );
    types::tile_entity_mut().add_converter_for_id(
        "minecraft:trial_spawner",
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            let mut normal_config = JCompound::new();
            for normal_key in TRIAL_SPAWNER_NORMAL_CONFIG_KEYS {
                if let Some(normal_value) = data.remove(normal_key) {
                    normal_config.insert(normal_key, normal_value);
                }
            }
            if !normal_config.is_empty() {
                data.insert("normal_config", normal_config);
            }
        }),
    );

    types::entity_mut().add_walker_for_id(
        VERSION,
        "minecraft:ominous_item_spawner",
        DataWalkerMapTypePaths::new(types::item_stack_ref(), "item"),
    );
}

fn convert_name(components: &mut JCompound, standard_names: &BTreeSet<&'static JavaStr>) {
    let Some(JValue::String(custom_name)) = components.get("minecraft:custom_name") else {
        return;
    };
    let Some(translation) = retrieve_translation_string(custom_name) else {
        return;
    };

    if standard_names.contains(&translation[..]) {
        rename_key(components, "minecraft:custom_name", "minecraft:item_name");
    }
}
