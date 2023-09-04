use crate::helpers::hooks::{DataHookEnforceNamespacedId, DataHookValueTypeEnforceNamespaced};
use crate::helpers::mc_namespace_map::McNamespaceMap;
use crate::types;
use log::warn;
use std::sync::OnceLock;
use valence_nbt::{List, Value};
use world_transmuter_engine::*;

const VERSION: u32 = 99;

static ITEM_ID_TO_TILE_ENTITY_ID: OnceLock<McNamespaceMap<&'static str>> = OnceLock::new();

fn item_id_to_tile_entity_id() -> &'static McNamespaceMap<'static, &'static str> {
    ITEM_ID_TO_TILE_ENTITY_ID.get_or_init(|| {
        let mut map = McNamespaceMap::new();
        map.insert_mc("furnace", "Furnace");
        map.insert_mc("lit_furnace", "Furnace");
        map.insert_mc("chest", "Chest");
        map.insert_mc("trapped_chest", "Chest");
        map.insert_mc("ender_chest", "EnderChest");
        map.insert_mc("jukebox", "RecordPlayer");
        map.insert_mc("dispenser", "Trap");
        map.insert_mc("dropper", "Dropper");
        map.insert_mc("sign", "Sign");
        map.insert_mc("mob_spawner", "MobSpawner");
        map.insert_mc("noteblock", "Music");
        map.insert_mc("brewing_stand", "Cauldron");
        map.insert_mc("enhanting_table", "EnchantTable");
        map.insert_mc("command_block", "CommandBlock");
        map.insert_mc("beacon", "Beacon");
        map.insert_mc("skull", "Skull");
        map.insert_mc("daylight_detector", "DLDetector");
        map.insert_mc("hopper", "Hopper");
        map.insert_mc("banner", "Banner");
        map.insert_mc("flower_pot", "FlowerPot");
        map.insert_mc("repeating_command_block", "CommandBlock");
        map.insert_mc("chain_command_block", "CommandBlock");
        map.insert_mc("standing_sign", "Sign");
        map.insert_mc("wall_sign", "Sign");
        map.insert_mc("piston_head", "Piston");
        map.insert_mc("daylight_detector_inverted", "DLDetector");
        map.insert_mc("unpowered_comparator", "Comparator");
        map.insert_mc("powered_comparator", "Comparator");
        map.insert_mc("wall_banner", "Banner");
        map.insert_mc("standing_banner", "Banner");
        map.insert_mc("structure_block", "Structure");
        map.insert_mc("end_portal", "Airportal");
        map.insert_mc("end_gateway", "EndGateway");
        map.insert_mc("shield", "Banner");
        map
    })
}

pub(crate) fn register() {
    // entities
    types::entity_mut().add_structure_walker(
        VERSION,
        DataWalkerMapTypePaths::new(types::entity_ref(), "Riding"),
    );
    types::entity_mut().add_walker_for_id(
        VERSION,
        "Item",
        DataWalkerMapTypePaths::new(types::item_stack_ref(), "Item"),
    );
    register_projectile("ThrownEgg");
    register_projectile("Arrow");
    register_projectile("TippedArrow");
    register_projectile("SpectralArrow");
    register_projectile("Snowball");
    register_projectile("Fireball");
    register_projectile("SmallFireball");
    register_projectile("ThrownEnderpearl");
    register_projectile("ThrownPotion");
    types::entity_mut().add_walker_for_id(
        VERSION,
        "ThrownPotion",
        DataWalkerMapTypePaths::new(types::item_stack_ref(), "Potion"),
    );
    register_projectile("ThrownExpBottle");
    types::entity_mut().add_walker_for_id(
        VERSION,
        "ItemFrame",
        DataWalkerMapTypePaths::new(types::item_stack_ref(), "Item"),
    );
    register_projectile("WitherSkull");
    types::entity_mut().add_walker_for_id(
        VERSION,
        "FallingSand",
        DataWalkerObjectTypePaths::new(types::block_name_ref(), "Block"),
    );
    types::entity_mut().add_walker_for_id(
        VERSION,
        "FallingSand",
        DataWalkerMapTypePaths::new(types::tile_entity_ref(), "BlockEntityData"),
    );
    types::entity_mut().add_walker_for_id(
        VERSION,
        "FireworksRocketEntity",
        DataWalkerMapTypePaths::new(types::item_stack_ref(), "FireworksItem"),
    );
    // Note: Minecart is the generic entity. It can be subtyped via an int to become one of the specific minecarts
    // (i.e rideable, chest, furnace, tnt, etc)
    // Because of this, we add all walkers to the generic type, even though they might not be needed.
    // Vanilla does not make the generic minecart convert spawners, but we do.

    // for all minecart types:
    for minecart_type in [
        "Minecart",
        "MinecartRideable",
        "MinecartChest",
        "MinecartFurnace",
        "MinecartTNT",
        "MinecartSpawner",
        "MinecartHopper",
        "MinecartCommandBlock",
    ] {
        types::entity_mut().add_walker_for_id(
            VERSION,
            minecart_type,
            DataWalkerObjectTypePaths::new(types::block_name_ref(), "DisplayTile"),
        );
    }
    // for chest types:
    for minecart_type in ["Minecart", "MinecartChest", "MinecartHopper"] {
        types::entity_mut().add_walker_for_id(
            VERSION,
            minecart_type,
            DataWalkerMapListPaths::new(types::item_stack_ref(), "Items"),
        );
    }
    // for spawner type:
    for minecart_type in ["Minecart", "MinecartSpawner"] {
        types::entity_mut().add_walker_for_id(
            VERSION,
            minecart_type,
            data_walker(move |data, from_version, to_version| {
                types::untagged_spawner().convert(data, from_version, to_version);
            }),
        );
    }
    register_mob("ArmorStand");
    register_mob("Creeper");
    register_mob("Skeleton");
    register_mob("Spider");
    register_mob("Giant");
    register_mob("Zombie");
    register_mob("Slime");
    register_mob("Ghast");
    register_mob("PigZombie");
    register_mob("Enderman");
    types::entity_mut().add_walker_for_id(
        VERSION,
        "Enderman",
        DataWalkerObjectTypePaths::new(types::block_name_ref(), "carried"),
    );
    register_mob("CaveSpider");
    register_mob("Silverfish");
    register_mob("Blaze");
    register_mob("LavaSlime");
    register_mob("EnderDragon");
    register_mob("WitherBoss");
    register_mob("Bat");
    register_mob("Witch");
    register_mob("Endermite");
    register_mob("Guardian");
    register_mob("Pig");
    register_mob("Sheep");
    register_mob("Cow");
    register_mob("Chicken");
    register_mob("Squid");
    register_mob("Wolf");
    register_mob("MushroomCow");
    register_mob("SnowMan");
    register_mob("Ozelot");
    register_mob("VillagerGolem");
    types::entity_mut().add_walker_for_id(
        VERSION,
        "EntityHorse",
        DataWalkerMapListPaths::new_multi(
            types::item_stack_ref(),
            vec!["Items".to_owned(), "Equipment".to_owned()],
        ),
    );
    types::entity_mut().add_walker_for_id(
        VERSION,
        "EntityHorse",
        DataWalkerMapTypePaths::new_multi(
            types::item_stack_ref(),
            vec!["ArmorItem".to_owned(), "SaddleItem".to_owned()],
        ),
    );
    register_mob("Rabbit");
    types::entity_mut().add_walker_for_id(
        VERSION,
        "Villager",
        DataWalkerMapListPaths::new_multi(
            types::item_stack_ref(),
            vec!["Inventory".to_owned(), "Equipment".to_owned()],
        ),
    );
    types::entity_mut().add_walker_for_id(
        VERSION,
        "Villager",
        data_walker(move |data, from_version, to_version| {
            if let Some(Value::Compound(offers)) = data.get_mut("Offers") {
                if let Some(Value::List(List::Compound(recipes))) = offers.get_mut("Recipes") {
                    for recipe in recipes {
                        convert_map_in_map(
                            types::item_stack_ref(),
                            recipe,
                            "buy",
                            from_version,
                            to_version,
                        );
                        convert_map_in_map(
                            types::item_stack_ref(),
                            recipe,
                            "buyB",
                            from_version,
                            to_version,
                        );
                        convert_map_in_map(
                            types::item_stack_ref(),
                            recipe,
                            "sell",
                            from_version,
                            to_version,
                        );
                    }
                }
            }
        }),
    );
    register_mob("Shulker");

    // tile entities

    // inventories
    register_inventory("Furnace");
    register_inventory("Chest");
    types::tile_entity_mut().add_walker_for_id(
        VERSION,
        "RecordPlayer",
        DataWalkerMapTypePaths::new(types::item_stack_ref(), "RecordItem"),
    );
    register_inventory("Trap");
    register_inventory("Dropper");
    types::tile_entity_mut().add_walker_for_id(
        VERSION,
        "MobSpawner",
        data_walker(move |data, from_version, to_version| {
            types::untagged_spawner().convert(data, from_version, to_version);
        }),
    );
    register_inventory("Cauldron");
    register_inventory("Hopper");
    // Note: Vanilla does not properly handle this case, it will not convert int ids!
    types::tile_entity_mut().add_walker_for_id(
        VERSION,
        "FlowerPot",
        DataWalkerObjectTypePaths::new(types::item_name_ref(), "Item"),
    );

    // rest

    types::item_stack_mut().add_structure_walker(
        VERSION,
        data_walker(move |data, from_version, to_version| {
            convert_object_in_map(types::item_name_ref(), data, "id", from_version, to_version);
            let [item_id, tag] = get_mut_multi(data, ["id", "tag"]);
            let item_id = item_id.map(|v| &*v);

            let Some(Value::Compound(tag)) = tag else {
                return;
            };

            // only things here are in tag, if changed update if above

            convert_map_list_in_map(
                types::item_stack_ref(),
                tag,
                "Items",
                from_version,
                to_version,
            );

            if let Some(Value::Compound(entity_tag)) = tag.get_mut("EntityTag") {
                let item_id_str = item_id.and_then(get_string_id);
                let entity_id = match item_id_str {
                    // The check for version id is removed here. For whatever reason, the legacy
                    // data converters used entity id "minecraft:armor_stand" when version was greater-than 514,
                    // but entity ids were not namespaced until V705! So somebody fucked up the legacy converters.
                    // DFU agrees with my analysis here, it will only set the entityId here to the namespaced variant
                    // with the V705 schema.
                    Some("minecraft:armor_stand") => Some("ArmorStand"),
                    // add missing item_frame entity id
                    Some("minecraft:item_frame") => Some("ItemFrame"),
                    _ => match entity_tag.get("id") {
                        Some(Value::String(id)) => Some(&id[..]),
                        _ => None,
                    },
                };

                let remove_id = if let Some(entity_id) = entity_id {
                    let remove_id = !matches!(entity_tag.get("id"), Some(Value::String(_)));
                    if remove_id {
                        let new_id = entity_id.to_owned();
                        entity_tag.insert("id", new_id);
                    }
                    remove_id
                } else {
                    if item_id_str != Some("minecraft:air") {
                        warn!(
                            "Unable to resolve Entity for ItemStack (V99): {:?}",
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

            if let Some(Value::Compound(block_entity_tag)) = tag.get_mut("BlockEntityTag") {
                let item_id_str = item_id.as_ref().and_then(|v| get_string_id(v));
                let entity_id =
                    item_id_str.and_then(|id| item_id_to_tile_entity_id().get(id).copied());

                let remove_id = if let Some(entity_id) = entity_id {
                    let remove_id = !matches!(block_entity_tag.get("id"), Some(Value::String(_)));
                    block_entity_tag.insert("id", entity_id);
                    remove_id
                } else {
                    if item_id_str != Some("minecraft:air") {
                        warn!(
                            "Unable to resolve BlockEntity for ItemStack (V99): {:?}",
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

    types::player_mut().add_structure_walker(
        VERSION,
        DataWalkerMapListPaths::new_multi(
            types::item_stack_ref(),
            vec!["Inventory".to_owned(), "EnderItems".to_owned()],
        ),
    );

    types::chunk_mut().add_structure_walker(
        VERSION,
        data_walker(move |data, from_version, to_version| {
            let Some(Value::Compound(level)) = data.get_mut("Level") else {
                return;
            };
            convert_map_list_in_map(
                types::entity_ref(),
                level,
                "Entities",
                from_version,
                to_version,
            );
            convert_map_list_in_map(
                types::tile_entity_ref(),
                level,
                "TileEntities",
                from_version,
                to_version,
            );

            if let Some(Value::List(List::Compound(tile_ticks))) = level.get_mut("TileTicks") {
                for tile_tick in tile_ticks {
                    convert_object_in_map(
                        types::block_name_ref(),
                        tile_tick,
                        "i",
                        from_version,
                        to_version,
                    );
                }
            }
        }),
    );

    types::entity_chunk_mut().add_structure_walker(
        VERSION,
        DataWalkerMapListPaths::new(types::entity_ref(), "Entities"),
    );

    types::saved_data_mut().add_structure_walker(
        VERSION,
        data_walker(move |data, from_version, to_version| {
            let Some(Value::Compound(data)) = data.get_mut("data") else {
                return;
            };

            convert_values_in_map(
                types::structure_feature_ref(),
                data,
                "Features",
                from_version,
                to_version,
            );
            convert_map_list_in_map(
                types::objective_ref(),
                data,
                "Objectives",
                from_version,
                to_version,
            );
            convert_map_list_in_map(types::team_ref(), data, "Teams", from_version, to_version);
        }),
    );

    types::block_name_mut().add_structure_hook(VERSION, DataHookValueTypeEnforceNamespaced);
    types::item_name_mut().add_structure_hook(VERSION, DataHookValueTypeEnforceNamespaced);
    types::item_stack_mut().add_structure_hook(VERSION, DataHookEnforceNamespacedId::id());
}

fn register_mob(id: impl Into<String>) {
    types::entity_mut().add_walker_for_id(
        VERSION,
        id,
        DataWalkerMapListPaths::new(types::item_stack_ref(), "Equipment"),
    );
}

fn register_projectile(id: impl Into<String>) {
    types::entity_mut().add_walker_for_id(
        VERSION,
        id,
        DataWalkerObjectTypePaths::new(types::block_name_ref(), "inTile"),
    );
}

fn register_inventory(id: impl Into<String>) {
    types::tile_entity_mut().add_walker_for_id(
        VERSION,
        id,
        DataWalkerMapListPaths::new(types::item_stack_ref(), "Items"),
    );
}

fn get_string_id(value: &Value) -> Option<&str> {
    if let Value::String(str) = value {
        Some(str)
    } else {
        value
            .as_i32()
            .and_then(crate::helpers::item_name_v102::get_name_from_id)
    }
}
