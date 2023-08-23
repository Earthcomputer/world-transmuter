use std::sync::OnceLock;
use crate::helpers::mc_namespace_map::McNamespaceMap;
use crate::helpers::rename::{rename_block, rename_entity, rename_item, rename_recipe, rename_stat};
use crate::MinecraftTypesMut;

const VERSION: u32 = 1510;

static RENAMED_ENTITY_IDS: OnceLock<McNamespaceMap<&'static str>> = OnceLock::new();

fn renamed_entity_ids() -> &'static McNamespaceMap<'static, &'static str> {
    RENAMED_ENTITY_IDS.get_or_init(|| {
        let mut map = McNamespaceMap::new();
        map.insert_mc("commandblock_minecart", "minecraft:command_block_minecart");
        map.insert_mc("ender_crystal", "minecraft:end_crystal");
        map.insert_mc("snowman", "minecraft:snow_golem");
        map.insert_mc("evocation_illager", "minecraft:evoker");
        map.insert_mc("evocation_fangs", "minecraft:evoker_fangs");
        map.insert_mc("illusion_illager", "minecraft:illusioner");
        map.insert_mc("vindication_illager", "minecraft:vindicator");
        map.insert_mc("villager_golem", "minecraft:iron_golem");
        map.insert_mc("xp_orb", "minecraft:experience_orb");
        map.insert_mc("xp_bottle", "minecraft:experience_bottle");
        map.insert_mc("eye_of_ender_signal", "minecraft:eye_of_ender");
        map.insert_mc("fireworks_rocket", "minecraft:firework_rocket");
        map
    })
}

static RENAMED_BLOCKS: OnceLock<McNamespaceMap<&'static str>> = OnceLock::new();

fn renamed_blocks() -> &'static McNamespaceMap<'static, &'static str> {
    RENAMED_BLOCKS.get_or_init(|| {
        let mut map = McNamespaceMap::new();
        map.insert_mc("portal", "minecraft:nether_portal");
        map.insert_mc("oak_bark", "minecraft:oak_wood");
        map.insert_mc("spruce_bark", "minecraft:spruce_wood");
        map.insert_mc("birch_bark", "minecraft:birch_wood");
        map.insert_mc("jungle_bark", "minecraft:jungle_wood");
        map.insert_mc("acacia_bark", "minecraft:acacia_wood");
        map.insert_mc("dark_oak_bark", "minecraft:dark_oak_wood");
        map.insert_mc("stripped_oak_bark", "minecraft:stripped_oak_wood");
        map.insert_mc("stripped_spruce_bark", "minecraft:stripped_spruce_wood");
        map.insert_mc("stripped_birch_bark", "minecraft:stripped_birch_wood");
        map.insert_mc("stripped_jungle_bark", "minecraft:stripped_jungle_wood");
        map.insert_mc("stripped_acacia_bark", "minecraft:stripped_acacia_wood");
        map.insert_mc("stripped_dark_oak_bark", "minecraft:stripped_dark_oak_wood");
        map.insert_mc("mob_spawner", "minecraft:spawner");
        map
    })
}

static RENAMED_ITEMS: OnceLock<McNamespaceMap<&'static str>> = OnceLock::new();

fn renamed_items() -> &'static McNamespaceMap<'static, &'static str> {
    RENAMED_ITEMS.get_or_init(|| {
        let mut map = McNamespaceMap::new();
        for (&k, &v) in renamed_blocks().iter_mc_to_value() {
            map.insert_mc(k, v);
        }
        map.insert_mc("clownfish", "minecraft:tropical_fish");
        map.insert_mc("chorus_fruit_popped", "minecraft:popped_chorus_fruit");
        map.insert_mc("evocation_illager_spawn_egg", "minecraft:evoker_spawn_egg");
        map.insert_mc("vindication_illager_spawn_egg", "minecraft:vindicator_spawn_egg");
        map
    })
}

static RECIPES_UPDATES: OnceLock<McNamespaceMap<&'static str>> = OnceLock::new();

fn recipes_updates() -> &'static McNamespaceMap<'static, &'static str> {
    RECIPES_UPDATES.get_or_init(|| {
        let mut map = McNamespaceMap::new();
        map.insert_mc("acacia_bark", "minecraft:acacia_wood");
        map.insert_mc("birch_bark", "minecraft:birch_wood");
        map.insert_mc("dark_oak_bark", "minecraft:dark_oak_wood");
        map.insert_mc("jungle_bark", "minecraft:jungle_wood");
        map.insert_mc("oak_bark", "minecraft:oak_wood");
        map.insert_mc("spruce_bark", "minecraft:spruce_wood");
        map
    })
}

pub(crate) fn register(types: &MinecraftTypesMut) {
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
