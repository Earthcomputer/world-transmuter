use crate::helpers::hooks::DataHookEnforceNamespacedId;
use crate::helpers::mc_namespace_map::McNamespaceMap;
use crate::MinecraftTypesMut;
use log::warn;
use rust_dataconverter_engine::{
    convert_map_list_in_map, convert_object_in_map, convert_object_list_in_map, data_walker,
    get_mut_multi, map_data_converter_func, AbstractMapDataType, DataWalkerMapListPaths,
    DataWalkerMapTypePaths, DataWalkerObjectTypePaths,
};
use std::collections::BTreeMap;
use std::sync::OnceLock;
use valence_nbt::Value;

const VERSION: u32 = 704;

static ITEM_ID_TO_TILE_ENTITY_ID: OnceLock<McNamespaceMap<&'static str>> = OnceLock::new();

fn item_id_to_tile_entity_id() -> &'static McNamespaceMap<'static, &'static str> {
    ITEM_ID_TO_TILE_ENTITY_ID.get_or_init(|| {
        let mut map = McNamespaceMap::new();
        map.insert_mc("furnace", "minecraft:furnace");
        map.insert_mc("lit_furnace", "minecraft:furnace");
        map.insert_mc("chest", "minecraft:chest");
        map.insert_mc("trapped_chest", "minecraft:chest");
        map.insert_mc("ender_chest", "minecraft:ender_chest");
        map.insert_mc("jukebox", "minecraft:jukebox");
        map.insert_mc("dispenser", "minecraft:dispenser");
        map.insert_mc("dropper", "minecraft:dropper");
        map.insert_mc("sign", "minecraft:sign");
        map.insert_mc("mob_spawner", "minecraft:mob_spawner");
        map.insert_mc("spawner", "minecraft:mob_spawner");
        map.insert_mc("noteblock", "minecraft:noteblock");
        map.insert_mc("brewing_stand", "minecraft:brewing_stand");
        map.insert_mc("enhanting_table", "minecraft:enchanting_table");
        map.insert_mc("command_block", "minecraft:command_block");
        map.insert_mc("beacon", "minecraft:beacon");
        map.insert_mc("skull", "minecraft:skull");
        map.insert_mc("daylight_detector", "minecraft:daylight_detector");
        map.insert_mc("hopper", "minecraft:hopper");
        map.insert_mc("banner", "minecraft:banner");
        map.insert_mc("flower_pot", "minecraft:flower_pot");
        map.insert_mc("repeating_command_block", "minecraft:command_block");
        map.insert_mc("chain_command_block", "minecraft:command_block");
        map.insert_mc("shulker_box", "minecraft:shulker_box");
        map.insert_mc("white_shulker_box", "minecraft:shulker_box");
        map.insert_mc("orange_shulker_box", "minecraft:shulker_box");
        map.insert_mc("magenta_shulker_box", "minecraft:shulker_box");
        map.insert_mc("light_blue_shulker_box", "minecraft:shulker_box");
        map.insert_mc("yellow_shulker_box", "minecraft:shulker_box");
        map.insert_mc("lime_shulker_box", "minecraft:shulker_box");
        map.insert_mc("pink_shulker_box", "minecraft:shulker_box");
        map.insert_mc("gray_shulker_box", "minecraft:shulker_box");
        map.insert_mc("silver_shulker_box", "minecraft:shulker_box");
        map.insert_mc("cyan_shulker_box", "minecraft:shulker_box");
        map.insert_mc("purple_shulker_box", "minecraft:shulker_box");
        map.insert_mc("blue_shulker_box", "minecraft:shulker_box");
        map.insert_mc("brown_shulker_box", "minecraft:shulker_box");
        map.insert_mc("green_shulker_box", "minecraft:shulker_box");
        map.insert_mc("red_shulker_box", "minecraft:shulker_box");
        map.insert_mc("black_shulker_box", "minecraft:shulker_box");
        map.insert_mc("bed", "minecraft:bed");
        map.insert_mc("light_gray_shulker_box", "minecraft:shulker_box");
        map.insert_mc("white_banner", "minecraft:banner");
        map.insert_mc("orange_banner", "minecraft:banner");
        map.insert_mc("magenta_banner", "minecraft:banner");
        map.insert_mc("light_blue_banner", "minecraft:banner");
        map.insert_mc("yellow_banner", "minecraft:banner");
        map.insert_mc("lime_banner", "minecraft:banner");
        map.insert_mc("pink_banner", "minecraft:banner");
        map.insert_mc("gray_banner", "minecraft:banner");
        map.insert_mc("silver_banner", "minecraft:banner");
        map.insert_mc("cyan_banner", "minecraft:banner");
        map.insert_mc("purple_banner", "minecraft:banner");
        map.insert_mc("blue_banner", "minecraft:banner");
        map.insert_mc("brown_banner", "minecraft:banner");
        map.insert_mc("green_banner", "minecraft:banner");
        map.insert_mc("red_banner", "minecraft:banner");
        map.insert_mc("black_banner", "minecraft:banner");
        map.insert_mc("standing_sign", "minecraft:sign");
        map.insert_mc("wall_sign", "minecraft:sign");
        map.insert_mc("piston_head", "minecraft:piston");
        map.insert_mc("daylight_detector_inverted", "minecraft:daylight_detector");
        map.insert_mc("unpowered_comparator", "minecraft:comparator");
        map.insert_mc("powered_comparator", "minecraft:comparator");
        map.insert_mc("wall_banner", "minecraft:banner");
        map.insert_mc("standing_banner", "minecraft:banner");
        map.insert_mc("structure_block", "minecraft:structure_block");
        map.insert_mc("end_portal", "minecraft:end_portal");
        map.insert_mc("end_gateway", "minecraft:end_gateway");
        map.insert_mc("shield", "minecraft:banner");
        map.insert_mc("white_bed", "minecraft:bed");
        map.insert_mc("orange_bed", "minecraft:bed");
        map.insert_mc("magenta_bed", "minecraft:bed");
        map.insert_mc("light_blue_bed", "minecraft:bed");
        map.insert_mc("yellow_bed", "minecraft:bed");
        map.insert_mc("lime_bed", "minecraft:bed");
        map.insert_mc("pink_bed", "minecraft:bed");
        map.insert_mc("gray_bed", "minecraft:bed");
        map.insert_mc("silver_bed", "minecraft:bed");
        map.insert_mc("cyan_bed", "minecraft:bed");
        map.insert_mc("purple_bed", "minecraft:bed");
        map.insert_mc("blue_bed", "minecraft:bed");
        map.insert_mc("brown_bed", "minecraft:bed");
        map.insert_mc("green_bed", "minecraft:bed");
        map.insert_mc("red_bed", "minecraft:bed");
        map.insert_mc("black_bed", "minecraft:bed");
        map.insert_mc("oak_sign", "minecraft:sign");
        map.insert_mc("spruce_sign", "minecraft:sign");
        map.insert_mc("birch_sign", "minecraft:sign");
        map.insert_mc("jungle_sign", "minecraft:sign");
        map.insert_mc("acacia_sign", "minecraft:sign");
        map.insert_mc("dark_oak_sign", "minecraft:sign");
        map.insert_mc("crimson_sign", "minecraft:sign");
        map.insert_mc("warped_sign", "minecraft:sign");
        map.insert_mc("skeleton_skull", "minecraft:skull");
        map.insert_mc("wither_skeleton_skull", "minecraft:skull");
        map.insert_mc("zombie_head", "minecraft:skull");
        map.insert_mc("player_head", "minecraft:skull");
        map.insert_mc("creeper_head", "minecraft:skull");
        map.insert_mc("dragon_head", "minecraft:skull");
        map.insert_mc("barrel", "minecraft:barrel");
        map.insert_mc("conduit", "minecraft:conduit");
        map.insert_mc("smoker", "minecraft:smoker");
        map.insert_mc("blast_furnace", "minecraft:blast_furnace");
        map.insert_mc("lectern", "minecraft:lectern");
        map.insert_mc("bell", "minecraft:bell");
        map.insert_mc("jigsaw", "minecraft:jigsaw");
        map.insert_mc("campfire", "minecraft:campfire");
        map.insert_mc("bee_nest", "minecraft:beehive");
        map.insert_mc("beehive", "minecraft:beehive");
        map.insert_mc("sculk_sensor", "minecraft:sculk_sensor");

        // These are missing from Vanilla (TODO check on update)
        // Can also use the test below to make sure we're synced with Paper's Java version
        map.insert_mc("enchanting_table", "minecraft:enchanting_table");
        map.insert_mc("comparator", "minecraft:comparator");
        map.insert_mc("light_gray_bed", "minecraft:bed");
        map.insert_mc("light_gray_banner", "minecraft:banner");
        map.insert_mc("soul_campfire", "minecraft:campfire");
        map
    })
}

static TILE_ID_UPDATE: OnceLock<BTreeMap<&'static str, &'static str>> = OnceLock::new();

fn tile_id_update() -> &'static BTreeMap<&'static str, &'static str> {
    TILE_ID_UPDATE.get_or_init(|| {
        let mut map = BTreeMap::new();
        map.insert("Airportal", "minecraft:end_portal");
        map.insert("Banner", "minecraft:banner");
        map.insert("Beacon", "minecraft:beacon");
        map.insert("Cauldron", "minecraft:brewing_stand");
        map.insert("Chest", "minecraft:chest");
        map.insert("Comparator", "minecraft:comparator");
        map.insert("Control", "minecraft:command_block");
        map.insert("DLDetector", "minecraft:daylight_detector");
        map.insert("Dropper", "minecraft:dropper");
        map.insert("EnchantTable", "minecraft:enchanting_table");
        map.insert("EndGateway", "minecraft:end_gateway");
        map.insert("EnderChest", "minecraft:ender_chest");
        map.insert("FlowerPot", "minecraft:flower_pot");
        map.insert("Furnace", "minecraft:furnace");
        map.insert("Hopper", "minecraft:hopper");
        map.insert("MobSpawner", "minecraft:mob_spawner");
        map.insert("Music", "minecraft:noteblock");
        map.insert("Piston", "minecraft:piston");
        map.insert("RecordPlayer", "minecraft:jukebox");
        map.insert("Sign", "minecraft:sign");
        map.insert("Skull", "minecraft:skull");
        map.insert("Structure", "minecraft:structure_block");
        map.insert("Trap", "minecraft:dispenser");
        map
    })
}

pub(crate) fn register(types: MinecraftTypesMut) {
    types.tile_entity().borrow_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            if let Some(Value::String(id)) = data.get_mut("id") {
                if let Some(new_id) = tile_id_update().get(&id[..]).copied() {
                    *id = new_id.to_owned();
                }
            }
        }),
    );

    register_inventory(types, "minecraft:furnace");
    register_inventory(types, "minecraft:chest");
    types.tile_entity().borrow_mut().add_walker_for_id(
        VERSION,
        "minecraft:jukebox",
        DataWalkerMapTypePaths::new(types.item_stack(), "RecordItem"),
    );
    register_inventory(types, "minecraft:dispenser");
    register_inventory(types, "minecraft:dropper");
    let untagged_spawner_type = types.untagged_spawner();
    types.tile_entity().borrow_mut().add_walker_for_id(
        VERSION,
        "minecraft:mob_spawner",
        data_walker(move |data, from_version, to_version| {
            untagged_spawner_type.convert(data, from_version, to_version);
        }),
    );
    register_inventory(types, "minecraft:brewing_stand");
    register_inventory(types, "minecraft:hopper");
    types.tile_entity().borrow_mut().add_walker_for_id(
        VERSION,
        "minecraft:flower_pot",
        DataWalkerObjectTypePaths::new(types.item_name(), "Item"),
    );

    let block_name_type = types.block_name();
    let entity_type = types.entity();
    let item_name_type = types.item_name();
    let item_stack_type = types.item_stack();
    let tile_entity_type = types.tile_entity();
    types.item_stack().borrow_mut().add_structure_walker(
        VERSION,
        data_walker(move |data, from_version, to_version| {
            convert_object_in_map(item_name_type, data, "id", from_version, to_version);
            let [item_id, tag] = get_mut_multi(data, ["id", "tag"]);
            let item_id = item_id.map(|v| &*v);

            let Some(Value::Compound(tag)) = tag else {
                return;
            };

            let item_id_str = match item_id {
                Some(Value::String(item_id_str)) => Some(&item_id_str[..]),
                _ => None,
            };

            // only things here are in tag, if changed update if above

            convert_map_list_in_map(item_stack_type, tag, "Items", from_version, to_version);

            if let Some(Value::Compound(entity_tag)) = tag.get_mut("EntityTag") {
                let entity_id = match item_id_str {
                    // The check for version id is removed here. For whatever reason, the legacy
                    // data converters used entity id "minecraft:armor_stand" when version was greater-than 514,
                    // but entity ids were not namespaced until V705! So somebody fucked up the legacy converters.
                    // DFU agrees with my analysis here, it will only set the entityId here to the namespaced variant
                    // with the V705 schema.
                    Some("minecraft:armor_stand") => Some(if from_version.get_version() < 705 {
                        "ArmorStand"
                    } else {
                        "minecraft:armor_stand"
                    }),
                    // add missing item_frame entity id
                    // version check is same for armorstand, as both were namespaced at the same time
                    Some("minecraft:item_frame") => Some(if from_version.get_version() < 705 {
                        "ItemFrame"
                    } else {
                        "minecraft:item_frame"
                    }),
                    // add missing glow_item_frame entity id
                    Some("minecraft:glow_item_frame") => Some("minecraft:glow_item_frame"),
                    // V1451 changes spawn eggs to have the sub entity id be a part of the item id, but of course Mojang never
                    // bothered to write in logic to set the sub entity id, so we have to.
                    // format is ALWAYS <namespace>:<id>_spawn_egg post flattening
                    Some(item_id_str) => item_id_str.strip_suffix("_spawn_egg").or_else(|| {
                        match entity_tag.get("id") {
                            Some(Value::String(id)) => Some(id),
                            _ => None,
                        }
                    }),
                    None => match entity_tag.get("id") {
                        Some(Value::String(id)) => Some(&id[..]),
                        _ => None,
                    },
                };

                let remove_id = if let Some(entity_id) = entity_id {
                    let remove_id = !matches!(entity_tag.get("id"), Some(Value::String(_)));
                    if remove_id {
                        entity_tag.insert("id", entity_id.to_owned());
                    }
                    remove_id
                } else {
                    if item_id_str != Some("minecraft:air") {
                        warn!(
                            "Unable to resolve Entity for ItemStack (V704): {:?}",
                            item_id
                        );
                    }
                    false
                };

                entity_type.convert(entity_tag, from_version, to_version);

                if remove_id {
                    entity_tag.remove("id");
                }
            }

            if let Some(Value::Compound(block_entity_tag)) = tag.get_mut("BlockEntityTag") {
                let entity_id =
                    item_id_str.and_then(|id| item_id_to_tile_entity_id().get(id).copied());

                let remove_id = if let Some(entity_id) = entity_id {
                    let remove_id = !matches!(block_entity_tag.get("id"), Some(Value::String(_)));
                    block_entity_tag.insert("id", entity_id);
                    remove_id
                } else {
                    if item_id_str != Some("minecraft:air") {
                        warn!(
                            "Unable to resolve BlockEntity for ItemStack (V704): {:?}",
                            item_id
                        );
                    }
                    false
                };

                tile_entity_type.convert(block_entity_tag, from_version, to_version);

                if remove_id {
                    block_entity_tag.remove("id");
                }
            }

            convert_object_list_in_map(
                block_name_type,
                tag,
                "CanDestroy",
                from_version,
                to_version,
            );
            convert_object_list_in_map(
                block_name_type,
                tag,
                "CanPlaceOn",
                from_version,
                to_version,
            );
        }),
    );

    // Enforce namespace for ids
    types
        .tile_entity()
        .borrow_mut()
        .add_structure_hook(VERSION, DataHookEnforceNamespacedId::id());
}

fn register_inventory(types: MinecraftTypesMut, id: impl Into<String>) {
    types.tile_entity().borrow_mut().add_walker_for_id(
        VERSION,
        id,
        DataWalkerMapListPaths::new(types.item_stack(), "Items"),
    );
}

#[cfg(test)]
mod test {
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
                    captures.get(1).unwrap().as_str().to_owned(),
                    captures.get(2).unwrap().as_str().to_owned(),
                );
            }
        }

        let our_map = item_id_to_tile_entity_id();

        let mut missing_from_ours = Vec::new();
        let mut missing_from_paper = Vec::new();
        let mut different = Vec::new();
        for (our_key, our_val) in our_map {
            match paper_map.get(our_key) {
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
                            our_map.get(elem).unwrap(),
                            paper_map.get(elem).unwrap()
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
