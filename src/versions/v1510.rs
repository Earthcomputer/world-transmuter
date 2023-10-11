use crate::helpers::mc_namespace_map::McNamespaceMap;
use crate::helpers::rename::{
    rename_block, rename_entity, rename_item, rename_recipe, rename_stat,
};
use crate::{static_string_mc_map, types};
use java_string::{format_java, JavaStr, JavaString};
use std::sync::OnceLock;

const VERSION: u32 = 1510;

static_string_mc_map! {
    RENAMED_ENTITY_IDS, renamed_entity_ids, {
        "commandblock_minecart" => "minecraft:command_block_minecart",
        "ender_crystal" => "minecraft:end_crystal",
        "snowman" => "minecraft:snow_golem",
        "evocation_illager" => "minecraft:evoker",
        "evocation_fangs" => "minecraft:evoker_fangs",
        "illusion_illager" => "minecraft:illusioner",
        "vindication_illager" => "minecraft:vindicator",
        "villager_golem" => "minecraft:iron_golem",
        "xp_orb" => "minecraft:experience_orb",
        "xp_bottle" => "minecraft:experience_bottle",
        "eye_of_ender_signal" => "minecraft:eye_of_ender",
        "fireworks_rocket" => "minecraft:firework_rocket",
    }
}

static_string_mc_map! {
    RENAMED_BLOCKS, renamed_blocks, {
        "portal" => "minecraft:nether_portal",
        "oak_bark" => "minecraft:oak_wood",
        "spruce_bark" => "minecraft:spruce_wood",
        "birch_bark" => "minecraft:birch_wood",
        "jungle_bark" => "minecraft:jungle_wood",
        "acacia_bark" => "minecraft:acacia_wood",
        "dark_oak_bark" => "minecraft:dark_oak_wood",
        "stripped_oak_bark" => "minecraft:stripped_oak_wood",
        "stripped_spruce_bark" => "minecraft:stripped_spruce_wood",
        "stripped_birch_bark" => "minecraft:stripped_birch_wood",
        "stripped_jungle_bark" => "minecraft:stripped_jungle_wood",
        "stripped_acacia_bark" => "minecraft:stripped_acacia_wood",
        "stripped_dark_oak_bark" => "minecraft:stripped_dark_oak_wood",
        "mob_spawner" => "minecraft:spawner",
    }
}

static RENAMED_ITEMS: OnceLock<McNamespaceMap<&'static JavaStr>> = OnceLock::new();

fn renamed_items() -> &'static McNamespaceMap<'static, &'static JavaStr> {
    RENAMED_ITEMS.get_or_init(|| {
        let mut map = McNamespaceMap::new();
        for (&k, &v) in renamed_blocks().iter_mc_to_value() {
            map.insert_mc(k, v);
        }
        map.insert_mc("clownfish", JavaStr::from_str("minecraft:tropical_fish"));
        map.insert_mc(
            "chorus_fruit_popped",
            JavaStr::from_str("minecraft:popped_chorus_fruit"),
        );
        map.insert_mc(
            "evocation_illager_spawn_egg",
            JavaStr::from_str("minecraft:evoker_spawn_egg"),
        );
        map.insert_mc(
            "vindication_illager_spawn_egg",
            JavaStr::from_str("minecraft:vindicator_spawn_egg"),
        );
        map
    })
}

static_string_mc_map! {
    RECIPES_UPDATES, recipes_updates, {
        "acacia_bark" => "minecraft:acacia_wood",
        "birch_bark" => "minecraft:birch_wood",
        "dark_oak_bark" => "minecraft:dark_oak_wood",
        "jungle_bark" => "minecraft:jungle_wood",
        "oak_bark" => "minecraft:oak_wood",
        "spruce_bark" => "minecraft:spruce_wood",
    }
}

pub(crate) fn register() {
    rename_block(VERSION, |name| {
        renamed_blocks().get(name).map(|&str| str.to_owned())
    });
    rename_item(VERSION, |name| {
        renamed_items().get(name).map(|&str| str.to_owned())
    });
    rename_recipe(VERSION, |name| {
        recipes_updates().get(name).map(|&str| str.to_owned())
    });
    rename_entity(VERSION, |name| {
        if let Some(path) = name.strip_prefix("minecraft:bred_") {
            renamed_entity_ids()
                .get(&format_java!("minecraft:{}", path)[..])
                .map(|&str| str.to_owned())
        } else {
            renamed_entity_ids().get(name).map(|&str| str.to_owned())
        }
    });
    rename_stat(VERSION, |name| match name.as_bytes() {
        b"minecraft:swim_one_cm" => Some(JavaString::from("minecraft:walk_on_water_one_cm")),
        b"minecraft:dive_one_cm" => Some(JavaString::from("minecraft:walk_under_water_one_cm")),
        _ => None,
    });

    types::entity_mut().copy_walkers(
        VERSION,
        "minecraft:commandblock_minecart",
        "minecraft:command_block_minecart",
    );
    types::entity_mut().copy_walkers(VERSION, "minecraft:ender_crystal", "minecraft:end_crystal");
    types::entity_mut().copy_walkers(VERSION, "minecraft:snowman", "minecraft:snow_golem");
    types::entity_mut().copy_walkers(VERSION, "minecraft:evocation_illager", "minecraft:evoker");
    types::entity_mut().copy_walkers(
        VERSION,
        "minecraft:evocation_fangs",
        "minecraft:evoker_fangs",
    );
    types::entity_mut().copy_walkers(
        VERSION,
        "minecraft:illusion_illager",
        "minecraft:illusioner",
    );
    types::entity_mut().copy_walkers(
        VERSION,
        "minecraft:vindication_illager",
        "minecraft:vindicator",
    );
    types::entity_mut().copy_walkers(VERSION, "minecraft:villager_golem", "minecraft:iron_golem");
    types::entity_mut().copy_walkers(VERSION, "minecraft:xp_orb", "minecraft:experience_orb");
    types::entity_mut().copy_walkers(
        VERSION,
        "minecraft:xp_bottle",
        "minecraft:experience_bottle",
    );
    types::entity_mut().copy_walkers(
        VERSION,
        "minecraft:eye_of_ender_signal",
        "minecraft:eye_of_ender",
    );
    types::entity_mut().copy_walkers(
        VERSION,
        "minecraft:fireworks_rocket",
        "minecraft:firework_rocket",
    );
}
