use std::lazy::SyncOnceCell;
use log::warn;
use rust_dataconverter_engine::{convert_map_list_in_map, convert_object_in_map, convert_object_list_in_map, data_converter_func, data_walker, DataType, DataWalkerMapListPaths, DataWalkerMapTypePaths, DataWalkerObjectTypePaths, MapType, ObjectType, Types};
use crate::helpers::hooks::DataHookEnforceNamespacedId;
use crate::MinecraftTypesMut;

const VERSION: u32 = 704;

static ITEM_ID_TO_TILE_ENTITY_ID: SyncOnceCell<rust_dataconverter_engine::Map<String, String>> = SyncOnceCell::new();

fn item_id_to_tile_entity_id() -> &'static rust_dataconverter_engine::Map<String, String> {
    ITEM_ID_TO_TILE_ENTITY_ID.get_or_init(|| {
        let mut map = rust_dataconverter_engine::Map::new();
        map.insert("minecraft:furnace".to_owned(), "minecraft:furnace".to_owned());
        map.insert("minecraft:lit_furnace".to_owned(), "minecraft:furnace".to_owned());
        map.insert("minecraft:chest".to_owned(), "minecraft:chest".to_owned());
        map.insert("minecraft:trapped_chest".to_owned(), "minecraft:chest".to_owned());
        map.insert("minecraft:ender_chest".to_owned(), "minecraft:ender_chest".to_owned());
        map.insert("minecraft:jukebox".to_owned(), "minecraft:jukebox".to_owned());
        map.insert("minecraft:dispenser".to_owned(), "minecraft:dispenser".to_owned());
        map.insert("minecraft:dropper".to_owned(), "minecraft:dropper".to_owned());
        map.insert("minecraft:sign".to_owned(), "minecraft:sign".to_owned());
        map.insert("minecraft:mob_spawner".to_owned(), "minecraft:mob_spawner".to_owned());
        map.insert("minecraft:spawner".to_owned(), "minecraft:mob_spawner".to_owned());
        map.insert("minecraft:noteblock".to_owned(), "minecraft:noteblock".to_owned());
        map.insert("minecraft:brewing_stand".to_owned(), "minecraft:brewing_stand".to_owned());
        map.insert("minecraft:enhanting_table".to_owned(), "minecraft:enchanting_table".to_owned());
        map.insert("minecraft:command_block".to_owned(), "minecraft:command_block".to_owned());
        map.insert("minecraft:beacon".to_owned(), "minecraft:beacon".to_owned());
        map.insert("minecraft:skull".to_owned(), "minecraft:skull".to_owned());
        map.insert("minecraft:daylight_detector".to_owned(), "minecraft:daylight_detector".to_owned());
        map.insert("minecraft:hopper".to_owned(), "minecraft:hopper".to_owned());
        map.insert("minecraft:banner".to_owned(), "minecraft:banner".to_owned());
        map.insert("minecraft:flower_pot".to_owned(), "minecraft:flower_pot".to_owned());
        map.insert("minecraft:repeating_command_block".to_owned(), "minecraft:command_block".to_owned());
        map.insert("minecraft:chain_command_block".to_owned(), "minecraft:command_block".to_owned());
        map.insert("minecraft:shulker_box".to_owned(), "minecraft:shulker_box".to_owned());
        map.insert("minecraft:white_shulker_box".to_owned(), "minecraft:shulker_box".to_owned());
        map.insert("minecraft:orange_shulker_box".to_owned(), "minecraft:shulker_box".to_owned());
        map.insert("minecraft:magenta_shulker_box".to_owned(), "minecraft:shulker_box".to_owned());
        map.insert("minecraft:light_blue_shulker_box".to_owned(), "minecraft:shulker_box".to_owned());
        map.insert("minecraft:yellow_shulker_box".to_owned(), "minecraft:shulker_box".to_owned());
        map.insert("minecraft:lime_shulker_box".to_owned(), "minecraft:shulker_box".to_owned());
        map.insert("minecraft:pink_shulker_box".to_owned(), "minecraft:shulker_box".to_owned());
        map.insert("minecraft:gray_shulker_box".to_owned(), "minecraft:shulker_box".to_owned());
        map.insert("minecraft:silver_shulker_box".to_owned(), "minecraft:shulker_box".to_owned());
        map.insert("minecraft:cyan_shulker_box".to_owned(), "minecraft:shulker_box".to_owned());
        map.insert("minecraft:purple_shulker_box".to_owned(), "minecraft:shulker_box".to_owned());
        map.insert("minecraft:blue_shulker_box".to_owned(), "minecraft:shulker_box".to_owned());
        map.insert("minecraft:brown_shulker_box".to_owned(), "minecraft:shulker_box".to_owned());
        map.insert("minecraft:green_shulker_box".to_owned(), "minecraft:shulker_box".to_owned());
        map.insert("minecraft:red_shulker_box".to_owned(), "minecraft:shulker_box".to_owned());
        map.insert("minecraft:black_shulker_box".to_owned(), "minecraft:shulker_box".to_owned());
        map.insert("minecraft:bed".to_owned(), "minecraft:bed".to_owned());
        map.insert("minecraft:light_gray_shulker_box".to_owned(), "minecraft:shulker_box".to_owned());
        map.insert("minecraft:white_banner".to_owned(), "minecraft:banner".to_owned());
        map.insert("minecraft:orange_banner".to_owned(), "minecraft:banner".to_owned());
        map.insert("minecraft:magenta_banner".to_owned(), "minecraft:banner".to_owned());
        map.insert("minecraft:light_blue_banner".to_owned(), "minecraft:banner".to_owned());
        map.insert("minecraft:yellow_banner".to_owned(), "minecraft:banner".to_owned());
        map.insert("minecraft:lime_banner".to_owned(), "minecraft:banner".to_owned());
        map.insert("minecraft:pink_banner".to_owned(), "minecraft:banner".to_owned());
        map.insert("minecraft:gray_banner".to_owned(), "minecraft:banner".to_owned());
        map.insert("minecraft:silver_banner".to_owned(), "minecraft:banner".to_owned());
        map.insert("minecraft:cyan_banner".to_owned(), "minecraft:banner".to_owned());
        map.insert("minecraft:purple_banner".to_owned(), "minecraft:banner".to_owned());
        map.insert("minecraft:blue_banner".to_owned(), "minecraft:banner".to_owned());
        map.insert("minecraft:brown_banner".to_owned(), "minecraft:banner".to_owned());
        map.insert("minecraft:green_banner".to_owned(), "minecraft:banner".to_owned());
        map.insert("minecraft:red_banner".to_owned(), "minecraft:banner".to_owned());
        map.insert("minecraft:black_banner".to_owned(), "minecraft:banner".to_owned());
        map.insert("minecraft:standing_sign".to_owned(), "minecraft:sign".to_owned());
        map.insert("minecraft:wall_sign".to_owned(), "minecraft:sign".to_owned());
        map.insert("minecraft:piston_head".to_owned(), "minecraft:piston".to_owned());
        map.insert("minecraft:daylight_detector_inverted".to_owned(), "minecraft:daylight_detector".to_owned());
        map.insert("minecraft:unpowered_comparator".to_owned(), "minecraft:comparator".to_owned());
        map.insert("minecraft:powered_comparator".to_owned(), "minecraft:comparator".to_owned());
        map.insert("minecraft:wall_banner".to_owned(), "minecraft:banner".to_owned());
        map.insert("minecraft:standing_banner".to_owned(), "minecraft:banner".to_owned());
        map.insert("minecraft:structure_block".to_owned(), "minecraft:structure_block".to_owned());
        map.insert("minecraft:end_portal".to_owned(), "minecraft:end_portal".to_owned());
        map.insert("minecraft:end_gateway".to_owned(), "minecraft:end_gateway".to_owned());
        map.insert("minecraft:shield".to_owned(), "minecraft:banner".to_owned());
        map.insert("minecraft:white_bed".to_owned(), "minecraft:bed".to_owned());
        map.insert("minecraft:orange_bed".to_owned(), "minecraft:bed".to_owned());
        map.insert("minecraft:magenta_bed".to_owned(), "minecraft:bed".to_owned());
        map.insert("minecraft:light_blue_bed".to_owned(), "minecraft:bed".to_owned());
        map.insert("minecraft:yellow_bed".to_owned(), "minecraft:bed".to_owned());
        map.insert("minecraft:lime_bed".to_owned(), "minecraft:bed".to_owned());
        map.insert("minecraft:pink_bed".to_owned(), "minecraft:bed".to_owned());
        map.insert("minecraft:gray_bed".to_owned(), "minecraft:bed".to_owned());
        map.insert("minecraft:silver_bed".to_owned(), "minecraft:bed".to_owned());
        map.insert("minecraft:cyan_bed".to_owned(), "minecraft:bed".to_owned());
        map.insert("minecraft:purple_bed".to_owned(), "minecraft:bed".to_owned());
        map.insert("minecraft:blue_bed".to_owned(), "minecraft:bed".to_owned());
        map.insert("minecraft:brown_bed".to_owned(), "minecraft:bed".to_owned());
        map.insert("minecraft:green_bed".to_owned(), "minecraft:bed".to_owned());
        map.insert("minecraft:red_bed".to_owned(), "minecraft:bed".to_owned());
        map.insert("minecraft:black_bed".to_owned(), "minecraft:bed".to_owned());
        map.insert("minecraft:oak_sign".to_owned(), "minecraft:sign".to_owned());
        map.insert("minecraft:spruce_sign".to_owned(), "minecraft:sign".to_owned());
        map.insert("minecraft:birch_sign".to_owned(), "minecraft:sign".to_owned());
        map.insert("minecraft:jungle_sign".to_owned(), "minecraft:sign".to_owned());
        map.insert("minecraft:acacia_sign".to_owned(), "minecraft:sign".to_owned());
        map.insert("minecraft:dark_oak_sign".to_owned(), "minecraft:sign".to_owned());
        map.insert("minecraft:crimson_sign".to_owned(), "minecraft:sign".to_owned());
        map.insert("minecraft:warped_sign".to_owned(), "minecraft:sign".to_owned());
        map.insert("minecraft:skeleton_skull".to_owned(), "minecraft:skull".to_owned());
        map.insert("minecraft:wither_skeleton_skull".to_owned(), "minecraft:skull".to_owned());
        map.insert("minecraft:zombie_head".to_owned(), "minecraft:skull".to_owned());
        map.insert("minecraft:player_head".to_owned(), "minecraft:skull".to_owned());
        map.insert("minecraft:creeper_head".to_owned(), "minecraft:skull".to_owned());
        map.insert("minecraft:dragon_head".to_owned(), "minecraft:skull".to_owned());
        map.insert("minecraft:barrel".to_owned(), "minecraft:barrel".to_owned());
        map.insert("minecraft:conduit".to_owned(), "minecraft:conduit".to_owned());
        map.insert("minecraft:smoker".to_owned(), "minecraft:smoker".to_owned());
        map.insert("minecraft:blast_furnace".to_owned(), "minecraft:blast_furnace".to_owned());
        map.insert("minecraft:lectern".to_owned(), "minecraft:lectern".to_owned());
        map.insert("minecraft:bell".to_owned(), "minecraft:bell".to_owned());
        map.insert("minecraft:jigsaw".to_owned(), "minecraft:jigsaw".to_owned());
        map.insert("minecraft:campfire".to_owned(), "minecraft:campfire".to_owned());
        map.insert("minecraft:bee_nest".to_owned(), "minecraft:beehive".to_owned());
        map.insert("minecraft:beehive".to_owned(), "minecraft:beehive".to_owned());
        map.insert("minecraft:sculk_sensor".to_owned(), "minecraft:sculk_sensor".to_owned());

        // These are missing from Vanilla (TODO check on update)
        // Can also use the test below to make sure we're synced with Paper's Java version
        map.insert("minecraft:enchanting_table".to_owned(), "minecraft:enchanting_table".to_owned());
        map.insert("minecraft:comparator".to_owned(), "minecraft:comparator".to_owned());
        map.insert("minecraft:light_gray_bed".to_owned(), "minecraft:bed".to_owned());
        map.insert("minecraft:light_gray_banner".to_owned(), "minecraft:banner".to_owned());
        map.insert("minecraft:soul_campfire".to_owned(), "minecraft:campfire".to_owned());
        map
    })
}

static TILE_ID_UPDATE: SyncOnceCell<rust_dataconverter_engine::Map<String, String>> = SyncOnceCell::new();

fn tile_id_update() -> &'static rust_dataconverter_engine::Map<String, String> {
    TILE_ID_UPDATE.get_or_init(|| {
        let mut map = rust_dataconverter_engine::Map::new();
        map.insert("Airportal".to_owned(), "minecraft:end_portal".to_owned());
        map.insert("Banner".to_owned(), "minecraft:banner".to_owned());
        map.insert("Beacon".to_owned(), "minecraft:beacon".to_owned());
        map.insert("Cauldron".to_owned(), "minecraft:brewing_stand".to_owned());
        map.insert("Chest".to_owned(), "minecraft:chest".to_owned());
        map.insert("Comparator".to_owned(), "minecraft:comparator".to_owned());
        map.insert("Control".to_owned(), "minecraft:command_block".to_owned());
        map.insert("DLDetector".to_owned(), "minecraft:daylight_detector".to_owned());
        map.insert("Dropper".to_owned(), "minecraft:dropper".to_owned());
        map.insert("EnchantTable".to_owned(), "minecraft:enchanting_table".to_owned());
        map.insert("EndGateway".to_owned(), "minecraft:end_gateway".to_owned());
        map.insert("EnderChest".to_owned(), "minecraft:ender_chest".to_owned());
        map.insert("FlowerPot".to_owned(), "minecraft:flower_pot".to_owned());
        map.insert("Furnace".to_owned(), "minecraft:furnace".to_owned());
        map.insert("Hopper".to_owned(), "minecraft:hopper".to_owned());
        map.insert("MobSpawner".to_owned(), "minecraft:mob_spawner".to_owned());
        map.insert("Music".to_owned(), "minecraft:noteblock".to_owned());
        map.insert("Piston".to_owned(), "minecraft:piston".to_owned());
        map.insert("RecordPlayer".to_owned(), "minecraft:jukebox".to_owned());
        map.insert("Sign".to_owned(), "minecraft:sign".to_owned());
        map.insert("Skull".to_owned(), "minecraft:skull".to_owned());
        map.insert("Structure".to_owned(), "minecraft:structure_block".to_owned());
        map.insert("Trap".to_owned(), "minecraft:dispenser".to_owned());
        map
    })
}

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    types.tile_entity.borrow_mut().add_structure_converter(VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        if let Some(id) = data.get_string("id") {
            if let Some(new_id) = tile_id_update().get(id) {
                let id = id.to_owned();
                data.set(id, T::Object::create_string(new_id.clone()));
            }
        }
    }));

    register_inventory(types, "minecraft:furnace");
    register_inventory(types, "minecraft:chest");
    types.tile_entity.borrow_mut().add_walker_for_id(VERSION, "minecraft:jukebox", DataWalkerMapTypePaths::new(types.item_stack, "RecordItem"));
    register_inventory(types, "minecraft:dispenser");
    register_inventory(types, "minecraft:dropper");
    let untagged_spawner_type = types.untagged_spawner;
    types.tile_entity.borrow_mut().add_walker_for_id(VERSION, "minecraft:mob_spawner", data_walker::<T, _>(move |data, from_version, to_version| {
        untagged_spawner_type.convert(data, from_version, to_version);
    }));
    register_inventory(types, "minecraft:brewing_stand");
    register_inventory(types, "minecraft:hopper");
    types.tile_entity.borrow_mut().add_walker_for_id(VERSION, "minecraft:flower_pot", DataWalkerObjectTypePaths::new(types.item_name, "Item"));

    let block_name_type = types.block_name;
    let entity_type = types.entity;
    let item_name_type = types.item_name;
    let item_stack_type = types.item_stack;
    let tile_entity_type = types.tile_entity;
    types.item_stack.borrow_mut().add_structure_walker(VERSION, data_walker::<T, _>(move |data, from_version, to_version| {
        convert_object_in_map::<_, T>(item_name_type, data, "id", from_version, to_version);
        let [item_id, tag] = data.get_mut_multi(["id", "tag"]);

        let tag = match tag.and_then(|v| v.as_map_mut()) {
            Some(tag) => tag,
            None => return
        };

        // only things here are in tag, if changed update if above

        convert_map_list_in_map::<_, T>(item_stack_type, tag, "Items", from_version, to_version);

        if let Some(entity_tag) = tag.get_map_mut("EntityTag") {
            let item_id_str = item_id.as_ref().and_then(|o| o.as_string());
            let entity_id = match item_id_str {
                // The check for version id is removed here. For whatever reason, the legacy
                // data converters used entity id "minecraft:armor_stand" when version was greater-than 514,
                // but entity ids were not namespaced until V705! So somebody fucked up the legacy converters.
                // DFU agrees with my analysis here, it will only set the entityId here to the namespaced variant
                // with the V705 schema.
                Some("minecraft:armor_stand") => Some(if from_version.get_version() < 705 {"ArmorStand"} else {"minecraft:armor_stand"}),
                // add missing item_frame entity id
                // version check is same for armorstand, as both were namespaced at the same time
                Some("minecraft:item_frame") => Some(if from_version.get_version() < 705 {"ItemFrame"} else {"minecraft:item_frame"}),
                // add missing glow_item_frame entity id
                Some("minecraft:glow_item_frame") => Some("minecraft:glow_item_frame"),
                // V1451 changes spawn eggs to have the sub entity id be a part of the item id, but of course Mojang never
                // bothered to write in logic to set the sub entity id, so we have to.
                // format is ALWAYS <namespace>:<id>_spawn_egg post flattening
                Some(item_id_str) => item_id_str.strip_suffix("_spawn_egg").or_else(|| entity_tag.get_string("id")),
                None => entity_tag.get_string("id")
            };

            let remove_id = if let Some(entity_id) = entity_id {
                let remove_id = entity_tag.get_string("id").is_none();
                if remove_id {
                    let new_id = entity_id.to_owned();
                    entity_tag.set("id", T::Object::create_string(new_id));
                }
                remove_id
            } else {
                if item_id_str != Some("minecraft:air") {
                    warn!("Unable to resolve Entity for ItemStack (V704): {:?}", item_id);
                }
                false
            };

            entity_type.convert(entity_tag, from_version, to_version);

            if remove_id {
                entity_tag.remove("id");
            }
        }

        if let Some(block_entity_tag) = tag.get_map_mut("BlockEntityTag") {
            let item_id_str = item_id.as_ref().and_then(|o| o.as_string());
            let entity_id = item_id_str.and_then(|id| item_id_to_tile_entity_id().get(id));

            let remove_id = if let Some(entity_id) = entity_id {
                let remove_id = block_entity_tag.get_string("id").is_none();
                block_entity_tag.set("id", T::Object::create_string(entity_id.to_owned()));
                remove_id
            } else {
                if item_id_str != Some("minecraft:air") {
                    warn!("Unable to resolve BlockEntity for ItemStack (V704): {:?}", item_id);
                }
                false
            };

            tile_entity_type.convert(block_entity_tag, from_version, to_version);

            if remove_id {
                block_entity_tag.remove("id");
            }
        }

        convert_object_list_in_map::<_, T>(block_name_type, tag, "CanDestroy", from_version, to_version);
        convert_object_list_in_map::<_, T>(block_name_type, tag, "CanPlaceOn", from_version, to_version);
    }));

    // Enforce namespace for ids
    types.tile_entity.borrow_mut().add_structure_hook(VERSION, DataHookEnforceNamespacedId::<T>::id());
}

fn register_inventory<T: Types + ?Sized>(types: &MinecraftTypesMut<T>, id: impl Into<String>) {
    types.tile_entity.borrow_mut().add_walker_for_id(VERSION, id, DataWalkerMapListPaths::new(types.item_stack, "Items"));
}

#[cfg(test)]
mod test {
    #[test]
    #[cfg(feature = "update_checks")]
    fn test_in_sync_with_paper() {
        use std::io::{BufRead, BufReader};
        use crate::versions::v704::item_id_to_tile_entity_id;

        let regex = regex::Regex::new(r#"\s*ITEM_ID_TO_TILE_ENTITY_ID\s*\.\s*put\s*\("([\w:]*)"\s*,\s*"([\w:]*)"\s*\)\s*;\s*"#).unwrap();

        const URL: &str = "https://raw.githubusercontent.com/PaperMC/DataConverter/master/src/main/java/ca/spottedleaf/dataconverter/minecraft/versions/V704.java";
        let response = attohttpc::get(URL).send().expect("Failed to download V704.java");
        let mut paper_map = rust_dataconverter_engine::Map::new();
        for line in BufReader::new(response).lines() {
            let line = line.expect("Failed to download V704.java");
            if let Some(captures) = regex.captures(&line) {
                paper_map.insert(captures.get(1).unwrap().as_str().to_owned(), captures.get(2).unwrap().as_str().to_owned());
            }
        }

        let our_map = item_id_to_tile_entity_id();

        let mut missing_from_ours = Vec::new();
        let mut missing_from_paper = Vec::new();
        let mut different = Vec::new();
        for (our_key, our_val) in our_map {
            match paper_map.get(our_key) {
                Some(paper_val) => if paper_val != our_val {
                    different.push(our_key);
                }
                None => missing_from_paper.push(our_key)
            }
        }
        for paper_key in paper_map.keys() {
            if !our_map.contains_key(paper_key) {
                missing_from_ours.push(paper_key);
            }
        }

        if !missing_from_ours.is_empty() || !missing_from_paper.is_empty() || !different.is_empty() {
            let mut error = String::new();
            if !missing_from_ours.is_empty() {
                error.push_str(format!("Paper has {} entries missing from ours:\n", missing_from_ours.len()).as_str());
                for elem in missing_from_ours {
                    error.push_str(format!("- {}\n", elem).as_str());
                }
            }
            if !missing_from_paper.is_empty() {
                error.push_str(format!("We have {} extra entries that are absent in Paper:\n", missing_from_paper.len()).as_str());
                for elem in missing_from_paper {
                    error.push_str(format!("- {}\n", elem).as_str());
                }
            }
            if !different.is_empty() {
                error.push_str(format!("We have {} entries that are different from Paper:\n", different.len()).as_str());
                for elem in different {
                    error.push_str(format!("- {}: {} (ours) vs {} (paper)\n", elem, our_map.get(elem).unwrap(), paper_map.get(elem).unwrap()).as_str());
                }
            }
            if error.ends_with('\n') {
                error.remove(error.len() - 1);
            }

            assert!(false, "{}", error);
        }
    }
}
