use crate::helpers::hooks::DataHookEnforceNamespacedId;
use crate::{static_string_map, static_string_mc_map, types};
use java_string::{JavaStr, JavaString};
use tracing::warn;
use world_transmuter_engine::{
    convert_map_list_in_map, convert_object_in_map, convert_object_list_in_map, data_walker,
    get_mut_multi, map_data_converter_func, AbstractMapDataType, DataWalkerMapListPaths,
    DataWalkerMapTypePaths, DataWalkerObjectTypePaths, JValue,
};

const VERSION: u32 = 704;

static_string_mc_map! {
    ITEM_ID_TO_TILE_ENTITY_ID, item_id_to_tile_entity_id, {
        "furnace" => "minecraft:furnace",
        "lit_furnace" => "minecraft:furnace",
        "chest" => "minecraft:chest",
        "trapped_chest" => "minecraft:chest",
        "ender_chest" => "minecraft:ender_chest",
        "jukebox" => "minecraft:jukebox",
        "dispenser" => "minecraft:dispenser",
        "dropper" => "minecraft:dropper",
        "sign" => "minecraft:sign",
        "mob_spawner" => "minecraft:mob_spawner",
        "spawner" => "minecraft:mob_spawner",
        "noteblock" => "minecraft:noteblock",
        "brewing_stand" => "minecraft:brewing_stand",
        "enhanting_table" => "minecraft:enchanting_table",
        "command_block" => "minecraft:command_block",
        "beacon" => "minecraft:beacon",
        "skull" => "minecraft:skull",
        "daylight_detector" => "minecraft:daylight_detector",
        "hopper" => "minecraft:hopper",
        "banner" => "minecraft:banner",
        "flower_pot" => "minecraft:flower_pot",
        "repeating_command_block" => "minecraft:command_block",
        "chain_command_block" => "minecraft:command_block",
        "shulker_box" => "minecraft:shulker_box",
        "white_shulker_box" => "minecraft:shulker_box",
        "orange_shulker_box" => "minecraft:shulker_box",
        "magenta_shulker_box" => "minecraft:shulker_box",
        "light_blue_shulker_box" => "minecraft:shulker_box",
        "yellow_shulker_box" => "minecraft:shulker_box",
        "lime_shulker_box" => "minecraft:shulker_box",
        "pink_shulker_box" => "minecraft:shulker_box",
        "gray_shulker_box" => "minecraft:shulker_box",
        "silver_shulker_box" => "minecraft:shulker_box",
        "cyan_shulker_box" => "minecraft:shulker_box",
        "purple_shulker_box" => "minecraft:shulker_box",
        "blue_shulker_box" => "minecraft:shulker_box",
        "brown_shulker_box" => "minecraft:shulker_box",
        "green_shulker_box" => "minecraft:shulker_box",
        "red_shulker_box" => "minecraft:shulker_box",
        "black_shulker_box" => "minecraft:shulker_box",
        "bed" => "minecraft:bed",
        "light_gray_shulker_box" => "minecraft:shulker_box",
        "white_banner" => "minecraft:banner",
        "orange_banner" => "minecraft:banner",
        "magenta_banner" => "minecraft:banner",
        "light_blue_banner" => "minecraft:banner",
        "yellow_banner" => "minecraft:banner",
        "lime_banner" => "minecraft:banner",
        "pink_banner" => "minecraft:banner",
        "gray_banner" => "minecraft:banner",
        "silver_banner" => "minecraft:banner",
        "cyan_banner" => "minecraft:banner",
        "purple_banner" => "minecraft:banner",
        "blue_banner" => "minecraft:banner",
        "brown_banner" => "minecraft:banner",
        "green_banner" => "minecraft:banner",
        "red_banner" => "minecraft:banner",
        "black_banner" => "minecraft:banner",
        "standing_sign" => "minecraft:sign",
        "wall_sign" => "minecraft:sign",
        "piston_head" => "minecraft:piston",
        "daylight_detector_inverted" => "minecraft:daylight_detector",
        "unpowered_comparator" => "minecraft:comparator",
        "powered_comparator" => "minecraft:comparator",
        "wall_banner" => "minecraft:banner",
        "standing_banner" => "minecraft:banner",
        "structure_block" => "minecraft:structure_block",
        "end_portal" => "minecraft:end_portal",
        "end_gateway" => "minecraft:end_gateway",
        "shield" => "minecraft:banner",
        "white_bed" => "minecraft:bed",
        "orange_bed" => "minecraft:bed",
        "magenta_bed" => "minecraft:bed",
        "light_blue_bed" => "minecraft:bed",
        "yellow_bed" => "minecraft:bed",
        "lime_bed" => "minecraft:bed",
        "pink_bed" => "minecraft:bed",
        "gray_bed" => "minecraft:bed",
        "silver_bed" => "minecraft:bed",
        "cyan_bed" => "minecraft:bed",
        "purple_bed" => "minecraft:bed",
        "blue_bed" => "minecraft:bed",
        "brown_bed" => "minecraft:bed",
        "green_bed" => "minecraft:bed",
        "red_bed" => "minecraft:bed",
        "black_bed" => "minecraft:bed",
        "oak_sign" => "minecraft:sign",
        "spruce_sign" => "minecraft:sign",
        "birch_sign" => "minecraft:sign",
        "jungle_sign" => "minecraft:sign",
        "acacia_sign" => "minecraft:sign",
        "dark_oak_sign" => "minecraft:sign",
        "crimson_sign" => "minecraft:sign",
        "warped_sign" => "minecraft:sign",
        "skeleton_skull" => "minecraft:skull",
        "wither_skeleton_skull" => "minecraft:skull",
        "zombie_head" => "minecraft:skull",
        "player_head" => "minecraft:skull",
        "creeper_head" => "minecraft:skull",
        "dragon_head" => "minecraft:skull",
        "barrel" => "minecraft:barrel",
        "conduit" => "minecraft:conduit",
        "smoker" => "minecraft:smoker",
        "blast_furnace" => "minecraft:blast_furnace",
        "lectern" => "minecraft:lectern",
        "bell" => "minecraft:bell",
        "jigsaw" => "minecraft:jigsaw",
        "campfire" => "minecraft:campfire",
        "bee_nest" => "minecraft:beehive",
        "beehive" => "minecraft:beehive",
        "sculk_sensor" => "minecraft:sculk_sensor",
        "decorated_pot" => "minecraft:decorated_pot",
        "crafter" => "minecraft:crafter",

        // These are missing from Vanilla (TODO check on update)
        // Can also use the test below to make sure we're synced with Paper's Java version
        "enchanting_table" => "minecraft:enchanting_table",
        "comparator" => "minecraft:comparator",
        "light_gray_bed" => "minecraft:bed",
        "light_gray_banner" => "minecraft:banner",
        "soul_campfire" => "minecraft:campfire",
        "sculk_catalyst" => "minecraft:sculk_catalyst",
        "mangrove_sign" => "minecraft:sign",
        "sculk_shrieker" => "minecraft:sculk_shrieker",
        "chiseled_bookshelf" => "minecraft:chiseled_bookshelf",
        "bamboo_sign" => "minecraft:sign",
        "oak_hanging_sign" => "minecraft:sign",
        "spruce_hanging_sign" => "minecraft:sign",
        "birch_hanging_sign" => "minecraft:sign",
        "jungle_hanging_sign" => "minecraft:sign",
        "acacia_hanging_sign" => "minecraft:sign",
        "dark_oak_hanging_sign" => "minecraft:sign",
        "mangrove_hanging_sign" => "minecraft:sign",
        "bamboo_hanging_sign" => "minecraft:sign",
        "crimson_hanging_sign" => "minecraft:sign",
        "warped_hanging_sign" => "minecraft:sign",
        "piglin_head" => "minecraft:skull",
        "suspicious_sand" => "minecraft:brushable_block", // note: this was renamed in the past, see special case in the itemstack walker
        "cherry_sign" => "minecraft:sign",
        "cherry_hanging_sign" => "minecraft:sign",
        "suspicious_gravel" => "minecraft:brushable_block",
        "calibrated_sculk_sensor" => "minecraft:calibrated_sculk_sensor",
        "trial_spawner" => "minecraft:trial-spawner",
    }
}

static_string_map! {
    TILE_ID_UPDATE, tile_id_update, {
        "Airportal" => "minecraft:end_portal",
        "Banner" => "minecraft:banner",
        "Beacon" => "minecraft:beacon",
        "Cauldron" => "minecraft:brewing_stand",
        "Chest" => "minecraft:chest",
        "Comparator" => "minecraft:comparator",
        "Control" => "minecraft:command_block",
        "DLDetector" => "minecraft:daylight_detector",
        "Dropper" => "minecraft:dropper",
        "EnchantTable" => "minecraft:enchanting_table",
        "EndGateway" => "minecraft:end_gateway",
        "EnderChest" => "minecraft:ender_chest",
        "FlowerPot" => "minecraft:flower_pot",
        "Furnace" => "minecraft:furnace",
        "Hopper" => "minecraft:hopper",
        "MobSpawner" => "minecraft:mob_spawner",
        "Music" => "minecraft:noteblock",
        "Piston" => "minecraft:piston",
        "RecordPlayer" => "minecraft:jukebox",
        "Sign" => "minecraft:sign",
        "Skull" => "minecraft:skull",
        "Structure" => "minecraft:structure_block",
        "Trap" => "minecraft:dispenser",
    }
}

pub(crate) fn register() {
    types::tile_entity_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            if let Some(JValue::String(id)) = data.get_mut("id") {
                if let Some(new_id) = tile_id_update().get(&id[..]).copied() {
                    *id = new_id.to_owned();
                }
            }
        }),
    );

    register_inventory("minecraft:furnace");
    register_inventory("minecraft:chest");
    types::tile_entity_mut().add_walker_for_id(
        VERSION,
        "minecraft:jukebox",
        DataWalkerMapTypePaths::new(types::item_stack_ref(), "RecordItem"),
    );
    register_inventory("minecraft:dispenser");
    register_inventory("minecraft:dropper");
    types::tile_entity_mut().add_walker_for_id(
        VERSION,
        "minecraft:mob_spawner",
        data_walker(move |data, from_version, to_version| {
            types::untagged_spawner().convert(data, from_version, to_version);
        }),
    );
    register_inventory("minecraft:brewing_stand");
    register_inventory("minecraft:hopper");
    types::tile_entity_mut().add_walker_for_id(
        VERSION,
        "minecraft:flower_pot",
        DataWalkerObjectTypePaths::new(types::item_name_ref(), "Item"),
    );

    types::item_stack_mut().add_structure_walker(
        VERSION,
        data_walker(move |data, from_version, to_version| {
            convert_object_in_map(types::item_name_ref(), data, "id", from_version, to_version);
            let [item_id, tag] = get_mut_multi(data, ["id", "tag"]);
            let item_id = item_id.map(|v| &*v);

            let Some(JValue::Compound(tag)) = tag else {
                return;
            };

            let item_id_str = match item_id {
                Some(JValue::String(item_id_str)) => Some(&item_id_str[..]),
                _ => None,
            };

            // only things here are in tag, if changed update if above

            convert_map_list_in_map(
                types::item_stack_ref(),
                tag,
                "Items",
                from_version,
                to_version,
            );

            if let Some(JValue::Compound(entity_tag)) = tag.get_mut("EntityTag") {
                let entity_id = match item_id_str.map(JavaStr::as_bytes) {
                    // The check for version id is removed here. For whatever reason, the legacy
                    // data converters used entity id "minecraft:armor_stand" when version was greater-than 514,
                    // but entity ids were not namespaced until V705! So somebody fucked up the legacy converters.
                    // DFU agrees with my analysis here, it will only set the entityId here to the namespaced variant
                    // with the V705 schema.
                    Some(b"minecraft:armor_stand") => Some(if from_version.get_version() < 705 {
                        JavaStr::from_str("ArmorStand")
                    } else {
                        JavaStr::from_str("minecraft:armor_stand")
                    }),
                    // add missing item_frame entity id
                    // version check is same for armorstand, as both were namespaced at the same time
                    Some(b"minecraft:item_frame") => Some(if from_version.get_version() < 705 {
                        JavaStr::from_str("ItemFrame")
                    } else {
                        JavaStr::from_str("minecraft:item_frame")
                    }),
                    // add missing glow_item_frame entity id
                    Some(b"minecraft:glow_item_frame") => {
                        Some(JavaStr::from_str("minecraft:glow_item_frame"))
                    }
                    // V1451 changes spawn eggs to have the sub entity id be a part of the item id, but of course Mojang never
                    // bothered to write in logic to set the sub entity id, so we have to.
                    // format is ALWAYS <namespace>:<id>_spawn_egg post flattening
                    Some(_) => {
                        item_id_str
                            .unwrap()
                            .strip_suffix("_spawn_egg")
                            .or_else(|| match entity_tag.get("id") {
                                Some(JValue::String(id)) => Some(id),
                                _ => None,
                            })
                    }
                    None => match entity_tag.get("id") {
                        Some(JValue::String(id)) => Some(&id[..]),
                        _ => None,
                    },
                };

                let remove_id = if let Some(entity_id) = entity_id {
                    let remove_id = !matches!(entity_tag.get("id"), Some(JValue::String(_)));
                    if remove_id {
                        entity_tag.insert("id", entity_id.to_owned());
                    }
                    remove_id
                } else {
                    if item_id_str != Some(JavaStr::from_str("minecraft:air")) {
                        warn!(
                            "Unable to resolve Entity for ItemStack (V704): {:?}",
                            item_id
                        );
                    }
                    false
                };

                types::entity().convert(entity_tag, from_version, to_version);

                if remove_id {
                    entity_tag.remove("id");
                }
            }

            if let Some(JValue::Compound(block_entity_tag)) = tag.get_mut("BlockEntityTag") {
                let entity_id =
                    item_id_str.and_then(|id| item_id_to_tile_entity_id().get(id).copied());

                let remove_id = if let Some(entity_id) = entity_id {
                    let remove_id = !matches!(block_entity_tag.get("id"), Some(JValue::String(_)));
                    block_entity_tag.insert("id", entity_id);
                    remove_id
                } else {
                    if item_id_str != Some(JavaStr::from_str("minecraft:air")) {
                        warn!(
                            "Unable to resolve BlockEntity for ItemStack (V704): {:?}",
                            item_id
                        );
                    }
                    false
                };

                types::tile_entity().convert(block_entity_tag, from_version, to_version);

                if remove_id {
                    block_entity_tag.remove("id");
                }
            }

            convert_object_list_in_map(
                types::block_name_ref(),
                tag,
                "CanDestroy",
                from_version,
                to_version,
            );
            convert_object_list_in_map(
                types::block_name_ref(),
                tag,
                "CanPlaceOn",
                from_version,
                to_version,
            );
        }),
    );

    // Enforce namespace for ids
    types::tile_entity_mut().add_structure_hook(VERSION, DataHookEnforceNamespacedId::id());
}

fn register_inventory(id: impl Into<JavaString>) {
    types::tile_entity_mut().add_walker_for_id(
        VERSION,
        id,
        DataWalkerMapListPaths::new(types::item_stack_ref(), "Items"),
    );
}

#[cfg(test)]
mod test {
    use java_string::{format_java, JavaString};
    use std::collections::BTreeMap;

    #[test]
    #[cfg(feature = "update_checks")]
    fn test_in_sync_with_paper() {
        use crate::versions::v704::item_id_to_tile_entity_id;
        use std::io::{BufRead, BufReader};

        let regex = regex::Regex::new(r#"\s*ITEM_ID_TO_TILE_ENTITY_ID\s*\.\s*put\s*\("([\w:]*)"\s*,\s*"([\w:]*)"\s*\)\s*;\s*"#).unwrap();

        const URL: &str = "https://raw.githubusercontent.com/PaperMC/DataConverter/master/src/main/java/ca/spottedleaf/dataconverter/minecraft/versions/V704.java";
        let response = attohttpc::get(URL)
            .send()
            .expect("Failed to download V704.java");
        let mut paper_map = BTreeMap::new();
        for line in BufReader::new(response).lines() {
            let line = line.expect("Failed to download V704.java");
            if let Some(captures) = regex.captures(&line) {
                paper_map.insert(
                    JavaString::from(captures.get(1).unwrap().as_str()),
                    JavaString::from(captures.get(2).unwrap().as_str()),
                );
            }
        }

        let our_map = item_id_to_tile_entity_id();

        let mut missing_from_ours = Vec::new();
        let mut missing_from_paper = Vec::new();
        let mut different = Vec::new();
        for (our_key, our_val) in our_map.iter_mc_to_value() {
            let our_key = format_java!("minecraft:{our_key}");
            match paper_map.get(&our_key) {
                Some(paper_val) => {
                    if paper_val != our_val {
                        different.push(our_key);
                    }
                }
                None => missing_from_paper.push(our_key),
            }
        }
        for paper_key in paper_map.keys() {
            if !our_map.contains_key(paper_key) {
                missing_from_ours.push(paper_key);
            }
        }

        if !missing_from_ours.is_empty() || !missing_from_paper.is_empty() || !different.is_empty()
        {
            let mut error = String::new();
            if !missing_from_ours.is_empty() {
                error.push_str(
                    format!(
                        "Paper has {} entries missing from ours:\n",
                        missing_from_ours.len()
                    )
                    .as_str(),
                );
                for elem in missing_from_ours {
                    error.push_str(format!("- {}\n", elem).as_str());
                }
            }
            if !missing_from_paper.is_empty() {
                error.push_str(
                    format!(
                        "We have {} extra entries that are absent in Paper:\n",
                        missing_from_paper.len()
                    )
                    .as_str(),
                );
                for elem in missing_from_paper {
                    error.push_str(format!("- {}\n", elem).as_str());
                }
            }
            if !different.is_empty() {
                error.push_str(
                    format!(
                        "We have {} entries that are different from Paper:\n",
                        different.len()
                    )
                    .as_str(),
                );
                for elem in different {
                    error.push_str(
                        format!(
                            "- {}: {} (ours) vs {} (paper)\n",
                            elem,
                            our_map.get(&elem).unwrap(),
                            paper_map.get(&elem).unwrap()
                        )
                        .as_str(),
                    );
                }
            }
            if error.ends_with('\n') {
                error.remove(error.len() - 1);
            }

            assert!(false, "{}", error);
        }
    }
}
