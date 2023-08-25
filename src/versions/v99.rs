use crate::helpers::hooks::{DataHookEnforceNamespacedId, DataHookValueTypeEnforceNamespaced};
use crate::helpers::mc_namespace_map::McNamespaceMap;
use crate::MinecraftTypesMut;
use log::warn;
use rust_dataconverter_engine::*;
use std::sync::OnceLock;
use valence_nbt::{List, Value};

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

pub(crate) fn register(types: &MinecraftTypesMut) {
    // entities
    types
        .entity
        .borrow_mut()
        .add_structure_walker(VERSION, DataWalkerMapTypePaths::new(types.entity, "Riding"));
    types.entity.borrow_mut().add_walker_for_id(
        VERSION,
        "Item",
        DataWalkerMapTypePaths::new(types.item_stack, "Item"),
    );
    register_projectile(types, "ThrownEgg");
    register_projectile(types, "Arrow");
    register_projectile(types, "TippedArrow");
    register_projectile(types, "SpectralArrow");
    register_projectile(types, "Snowball");
    register_projectile(types, "Fireball");
    register_projectile(types, "SmallFireball");
    register_projectile(types, "ThrownEnderpearl");
    register_projectile(types, "ThrownPotion");
    types.entity.borrow_mut().add_walker_for_id(
        VERSION,
        "ThrownPotion",
        DataWalkerMapTypePaths::new(types.item_stack, "Potion"),
    );
    register_projectile(types, "ThrownExpBottle");
    types.entity.borrow_mut().add_walker_for_id(
        VERSION,
        "ItemFrame",
        DataWalkerMapTypePaths::new(types.item_stack, "Item"),
    );
    register_projectile(types, "WitherSkull");
    types.entity.borrow_mut().add_walker_for_id(
        VERSION,
        "FallingSand",
        DataWalkerObjectTypePaths::new(types.block_name, "Block"),
    );
    types.entity.borrow_mut().add_walker_for_id(
        VERSION,
        "FallingSand",
        DataWalkerMapTypePaths::new(types.tile_entity, "BlockEntityData"),
    );
    types.entity.borrow_mut().add_walker_for_id(
        VERSION,
        "FireworksRocketEntity",
        DataWalkerMapTypePaths::new(types.item_stack, "FireworksItem"),
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
        types.entity.borrow_mut().add_walker_for_id(
            VERSION,
            minecart_type,
            DataWalkerObjectTypePaths::new(types.block_name, "DisplayTile"),
        );
    }
    // for chest types:
    for minecart_type in ["Minecart", "MinecartChest", "MinecartHopper"] {
        types.entity.borrow_mut().add_walker_for_id(
            VERSION,
            minecart_type,
            DataWalkerMapListPaths::new(types.item_stack, "Items"),
        );
    }
    // for spawner type:
    for minecart_type in ["Minecart", "MinecartSpawner"] {
        let spawner_type = types.untagged_spawner;
        types.entity.borrow_mut().add_walker_for_id(
            VERSION,
            minecart_type,
            data_walker(move |data, from_version, to_version| {
                spawner_type.convert(data, from_version, to_version);
            }),
        );
    }
    register_mob(types, "ArmorStand");
    register_mob(types, "Creeper");
    register_mob(types, "Skeleton");
    register_mob(types, "Spider");
    register_mob(types, "Giant");
    register_mob(types, "Zombie");
    register_mob(types, "Slime");
    register_mob(types, "Ghast");
    register_mob(types, "PigZombie");
    register_mob(types, "Enderman");
    types.entity.borrow_mut().add_walker_for_id(
        VERSION,
        "Enderman",
        DataWalkerObjectTypePaths::new(types.block_name, "carried"),
    );
    register_mob(types, "CaveSpider");
    register_mob(types, "Silverfish");
    register_mob(types, "Blaze");
    register_mob(types, "LavaSlime");
    register_mob(types, "EnderDragon");
    register_mob(types, "WitherBoss");
    register_mob(types, "Bat");
    register_mob(types, "Witch");
    register_mob(types, "Endermite");
    register_mob(types, "Guardian");
    register_mob(types, "Pig");
    register_mob(types, "Sheep");
    register_mob(types, "Cow");
    register_mob(types, "Chicken");
    register_mob(types, "Squid");
    register_mob(types, "Wolf");
    register_mob(types, "MushroomCow");
    register_mob(types, "SnowMan");
    register_mob(types, "Ozelot");
    register_mob(types, "VillagerGolem");
    types.entity.borrow_mut().add_walker_for_id(
        VERSION,
        "EntityHorse",
        DataWalkerMapListPaths::new_multi(
            types.item_stack,
            vec!["Items".to_owned(), "Equipment".to_owned()],
        ),
    );
    types.entity.borrow_mut().add_walker_for_id(
        VERSION,
        "EntityHorse",
        DataWalkerMapTypePaths::new_multi(
            types.item_stack,
            vec!["ArmorItem".to_owned(), "SaddleItem".to_owned()],
        ),
    );
    register_mob(types, "Rabbit");
    types.entity.borrow_mut().add_walker_for_id(
        VERSION,
        "Villager",
        DataWalkerMapListPaths::new_multi(
            types.item_stack,
            vec!["Inventory".to_owned(), "Equipment".to_owned()],
        ),
    );
    let item_stack_type = types.item_stack;
    types.entity.borrow_mut().add_walker_for_id(
        VERSION,
        "Villager",
        data_walker(move |data, from_version, to_version| {
            if let Some(Value::Compound(offers)) = data.get_mut("Offers") {
                if let Some(Value::List(List::Compound(recipes))) = offers.get_mut("Recipes") {
                    for recipe in recipes {
                        convert_map_in_map(
                            item_stack_type,
                            recipe,
                            "buy",
                            from_version,
                            to_version,
                        );
                        convert_map_in_map(
                            item_stack_type,
                            recipe,
                            "buyB",
                            from_version,
                            to_version,
                        );
                        convert_map_in_map(
                            item_stack_type,
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
    register_mob(types, "Shulker");

    // tile entities

    // inventories
    register_inventory(types, "Furnace");
    register_inventory(types, "Chest");
    types.tile_entity.borrow_mut().add_walker_for_id(
        VERSION,
        "RecordPlayer",
        DataWalkerMapTypePaths::new(types.item_stack, "RecordItem"),
    );
    register_inventory(types, "Trap");
    register_inventory(types, "Dropper");
    let spawner_type = types.untagged_spawner;
    types.tile_entity.borrow_mut().add_walker_for_id(
        VERSION,
        "MobSpawner",
        data_walker(move |data, from_version, to_version| {
            spawner_type.convert(data, from_version, to_version);
        }),
    );
    register_inventory(types, "Cauldron");
    register_inventory(types, "Hopper");
    // Note: Vanilla does not properly handle this case, it will not convert int ids!
    types.tile_entity.borrow_mut().add_walker_for_id(
        VERSION,
        "FlowerPot",
        DataWalkerObjectTypePaths::new(types.item_name, "Item"),
    );

    // rest

    let block_name_type = types.block_name;
    let entity_type = types.entity;
    let item_name_type = types.item_name;
    let item_stack_type = types.item_stack;
    let tile_entity_type = types.tile_entity;
    types.item_stack.borrow_mut().add_structure_walker(
        VERSION,
        data_walker(move |data, from_version, to_version| {
            convert_object_in_map(item_name_type, data, "id", from_version, to_version);
            let [item_id, tag] = get_mut_multi(data, ["id", "tag"]);
            let item_id = item_id.map(|v| &*v);

            let Some(Value::Compound(tag)) = tag else {
                return;
            };

            // only things here are in tag, if changed update if above

            convert_map_list_in_map(item_stack_type, tag, "Items", from_version, to_version);

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

                entity_type.convert(entity_tag, from_version, to_version);

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

    types.player.borrow_mut().add_structure_walker(
        VERSION,
        DataWalkerMapListPaths::new_multi(
            types.item_stack,
            vec!["Inventory".to_owned(), "EnderItems".to_owned()],
        ),
    );

    let block_name_type = types.block_name;
    let entity_type = types.entity;
    let tile_entity_type = types.tile_entity;
    types.chunk.borrow_mut().add_structure_walker(
        VERSION,
        data_walker(move |data, from_version, to_version| {
            let Some(Value::Compound(level)) = data.get_mut("Level") else {
                return;
            };
            convert_map_list_in_map(entity_type, level, "Entities", from_version, to_version);
            convert_map_list_in_map(
                tile_entity_type,
                level,
                "TileEntities",
                from_version,
                to_version,
            );

            if let Some(Value::List(List::Compound(tile_ticks))) = level.get_mut("TileTicks") {
                for tile_tick in tile_ticks {
                    convert_object_in_map(
                        block_name_type,
                        tile_tick,
                        "i",
                        from_version,
                        to_version,
                    );
                }
            }
        }),
    );

    types.entity_chunk.borrow_mut().add_structure_walker(
        VERSION,
        DataWalkerMapListPaths::new(types.entity, "Entities"),
    );

    let objective_type = types.objective;
    let structure_feature_type = types.structure_feature;
    let team_type = types.team;
    types.saved_data.borrow_mut().add_structure_walker(
        VERSION,
        data_walker(move |data, from_version, to_version| {
            let Some(Value::Compound(data)) = data.get_mut("data") else {
                return;
            };

            convert_values_in_map(
                structure_feature_type,
                data,
                "Features",
                from_version,
                to_version,
            );
            convert_map_list_in_map(objective_type, data, "Objectives", from_version, to_version);
            convert_map_list_in_map(team_type, data, "Teams", from_version, to_version);
        }),
    );

    types
        .block_name
        .borrow_mut()
        .add_structure_hook(VERSION, DataHookValueTypeEnforceNamespaced);
    types
        .item_name
        .borrow_mut()
        .add_structure_hook(VERSION, DataHookValueTypeEnforceNamespaced);
    types
        .item_stack
        .borrow_mut()
        .add_structure_hook(VERSION, DataHookEnforceNamespacedId::id());
}

fn register_mob(types: &MinecraftTypesMut, id: impl Into<String>) {
    types.entity.borrow_mut().add_walker_for_id(
        VERSION,
        id,
        DataWalkerMapListPaths::new(types.item_stack, "Equipment"),
    );
}

fn register_projectile(types: &MinecraftTypesMut, id: impl Into<String>) {
    types.entity.borrow_mut().add_walker_for_id(
        VERSION,
        id,
        DataWalkerObjectTypePaths::new(types.block_name, "inTile"),
    );
}

fn register_inventory(types: &MinecraftTypesMut, id: impl Into<String>) {
    types.tile_entity.borrow_mut().add_walker_for_id(
        VERSION,
        id,
        DataWalkerMapListPaths::new(types.item_stack, "Items"),
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
