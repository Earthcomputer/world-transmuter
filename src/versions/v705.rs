use crate::helpers::hooks::{DataHookEnforceNamespacedId, DataHookValueTypeEnforceNamespaced};
use crate::helpers::rename::rename_entity;
use crate::types;
use std::collections::BTreeMap;
use std::sync::OnceLock;
use valence_nbt::{List, Value};
use world_transmuter_engine::{
    convert_map_in_map, data_walker, AbstractMapDataType, DataWalkerMapListPaths,
    DataWalkerMapTypePaths, DataWalkerObjectTypePaths,
};

const VERSION: u32 = 705;

static ENTITY_ID_UPDATE: OnceLock<BTreeMap<&'static str, &'static str>> = OnceLock::new();

fn entity_id_update() -> &'static BTreeMap<&'static str, &'static str> {
    ENTITY_ID_UPDATE.get_or_init(|| {
        let mut map = BTreeMap::new();
        map.insert("AreaEffectCloud", "minecraft:area_effect_cloud");
        map.insert("ArmorStand", "minecraft:armor_stand");
        map.insert("Arrow", "minecraft:arrow");
        map.insert("Bat", "minecraft:bat");
        map.insert("Blaze", "minecraft:blaze");
        map.insert("Boat", "minecraft:boat");
        map.insert("CaveSpider", "minecraft:cave_spider");
        map.insert("Chicken", "minecraft:chicken");
        map.insert("Cow", "minecraft:cow");
        map.insert("Creeper", "minecraft:creeper");
        map.insert("Donkey", "minecraft:donkey");
        map.insert("DragonFireball", "minecraft:dragon_fireball");
        map.insert("ElderGuardian", "minecraft:elder_guardian");
        map.insert("EnderCrystal", "minecraft:ender_crystal");
        map.insert("EnderDragon", "minecraft:ender_dragon");
        map.insert("Enderman", "minecraft:enderman");
        map.insert("Endermite", "minecraft:endermite");
        map.insert("EyeOfEnderSignal", "minecraft:eye_of_ender_signal");
        map.insert("FallingSand", "minecraft:falling_block");
        map.insert("Fireball", "minecraft:fireball");
        map.insert("FireworksRocketEntity", "minecraft:fireworks_rocket");
        map.insert("Ghast", "minecraft:ghast");
        map.insert("Giant", "minecraft:giant");
        map.insert("Guardian", "minecraft:guardian");
        map.insert("Horse", "minecraft:horse");
        map.insert("Husk", "minecraft:husk");
        map.insert("Item", "minecraft:item");
        map.insert("ItemFrame", "minecraft:item_frame");
        map.insert("LavaSlime", "minecraft:magma_cube");
        map.insert("LeashKnot", "minecraft:leash_knot");
        map.insert("MinecartChest", "minecraft:chest_minecart");
        map.insert("MinecartCommandBlock", "minecraft:commandblock_minecart");
        map.insert("MinecartFurnace", "minecraft:furnace_minecart");
        map.insert("MinecartHopper", "minecraft:hopper_minecart");
        map.insert("MinecartRideable", "minecraft:minecart");
        map.insert("MinecartSpawner", "minecraft:spawner_minecart");
        map.insert("MinecartTNT", "minecraft:tnt_minecart");
        map.insert("Mule", "minecraft:mule");
        map.insert("MushroomCow", "minecraft:mooshroom");
        map.insert("Ozelot", "minecraft:ocelot");
        map.insert("Painting", "minecraft:painting");
        map.insert("Pig", "minecraft:pig");
        map.insert("PigZombie", "minecraft:zombie_pigman");
        map.insert("PolarBear", "minecraft:polar_bear");
        map.insert("PrimedTnt", "minecraft:tnt");
        map.insert("Rabbit", "minecraft:rabbit");
        map.insert("Sheep", "minecraft:sheep");
        map.insert("Shulker", "minecraft:shulker");
        map.insert("ShulkerBullet", "minecraft:shulker_bullet");
        map.insert("Silverfish", "minecraft:silverfish");
        map.insert("Skeleton", "minecraft:skeleton");
        map.insert("SkeletonHorse", "minecraft:skeleton_horse");
        map.insert("Slime", "minecraft:slime");
        map.insert("SmallFireball", "minecraft:small_fireball");
        map.insert("SnowMan", "minecraft:snowman");
        map.insert("Snowball", "minecraft:snowball");
        map.insert("SpectralArrow", "minecraft:spectral_arrow");
        map.insert("Spider", "minecraft:spider");
        map.insert("Squid", "minecraft:squid");
        map.insert("Stray", "minecraft:stray");
        map.insert("ThrownEgg", "minecraft:egg");
        map.insert("ThrownEnderpearl", "minecraft:ender_pearl");
        map.insert("ThrownExpBottle", "minecraft:xp_bottle");
        map.insert("ThrownPotion", "minecraft:potion");
        map.insert("Villager", "minecraft:villager");
        map.insert("VillagerGolem", "minecraft:villager_golem");
        map.insert("Witch", "minecraft:witch");
        map.insert("WitherBoss", "minecraft:wither");
        map.insert("WitherSkeleton", "minecraft:wither_skeleton");
        map.insert("WitherSkull", "minecraft:wither_skull");
        map.insert("Wolf", "minecraft:wolf");
        map.insert("XPOrb", "minecraft:xp_orb");
        map.insert("Zombie", "minecraft:zombie");
        map.insert("ZombieHorse", "minecraft:zombie_horse");
        map.insert("ZombieVillager", "minecraft:zombie_villager");
        map
    })
}

pub(crate) fn register() {
    rename_entity(VERSION, |id| {
        entity_id_update().get(id).copied().map(|id| id.to_owned())
    });

    register_mob("minecraft:armor_stand");
    register_throwable_projectile("minecraft:arrow");
    register_mob("minecraft:bat");
    register_mob("minecraft:blaze");
    register_mob("minecraft:cave_spider");
    types::entity_mut().add_walker_for_id(
        VERSION,
        "minecraft:chest_minecart",
        DataWalkerObjectTypePaths::new(types::block_name_ref(), "DisplayTile"),
    );
    types::entity_mut().add_walker_for_id(
        VERSION,
        "minecraft:chest_minecart",
        DataWalkerMapListPaths::new(types::item_stack_ref(), "Items"),
    );
    register_mob("minecraft:cow");
    register_mob("minecraft:creeper");
    types::entity_mut().add_walker_for_id(
        VERSION,
        "minecraft:donkey",
        DataWalkerMapListPaths::new_multi(
            types::item_stack_ref(),
            vec![
                "Items".to_owned(),
                "ArmorItems".to_owned(),
                "HandItems".to_owned(),
            ],
        ),
    );
    types::entity_mut().add_walker_for_id(
        VERSION,
        "minecraft:donkey",
        DataWalkerMapTypePaths::new(types::item_stack_ref(), "SaddleItem"),
    );
    register_throwable_projectile("minecraft:egg");
    register_mob("minecraft:elder_guardian");
    register_mob("minecraft:ender_dragon");
    register_mob("minecraft:enderman");
    types::entity_mut().add_walker_for_id(
        VERSION,
        "minecraft:enderman",
        DataWalkerObjectTypePaths::new(types::block_name_ref(), "carried"),
    );
    register_mob("minecraft:endermite");
    register_throwable_projectile("minecraft:ender_pearl");
    types::entity_mut().add_walker_for_id(
        VERSION,
        "minecraft:falling_block",
        DataWalkerObjectTypePaths::new(types::block_name_ref(), "Block"),
    );
    types::entity_mut().add_walker_for_id(
        VERSION,
        "minecraft:falling_block",
        DataWalkerMapTypePaths::new(types::tile_entity_ref(), "TileEntityData"),
    );
    register_throwable_projectile("minecraft:fireball");
    types::entity_mut().add_walker_for_id(
        VERSION,
        "minecraft:fireworks_rocket",
        DataWalkerObjectTypePaths::new(types::item_name_ref(), "FireworksItem"),
    );
    types::entity_mut().add_walker_for_id(
        VERSION,
        "minecraft:furnace_minecart",
        DataWalkerObjectTypePaths::new(types::block_name_ref(), "DisplayTile"),
    );
    register_mob("minecraft:ghast");
    register_mob("minecraft:giant");
    register_mob("minecraft:guardian");
    types::entity_mut().add_walker_for_id(
        VERSION,
        "minecraft:hopper_minecart",
        DataWalkerObjectTypePaths::new(types::block_name_ref(), "DisplayTile"),
    );
    types::entity_mut().add_walker_for_id(
        VERSION,
        "minecraft:hopper_minecart",
        DataWalkerMapListPaths::new(types::item_stack_ref(), "Items"),
    );
    types::entity_mut().add_walker_for_id(
        VERSION,
        "minecraft:horse",
        DataWalkerMapListPaths::new_multi(
            types::item_stack_ref(),
            vec!["ArmorItems".to_owned(), "HandItems".to_owned()],
        ),
    );
    types::entity_mut().add_walker_for_id(
        VERSION,
        "minecraft:horse",
        DataWalkerMapTypePaths::new_multi(
            types::item_stack_ref(),
            vec!["ArmorItem".to_owned(), "SaddleItem".to_owned()],
        ),
    );
    register_mob("minecraft:husk");
    types::entity_mut().add_walker_for_id(
        VERSION,
        "minecraft:item",
        DataWalkerMapTypePaths::new(types::item_stack_ref(), "Item"),
    );
    types::entity_mut().add_walker_for_id(
        VERSION,
        "minecraft:item_frame",
        DataWalkerMapTypePaths::new(types::item_stack_ref(), "Item"),
    );
    register_mob("minecraft:magma_cube");
    types::entity_mut().add_walker_for_id(
        VERSION,
        "minecraft:minecart",
        DataWalkerObjectTypePaths::new(types::block_name_ref(), "DisplayTile"),
    );
    register_mob("minecraft:mooshroom");
    types::entity_mut().add_walker_for_id(
        VERSION,
        "minecraft:mule",
        DataWalkerMapListPaths::new_multi(
            types::item_stack_ref(),
            vec![
                "Items".to_owned(),
                "ArmorItems".to_owned(),
                "HandItems".to_owned(),
            ],
        ),
    );
    types::entity_mut().add_walker_for_id(
        VERSION,
        "minecraft:mule",
        DataWalkerMapTypePaths::new(types::item_stack_ref(), "SaddleItem"),
    );
    register_mob("minecraft:ocelot");
    register_mob("minecraft:pig");
    register_mob("minecraft:polar_bear");
    types::entity_mut().add_walker_for_id(
        VERSION,
        "minecraft:potion",
        DataWalkerMapTypePaths::new(types::item_stack_ref(), "Potion"),
    );
    register_throwable_projectile("minecraft:potion");
    register_mob("minecraft:rabbit");
    register_mob("minecraft:sheep");
    register_mob("minecraft:shulker");
    register_mob("minecraft:silverfish");
    register_mob("minecraft:skeleton");
    types::entity_mut().add_walker_for_id(
        VERSION,
        "minecraft:skeleton_horse",
        DataWalkerMapListPaths::new_multi(
            types::item_stack_ref(),
            vec!["ArmorItems".to_owned(), "HandItems".to_owned()],
        ),
    );
    types::entity_mut().add_walker_for_id(
        VERSION,
        "minecraft:skeleton_horse",
        DataWalkerMapTypePaths::new(types::item_stack_ref(), "SaddleItem"),
    );
    register_mob("minecraft:slime");
    register_throwable_projectile("minecraft:small_fireball");
    register_throwable_projectile("minecraft:snowball");
    register_mob("minecraft:snowman");
    types::entity_mut().add_walker_for_id(
        VERSION,
        "minecraft:spawner_minecart",
        DataWalkerObjectTypePaths::new(types::block_name_ref(), "DisplayTile"),
    );
    types::entity_mut().add_walker_for_id(
        VERSION,
        "minecraft:spawner_minecart",
        data_walker(move |data, from_version, to_version| {
            types::untagged_spawner().convert(data, from_version, to_version);
        }),
    );
    register_throwable_projectile("minecraft:spectral_arrow");
    register_mob("minecraft:spider");
    register_mob("minecraft:squid");
    register_mob("minecraft:stray");
    types::entity_mut().add_walker_for_id(
        VERSION,
        "minecraft:tnt_minecart",
        DataWalkerObjectTypePaths::new(types::block_name_ref(), "DisplayTile"),
    );
    types::entity_mut().add_walker_for_id(
        VERSION,
        "minecraft:villager",
        DataWalkerMapListPaths::new_multi(
            types::item_stack_ref(),
            vec![
                "Inventory".to_owned(),
                "ArmorItems".to_owned(),
                "HandItems".to_owned(),
            ],
        ),
    );
    types::entity_mut().add_walker_for_id(
        VERSION,
        "minecraft:villager",
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
    register_mob("minecraft:villager_golem");
    register_mob("minecraft:witch");
    register_mob("minecraft:wither");
    register_mob("minecraft:wither_skeleton");
    register_throwable_projectile("minecraft:wither_skull");
    register_mob("minecraft:wolf");
    register_throwable_projectile("minecraft:xp_bottle");
    register_mob("minecraft:zombie");
    types::entity_mut().add_walker_for_id(
        VERSION,
        "minecraft:zombie_horse",
        DataWalkerMapListPaths::new_multi(
            types::item_stack_ref(),
            vec!["ArmorItems".to_owned(), "HandItems".to_owned()],
        ),
    );
    types::entity_mut().add_walker_for_id(
        VERSION,
        "minecraft:zombie_horse",
        DataWalkerMapTypePaths::new(types::item_stack_ref(), "SaddleItem"),
    );
    register_mob("minecraft:zombie_pigman");
    register_mob("minecraft:zombie_villager");
    register_mob("minecraft:evocation_illager");
    types::entity_mut().add_walker_for_id(
        VERSION,
        "minecraft:llama",
        DataWalkerMapListPaths::new_multi(
            types::item_stack_ref(),
            vec![
                "Items".to_owned(),
                "ArmorItems".to_owned(),
                "HandItems".to_owned(),
            ],
        ),
    );
    types::entity_mut().add_walker_for_id(
        VERSION,
        "minecraft:llama",
        DataWalkerMapTypePaths::new_multi(
            types::item_stack_ref(),
            vec!["SaddleItem".to_owned(), "DecorItem".to_owned()],
        ),
    );
    register_mob("minecraft:vex");
    register_mob("minecraft:vindication_illager");
    // Don't need to re-register itemstack walker, the V704 will correctly choose the right id for armorstand based on
    // the source version

    // Enforce namespace for ids
    types::entity_mut().add_structure_hook(VERSION, DataHookEnforceNamespacedId::id());
    types::entity_name_mut().add_structure_hook(VERSION, DataHookValueTypeEnforceNamespaced);
}

fn register_mob(id: impl Into<String>) {
    types::entity_mut().add_walker_for_id(
        VERSION,
        id,
        DataWalkerMapListPaths::new_multi(
            types::item_stack_ref(),
            vec!["ArmorItems".to_owned(), "HandItems".to_owned()],
        ),
    );
}

fn register_throwable_projectile(id: impl Into<String>) {
    types::entity_mut().add_walker_for_id(
        VERSION,
        id,
        DataWalkerObjectTypePaths::new(types::block_name_ref(), "inTile"),
    );
}
