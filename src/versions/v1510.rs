use std::lazy::SyncOnceCell;
use rust_dataconverter_engine::Types;
use crate::helpers::rename::{rename_block, rename_entity, rename_item, rename_recipe, rename_stat};
use crate::MinecraftTypesMut;

const VERSION: u32 = 1510;

static RENAMED_ENTITY_IDS: SyncOnceCell<rust_dataconverter_engine::Map<&'static str, &'static str>> = SyncOnceCell::new();

fn renamed_entity_ids() -> &'static rust_dataconverter_engine::Map<&'static str, &'static str> {
    RENAMED_ENTITY_IDS.get_or_init(|| {
        let mut map = rust_dataconverter_engine::Map::new();
        map.insert("minecraft:commandblock_minecart", "minecraft:command_block_minecart");
        map.insert("minecraft:ender_crystal", "minecraft:end_crystal");
        map.insert("minecraft:snowman", "minecraft:snow_golem");
        map.insert("minecraft:evocation_illager", "minecraft:evoker");
        map.insert("minecraft:evocation_fangs", "minecraft:evoker_fangs");
        map.insert("minecraft:illusion_illager", "minecraft:illusioner");
        map.insert("minecraft:vindication_illager", "minecraft:vindicator");
        map.insert("minecraft:villager_golem", "minecraft:iron_golem");
        map.insert("minecraft:xp_orb", "minecraft:experience_orb");
        map.insert("minecraft:xp_bottle", "minecraft:experience_bottle");
        map.insert("minecraft:eye_of_ender_signal", "minecraft:eye_of_ender");
        map.insert("minecraft:fireworks_rocket", "minecraft:firework_rocket");
        map
    })
}

static RENAMED_BLOCKS: SyncOnceCell<rust_dataconverter_engine::Map<&'static str, &'static str>> = SyncOnceCell::new();

fn renamed_blocks() -> &'static rust_dataconverter_engine::Map<&'static str, &'static str> {
    RENAMED_BLOCKS.get_or_init(|| {
        let mut map = rust_dataconverter_engine::Map::new();
        map.insert("minecraft:portal", "minecraft:nether_portal");
        map.insert("minecraft:oak_bark", "minecraft:oak_wood");
        map.insert("minecraft:spruce_bark", "minecraft:spruce_wood");
        map.insert("minecraft:birch_bark", "minecraft:birch_wood");
        map.insert("minecraft:jungle_bark", "minecraft:jungle_wood");
        map.insert("minecraft:acacia_bark", "minecraft:acacia_wood");
        map.insert("minecraft:dark_oak_bark", "minecraft:dark_oak_wood");
        map.insert("minecraft:stripped_oak_bark", "minecraft:stripped_oak_wood");
        map.insert("minecraft:stripped_spruce_bark", "minecraft:stripped_spruce_wood");
        map.insert("minecraft:stripped_birch_bark", "minecraft:stripped_birch_wood");
        map.insert("minecraft:stripped_jungle_bark", "minecraft:stripped_jungle_wood");
        map.insert("minecraft:stripped_acacia_bark", "minecraft:stripped_acacia_wood");
        map.insert("minecraft:stripped_dark_oak_bark", "minecraft:stripped_dark_oak_wood");
        map.insert("minecraft:mob_spawner", "minecraft:spawner");
        map
    })
}

static RENAMED_ITEMS: SyncOnceCell<rust_dataconverter_engine::Map<&'static str, &'static str>> = SyncOnceCell::new();

fn renamed_items() -> &'static rust_dataconverter_engine::Map<&'static str, &'static str> {
    RENAMED_ITEMS.get_or_init(|| {
        let mut map = rust_dataconverter_engine::Map::new();
        for (&k, &v) in renamed_blocks() {
            map.insert(k, v);
        }
        map.insert("minecraft:clownfish", "minecraft:tropical_fish");
        map.insert("minecraft:chorus_fruit_popped", "minecraft:popped_chorus_fruit");
        map.insert("minecraft:evocation_illager_spawn_egg", "minecraft:evoker_spawn_egg");
        map.insert("minecraft:vindication_illager_spawn_egg", "minecraft:vindicator_spawn_egg");
        map
    })
}

static RECIPES_UPDATES: SyncOnceCell<rust_dataconverter_engine::Map<&'static str, &'static str>> = SyncOnceCell::new();

fn recipes_updates() -> &'static rust_dataconverter_engine::Map<&'static str, &'static str> {
    RECIPES_UPDATES.get_or_init(|| {
        let mut map = rust_dataconverter_engine::Map::new();
        map.insert("minecraft:acacia_bark", "minecraft:acacia_wood");
        map.insert("minecraft:birch_bark", "minecraft:birch_wood");
        map.insert("minecraft:dark_oak_bark", "minecraft:dark_oak_wood");
        map.insert("minecraft:jungle_bark", "minecraft:jungle_wood");
        map.insert("minecraft:oak_bark", "minecraft:oak_wood");
        map.insert("minecraft:spruce_bark", "minecraft:spruce_wood");
        map
    })
}

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    rename_block(types, VERSION, |name| renamed_blocks().get(name).map(|&str| str.to_owned()));
    rename_item(types, VERSION, |name| renamed_items().get(name).map(|&str| str.to_owned()));
    rename_recipe(types, VERSION, |name| recipes_updates().get(name).map(|&str| str.to_owned()));
    rename_entity(types, VERSION, |name| {
        if let Some(path) = name.strip_prefix("minecraft:bred_") {
            renamed_entity_ids().get(&format!("minecraft:{}", path)[..]).map(|&str| str.to_owned())
        } else {
            renamed_entity_ids().get(name).map(|&str| str.to_owned())
        }
    });
    rename_stat(types, VERSION, |name| {
        match name {
            "minecraft:swim_one_cm" => Some("minecraft:walk_on_water_one_cm".to_owned()),
            "minecraft:dive_one_cm" => Some("minecraft:walk_under_water_one_cm".to_owned()),
            _ => None
        }
    });


    types.entity.borrow_mut().copy_walkers(VERSION, "minecraft:commandblock_minecart", "minecraft:command_block_minecart");
    types.entity.borrow_mut().copy_walkers(VERSION, "minecraft:ender_crystal", "minecraft:end_crystal");
    types.entity.borrow_mut().copy_walkers(VERSION, "minecraft:snowman", "minecraft:snow_golem");
    types.entity.borrow_mut().copy_walkers(VERSION, "minecraft:evocation_illager", "minecraft:evoker");
    types.entity.borrow_mut().copy_walkers(VERSION, "minecraft:evocation_fangs", "minecraft:evoker_fangs");
    types.entity.borrow_mut().copy_walkers(VERSION, "minecraft:illusion_illager", "minecraft:illusioner");
    types.entity.borrow_mut().copy_walkers(VERSION, "minecraft:vindication_illager", "minecraft:vindicator");
    types.entity.borrow_mut().copy_walkers(VERSION, "minecraft:villager_golem", "minecraft:iron_golem");
    types.entity.borrow_mut().copy_walkers(VERSION, "minecraft:xp_orb", "minecraft:experience_orb");
    types.entity.borrow_mut().copy_walkers(VERSION, "minecraft:xp_bottle", "minecraft:experience_bottle");
    types.entity.borrow_mut().copy_walkers(VERSION, "minecraft:eye_of_ender_signal", "minecraft:eye_of_ender");
    types.entity.borrow_mut().copy_walkers(VERSION, "minecraft:fireworks_rocket", "minecraft:firework_rocket");
}
