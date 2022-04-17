use std::lazy::SyncOnceCell;
use log::warn;
use rust_dataconverter_engine::*;
use crate::helpers::hooks::{DataHookEnforceNamespacedId, DataHookValueTypeEnforceNamespaced};
use crate::MinecraftTypesMut;

const VERSION: u32 = 99;

static ITEM_ID_TO_TILE_ENTITY_ID: SyncOnceCell<Map<String, String>> = SyncOnceCell::new();

fn item_id_to_tile_entity_id() -> &'static Map<String, String> {
    ITEM_ID_TO_TILE_ENTITY_ID.get_or_init(|| {
        let mut map = Map::new();
        map.insert("minecraft:furnace".to_owned(), "Furnace".to_owned());
        map.insert("minecraft:lit_furnace".to_owned(), "Furnace".to_owned());
        map.insert("minecraft:chest".to_owned(), "Chest".to_owned());
        map.insert("minecraft:trapped_chest".to_owned(), "Chest".to_owned());
        map.insert("minecraft:ender_chest".to_owned(), "EnderChest".to_owned());
        map.insert("minecraft:jukebox".to_owned(), "RecordPlayer".to_owned());
        map.insert("minecraft:dispenser".to_owned(), "Trap".to_owned());
        map.insert("minecraft:dropper".to_owned(), "Dropper".to_owned());
        map.insert("minecraft:sign".to_owned(), "Sign".to_owned());
        map.insert("minecraft:mob_spawner".to_owned(), "MobSpawner".to_owned());
        map.insert("minecraft:noteblock".to_owned(), "Music".to_owned());
        map.insert("minecraft:brewing_stand".to_owned(), "Cauldron".to_owned());
        map.insert("minecraft:enhanting_table".to_owned(), "EnchantTable".to_owned());
        map.insert("minecraft:command_block".to_owned(), "CommandBlock".to_owned());
        map.insert("minecraft:beacon".to_owned(), "Beacon".to_owned());
        map.insert("minecraft:skull".to_owned(), "Skull".to_owned());
        map.insert("minecraft:daylight_detector".to_owned(), "DLDetector".to_owned());
        map.insert("minecraft:hopper".to_owned(), "Hopper".to_owned());
        map.insert("minecraft:banner".to_owned(), "Banner".to_owned());
        map.insert("minecraft:flower_pot".to_owned(), "FlowerPot".to_owned());
        map.insert("minecraft:repeating_command_block".to_owned(), "CommandBlock".to_owned());
        map.insert("minecraft:chain_command_block".to_owned(), "CommandBlock".to_owned());
        map.insert("minecraft:standing_sign".to_owned(), "Sign".to_owned());
        map.insert("minecraft:wall_sign".to_owned(), "Sign".to_owned());
        map.insert("minecraft:piston_head".to_owned(), "Piston".to_owned());
        map.insert("minecraft:daylight_detector_inverted".to_owned(), "DLDetector".to_owned());
        map.insert("minecraft:unpowered_comparator".to_owned(), "Comparator".to_owned());
        map.insert("minecraft:powered_comparator".to_owned(), "Comparator".to_owned());
        map.insert("minecraft:wall_banner".to_owned(), "Banner".to_owned());
        map.insert("minecraft:standing_banner".to_owned(), "Banner".to_owned());
        map.insert("minecraft:structure_block".to_owned(), "Structure".to_owned());
        map.insert("minecraft:end_portal".to_owned(), "Airportal".to_owned());
        map.insert("minecraft:end_gateway".to_owned(), "EndGateway".to_owned());
        map.insert("minecraft:shield".to_owned(), "Banner".to_owned());
        map
    })
}

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    // entities
    types.entity.borrow_mut().add_structure_walker(VERSION, DataWalkerMapTypePaths::new(types.entity, "Riding"));
    types.entity.borrow_mut().add_walker_for_id(VERSION, "Item", DataWalkerMapTypePaths::new(types.item_stack, "Item"));
    register_projectile(types, "ThrownEgg");
    register_projectile(types, "Arrow");
    register_projectile(types, "TippedArrow");
    register_projectile(types, "SpectralArrow");
    register_projectile(types, "Snowball");
    register_projectile(types, "Fireball");
    register_projectile(types, "SmallFireball");
    register_projectile(types, "ThrownEnderpearl");
    register_projectile(types, "ThrownPotion");
    types.entity.borrow_mut().add_walker_for_id(VERSION, "ThrownPotion", DataWalkerMapTypePaths::new(types.item_stack, "Potion"));
    register_projectile(types, "ThrownExpBottle");
    types.entity.borrow_mut().add_walker_for_id(VERSION, "ItemFrame", DataWalkerMapTypePaths::new(types.item_stack, "Item"));
    register_projectile(types, "WitherSkull");
    types.entity.borrow_mut().add_walker_for_id(VERSION, "FallingSand", DataWalkerObjectTypePaths::new(types.block_name, "Block"));
    types.entity.borrow_mut().add_walker_for_id(VERSION, "FallingSand", DataWalkerMapTypePaths::new(types.tile_entity, "BlockEntityData"));
    types.entity.borrow_mut().add_walker_for_id(VERSION, "FireworksRocketEntity", DataWalkerMapTypePaths::new(types.item_stack, "FireworksItem"));
    // Note: Minecart is the generic entity. It can be subtyped via an int to become one of the specific minecarts
    // (i.e rideable, chest, furnace, tnt, etc)
    // Because of this, we add all walkers to the generic type, even though they might not be needed.
    // Vanilla does not make the generic minecart convert spawners, but we do.

    // for all minecart types:
    for minecart_type in ["Minecart", "MinecartRideable", "MinecartChest", "MinecartFurnace", "MinecartTNT", "MinecartSpawner", "MinecartHopper", "MinecartCommandBlock"] {
        types.entity.borrow_mut().add_walker_for_id(VERSION, minecart_type, DataWalkerObjectTypePaths::new(types.block_name, "DisplayTile"));
    }
    // for chest types:
    for minecart_type in ["Minecart", "MinecartChest", "MinecartHopper"] {
        types.entity.borrow_mut().add_walker_for_id(VERSION, minecart_type, DataWalkerMapListPaths::new(types.item_stack, "Items"));
    }
    // for spawner type:
    for minecart_type in ["Minecart", "MinecartSpawner"] {
        let spawner_type = types.untagged_spawner;
        types.entity.borrow_mut().add_walker_for_id(VERSION, minecart_type, data_walker::<T, _>(move |data, from_version, to_version| {
            spawner_type.convert(data, from_version, to_version);
        }));
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
    types.entity.borrow_mut().add_walker_for_id(VERSION, "Enderman", DataWalkerObjectTypePaths::new(types.block_name, "carried"));
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
    types.entity.borrow_mut().add_walker_for_id(VERSION, "EntityHorse", DataWalkerMapListPaths::new_multi(types.item_stack, vec!["Items".to_owned(), "Equipment".to_owned()]));
    types.entity.borrow_mut().add_walker_for_id(VERSION, "EntityHorse", DataWalkerMapTypePaths::new_multi(types.item_stack, vec!["ArmorItem".to_owned(), "SaddleItem".to_owned()]));
    register_mob(types, "Rabbit");
    types.entity.borrow_mut().add_walker_for_id(VERSION, "Villager", DataWalkerMapListPaths::new_multi(types.item_stack, vec!["Inventory".to_owned(), "Equipment".to_owned()]));
    let item_stack_type = types.item_stack;
    types.entity.borrow_mut().add_walker_for_id(VERSION, "Villager", data_walker::<T, _>(move |data, from_version, to_version| {
        if let Some(offers) = data.get_map_mut("Offers") {
            if let Some(recipes) = offers.get_list_mut("Recipes") {
                for recipe in recipes.iter_mut() {
                    if let Some(recipe_map) = recipe.as_map_mut() {
                        convert_map_in_map::<_, T>(item_stack_type, recipe_map, "buy", from_version, to_version);
                        convert_map_in_map::<_, T>(item_stack_type, recipe_map, "buyB", from_version, to_version);
                        convert_map_in_map::<_, T>(item_stack_type, recipe_map, "sell", from_version, to_version);
                    }
                }
            }
        }
    }));
    register_mob(types, "Shulker");

    // tile entities

    // inventories
    register_inventory(types, "Furnace");
    register_inventory(types, "Chest");
    types.tile_entity.borrow_mut().add_walker_for_id(VERSION, "RecordPlayer", DataWalkerMapTypePaths::new(types.item_stack, "RecordItem"));
    register_inventory(types, "Trap");
    register_inventory(types, "Dropper");
    let spawner_type = types.untagged_spawner;
    types.tile_entity.borrow_mut().add_walker_for_id(VERSION, "MobSpawner", data_walker::<T, _>(move |data, from_version, to_version| {
        spawner_type.convert(data, from_version, to_version);
    }));
    register_inventory(types, "Cauldron");
    register_inventory(types, "Hopper");
    // Note: Vanilla does not properly handle this case, it will not convert int ids!
    types.tile_entity.borrow_mut().add_walker_for_id(VERSION, "FlowerPot", DataWalkerObjectTypePaths::new(types.item_name, "Item"));

    // rest

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
            let item_id_str = item_id.as_ref().and_then(|v| get_string_id::<T>(v));
            let entity_id = match item_id_str {
                // The check for version id is removed here. For whatever reason, the legacy
                // data converters used entity id "minecraft:armor_stand" when version was greater-than 514,
                // but entity ids were not namespaced until V705! So somebody fucked up the legacy converters.
                // DFU agrees with my analysis here, it will only set the entityId here to the namespaced variant
                // with the V705 schema.
                Some("minecraft:armor_stand") => Some("ArmorStand"),
                // add missing item_frame entity id
                Some("minecraft:item_frame") => Some("ItemFrame"),
                _ => entity_tag.get_string("id")
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
                    warn!("Unable to resolve Entity for ItemStack (V99): {:?}", item_id);
                }
                false
            };

            entity_type.convert(entity_tag, from_version, to_version);

            if remove_id {
                entity_tag.remove("id");
            }
        }

        if let Some(block_entity_tag) = tag.get_map_mut("BlockEntityTag") {
            let item_id_str = item_id.as_ref().and_then(|v| get_string_id::<T>(v));
            let entity_id = item_id_str.and_then(|id| item_id_to_tile_entity_id().get(id));

            let remove_id = if let Some(entity_id) = entity_id {
                let remove_id = block_entity_tag.get_string("id").is_none();
                block_entity_tag.set("id", T::Object::create_string(entity_id.to_owned()));
                remove_id
            } else {
                if item_id_str != Some("minecraft:air") {
                    warn!("Unable to resolve BlockEntity for ItemStack (V99): {:?}", item_id);
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

    types.player.borrow_mut().add_structure_walker(VERSION, DataWalkerMapListPaths::new_multi(types.item_stack, vec!["Inventory".to_owned(), "EnderItems".to_owned()]));

    let block_name_type = types.block_name;
    let entity_type = types.entity;
    let tile_entity_type = types.tile_entity;
    types.chunk.borrow_mut().add_structure_walker(VERSION, data_walker::<T, _>(move |data, from_version, to_version| {
        let level = match data.get_map_mut("Level") {
            Some(level) => level,
            None => return
        };
        convert_map_list_in_map::<_, T>(entity_type, level, "Entities", from_version, to_version);
        convert_map_list_in_map::<_, T>(tile_entity_type, level, "TileEntities", from_version, to_version);

        if let Some(tile_ticks) = level.get_list_mut("TileTicks") {
            for tile_tick in tile_ticks.iter_mut() {
                if let Some(tile_tick) = tile_tick.as_map_mut() {
                    convert_object_in_map::<_, T>(block_name_type, tile_tick, "i", from_version, to_version);
                }
            }
        }
    }));

    types.entity_chunk.borrow_mut().add_structure_walker(VERSION, DataWalkerMapListPaths::new(types.entity, "Entities"));

    let objective_type = types.objective;
    let structure_feature_type = types.structure_feature;
    let team_type = types.team;
    types.saved_data.borrow_mut().add_structure_walker(VERSION, data_walker::<T, _>(move |data, from_version, to_version| {
        let data = match data.get_map_mut("data") {
            Some(data) => data,
            None => return
        };

        convert_values_in_map::<_, T>(structure_feature_type, data, "Features", from_version, to_version);
        convert_map_list_in_map::<_, T>(objective_type, data, "Objectives", from_version, to_version);
        convert_map_list_in_map::<_, T>(team_type, data, "Teams", from_version, to_version);
    }));

    types.block_name.borrow_mut().add_structure_hook(VERSION, DataHookValueTypeEnforceNamespaced::<T>::new());
    types.item_name.borrow_mut().add_structure_hook(VERSION, DataHookValueTypeEnforceNamespaced::<T>::new());
    types.item_stack.borrow_mut().add_structure_hook(VERSION, DataHookEnforceNamespacedId::<T>::id());
}

fn register_mob<T: Types + ?Sized>(types: &MinecraftTypesMut<T>, id: impl Into<String>) {
    types.entity.borrow_mut().add_walker_for_id(VERSION, id, DataWalkerMapListPaths::new(types.item_stack, "Equipment"));
}

fn register_projectile<T: Types + ?Sized>(types: &MinecraftTypesMut<T>, id: impl Into<String>) {
    types.entity.borrow_mut().add_walker_for_id(VERSION, id, DataWalkerObjectTypePaths::new(types.block_name, "inTile"));
}

fn register_inventory<T: Types + ?Sized>(types: &MinecraftTypesMut<T>, id: impl Into<String>) {
    types.tile_entity.borrow_mut().add_walker_for_id(VERSION, id, DataWalkerMapListPaths::new(types.item_stack, "Items"));
}

fn get_string_id<T: Types + ?Sized>(value: &T::Object) -> Option<&str> {
    value.as_string().or_else(|| value.as_i64().and_then(|i| crate::helpers::item_name_v102::get_name_from_id(i as i32)))
}
