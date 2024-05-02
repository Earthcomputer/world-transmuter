use crate::helpers::hooks::{DataHookEnforceNamespacedId, DataHookValueTypeEnforceNamespaced};
use crate::{static_string_mc_map, types};
use java_string::{JavaStr, JavaString};
use tracing::warn;
use world_transmuter_engine::*;

const VERSION: u32 = 99;

static_string_mc_map! {
    ITEM_ID_TO_TILE_ENTITY_ID, item_id_to_tile_entity_id, {
        "furnace" => "Furnace",
        "lit_furnace" => "Furnace",
        "chest" => "Chest",
        "trapped_chest" => "Chest",
        "ender_chest" => "EnderChest",
        "jukebox" => "RecordPlayer",
        "dispenser" => "Trap",
        "dropper" => "Dropper",
        "sign" => "Sign",
        "mob_spawner" => "MobSpawner",
        "noteblock" => "Music",
        "brewing_stand" => "Cauldron",
        "enhanting_table" => "EnchantTable",
        "command_block" => "CommandBlock",
        "beacon" => "Beacon",
        "skull" => "Skull",
        "daylight_detector" => "DLDetector",
        "hopper" => "Hopper",
        "banner" => "Banner",
        "flower_pot" => "FlowerPot",
        "repeating_command_block" => "CommandBlock",
        "chain_command_block" => "CommandBlock",
        "standing_sign" => "Sign",
        "wall_sign" => "Sign",
        "piston_head" => "Piston",
        "daylight_detector_inverted" => "DLDetector",
        "unpowered_comparator" => "Comparator",
        "powered_comparator" => "Comparator",
        "wall_banner" => "Banner",
        "standing_banner" => "Banner",
        "structure_block" => "Structure",
        "end_portal" => "Airportal",
        "end_gateway" => "EndGateway",
        "shield" => "Banner",
    }
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
            map_data_walker(move |data, from_version, to_version| {
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
        map_data_walker(move |data, from_version, to_version| {
            if let Some(JValue::Compound(offers)) = data.get_mut("Offers") {
                convert_map_list_in_map(
                    types::villager_trade_ref(),
                    offers,
                    "Recipes",
                    from_version,
                    to_version,
                );
            }
        }),
    );
    register_mob("Shulker");
    types::entity_mut().add_walker_for_id(
        VERSION,
        "AreaEffectCloud",
        DataWalkerDynamicTypePaths::new(types::particle_ref(), "Particle"),
    );

    // tile entities
    types::tile_entity_mut().add_structure_walker(
        VERSION,
        map_data_walker(move |data, from_version, to_version| {
            convert_map_in_map(
                types::data_components_ref(),
                data,
                "components",
                from_version,
                to_version,
            );
        }),
    );

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
        map_data_walker(move |data, from_version, to_version| {
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
        map_data_walker(move |data, from_version, to_version| {
            convert_object_in_map(types::item_name_ref(), data, "id", from_version, to_version);
            let [item_id, tag] = get_mut_multi(data, ["id", "tag"]);
            let item_id = item_id.map(|v| &*v);

            let Some(JValue::Compound(tag)) = tag else {
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
            convert_map_list_in_map(
                types::item_stack_ref(),
                tag,
                "ChargedProjectiles",
                from_version,
                to_version,
            );

            if let Some(JValue::Compound(entity_tag)) = tag.get_mut("EntityTag") {
                let item_id_str = item_id.and_then(get_string_id);
                let entity_id = match item_id_str.map(JavaStr::as_bytes) {
                    // The check for version id is removed here. For whatever reason, the legacy
                    // data converters used entity id "minecraft:armor_stand" when version was greater-than 514,
                    // but entity ids were not namespaced until V705! So somebody fucked up the legacy converters.
                    // DFU agrees with my analysis here, it will only set the entityId here to the namespaced variant
                    // with the V705 schema.
                    Some(b"minecraft:armor_stand") => Some(JavaStr::from_str("ArmorStand")),
                    // add missing item_frame entity id
                    Some(b"minecraft:item_frame") => Some(JavaStr::from_str("ItemFrame")),
                    Some(b"minecraft:painting") => Some(JavaStr::from_str("Painting")),
                    _ => match entity_tag.get("id") {
                        Some(JValue::String(id)) => Some(&id[..]),
                        _ => None,
                    },
                };

                let remove_id = if let Some(entity_id) = entity_id {
                    let remove_id = !matches!(entity_tag.get("id"), Some(JValue::String(_)));
                    if remove_id {
                        let new_id = entity_id.to_owned();
                        entity_tag.insert("id", new_id);
                    }
                    remove_id
                } else {
                    if item_id_str != Some(JavaStr::from_str("minecraft:air")) {
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

            if let Some(JValue::Compound(block_entity_tag)) = tag.get_mut("BlockEntityTag") {
                let item_id_str = item_id.as_ref().and_then(|v| get_string_id(v));
                let entity_id =
                    item_id_str.and_then(|id| item_id_to_tile_entity_id().get(id).copied());

                let remove_id = if let Some(entity_id) = entity_id {
                    let remove_id = !matches!(block_entity_tag.get("id"), Some(JValue::String(_)));
                    block_entity_tag.insert("id", entity_id);
                    remove_id
                } else {
                    if item_id_str != Some(JavaStr::from_str("minecraft:air")) {
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
        map_data_walker(move |data, from_version, to_version| {
            let Some(JValue::Compound(level)) = data.get_mut("Level") else {
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

            if let Some(JValue::List(JList::Compound(tile_ticks))) = level.get_mut("TileTicks") {
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

    types::saved_data_scoreboard_mut().add_structure_walker(
        VERSION,
        map_data_walker(move |data, from_version, to_version| {
            let Some(JValue::Compound(data)) = data.get_mut("data") else {
                return;
            };

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
    types::saved_data_structure_feature_indices_mut().add_structure_walker(
        VERSION,
        map_data_walker(move |data, from_version, to_version| {
            let Some(JValue::Compound(data)) = data.get_mut("data") else {
                return;
            };
            convert_values_in_map(
                types::structure_feature_ref(),
                data,
                "Features",
                from_version,
                to_version,
            );
        }),
    );

    types::villager_trade_mut().add_structure_walker(
        VERSION,
        map_data_walker(move |data, from_version, to_version| {
            convert_map_in_map(
                types::item_stack_ref(),
                data,
                "buy",
                from_version,
                to_version,
            );
            convert_map_in_map(
                types::item_stack_ref(),
                data,
                "buyB",
                from_version,
                to_version,
            );
            convert_map_in_map(
                types::item_stack_ref(),
                data,
                "sell",
                from_version,
                to_version,
            );
        }),
    );

    types::block_name_mut().add_structure_hook(VERSION, DataHookValueTypeEnforceNamespaced);
    types::item_name_mut().add_structure_hook(VERSION, DataHookValueTypeEnforceNamespaced);
    types::item_stack_mut().add_structure_hook(VERSION, DataHookEnforceNamespacedId::id());
}

fn register_mob(id: impl Into<JavaString>) {
    types::entity_mut().add_walker_for_id(
        VERSION,
        id,
        DataWalkerMapListPaths::new(types::item_stack_ref(), "Equipment"),
    );
}

fn register_projectile(id: impl Into<JavaString>) {
    types::entity_mut().add_walker_for_id(
        VERSION,
        id,
        DataWalkerObjectTypePaths::new(types::block_name_ref(), "inTile"),
    );
}

fn register_inventory(id: impl Into<JavaString>) {
    types::tile_entity_mut().add_walker_for_id(
        VERSION,
        id,
        DataWalkerMapListPaths::new(types::item_stack_ref(), "Items"),
    );
}

fn get_string_id(value: &JValue) -> Option<&JavaStr> {
    if let JValue::String(str) = value {
        Some(str)
    } else {
        value
            .as_i32()
            .and_then(crate::helpers::item_name_v102::get_name_from_id)
    }
}
