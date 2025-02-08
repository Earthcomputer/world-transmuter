use crate::helpers::hooks::{DataHookEnforceNamespacedId, DataHookValueTypeEnforceNamespaced};
use crate::helpers::rename::rename_entity;
use crate::versions::v100;
use crate::{static_string_map, types};
use java_string::JavaString;
use world_transmuter_engine::{
    convert_map_list_in_map, map_data_walker, AbstractMapDataType, DataWalkerDynamicTypePaths,
    DataWalkerMapListPaths, DataWalkerMapTypePaths, DataWalkerObjectTypePaths, JValue,
};

const VERSION: u32 = 705;

static_string_map! {
    entity_id_update = {
        "AreaEffectCloud" => "minecraft:area_effect_cloud",
        "ArmorStand" => "minecraft:armor_stand",
        "Arrow" => "minecraft:arrow",
        "Bat" => "minecraft:bat",
        "Blaze" => "minecraft:blaze",
        "Boat" => "minecraft:boat",
        "CaveSpider" => "minecraft:cave_spider",
        "Chicken" => "minecraft:chicken",
        "Cow" => "minecraft:cow",
        "Creeper" => "minecraft:creeper",
        "Donkey" => "minecraft:donkey",
        "DragonFireball" => "minecraft:dragon_fireball",
        "ElderGuardian" => "minecraft:elder_guardian",
        "EnderCrystal" => "minecraft:ender_crystal",
        "EnderDragon" => "minecraft:ender_dragon",
        "Enderman" => "minecraft:enderman",
        "Endermite" => "minecraft:endermite",
        "EyeOfEnderSignal" => "minecraft:eye_of_ender_signal",
        "FallingSand" => "minecraft:falling_block",
        "Fireball" => "minecraft:fireball",
        "FireworksRocketEntity" => "minecraft:fireworks_rocket",
        "Ghast" => "minecraft:ghast",
        "Giant" => "minecraft:giant",
        "Guardian" => "minecraft:guardian",
        "Horse" => "minecraft:horse",
        "Husk" => "minecraft:husk",
        "Item" => "minecraft:item",
        "ItemFrame" => "minecraft:item_frame",
        "LavaSlime" => "minecraft:magma_cube",
        "LeashKnot" => "minecraft:leash_knot",
        "MinecartChest" => "minecraft:chest_minecart",
        "MinecartCommandBlock" => "minecraft:commandblock_minecart",
        "MinecartFurnace" => "minecraft:furnace_minecart",
        "MinecartHopper" => "minecraft:hopper_minecart",
        "MinecartRideable" => "minecraft:minecart",
        "MinecartSpawner" => "minecraft:spawner_minecart",
        "MinecartTNT" => "minecraft:tnt_minecart",
        "Mule" => "minecraft:mule",
        "MushroomCow" => "minecraft:mooshroom",
        "Ozelot" => "minecraft:ocelot",
        "Painting" => "minecraft:painting",
        "Pig" => "minecraft:pig",
        "PigZombie" => "minecraft:zombie_pigman",
        "PolarBear" => "minecraft:polar_bear",
        "PrimedTnt" => "minecraft:tnt",
        "Rabbit" => "minecraft:rabbit",
        "Sheep" => "minecraft:sheep",
        "Shulker" => "minecraft:shulker",
        "ShulkerBullet" => "minecraft:shulker_bullet",
        "Silverfish" => "minecraft:silverfish",
        "Skeleton" => "minecraft:skeleton",
        "SkeletonHorse" => "minecraft:skeleton_horse",
        "Slime" => "minecraft:slime",
        "SmallFireball" => "minecraft:small_fireball",
        "SnowMan" => "minecraft:snowman",
        "Snowball" => "minecraft:snowball",
        "SpectralArrow" => "minecraft:spectral_arrow",
        "Spider" => "minecraft:spider",
        "Squid" => "minecraft:squid",
        "Stray" => "minecraft:stray",
        "ThrownEgg" => "minecraft:egg",
        "ThrownEnderpearl" => "minecraft:ender_pearl",
        "ThrownExpBottle" => "minecraft:xp_bottle",
        "ThrownPotion" => "minecraft:potion",
        "Villager" => "minecraft:villager",
        "VillagerGolem" => "minecraft:villager_golem",
        "Witch" => "minecraft:witch",
        "WitherBoss" => "minecraft:wither",
        "WitherSkeleton" => "minecraft:wither_skeleton",
        "WitherSkull" => "minecraft:wither_skull",
        "Wolf" => "minecraft:wolf",
        "XPOrb" => "minecraft:xp_orb",
        "Zombie" => "minecraft:zombie",
        "ZombieHorse" => "minecraft:zombie_horse",
        "ZombieVillager" => "minecraft:zombie_villager",
    }
}

pub(crate) fn register() {
    rename_entity(VERSION, |id| {
        entity_id_update().get(id).copied().map(|id| id.to_owned())
    });

    types::entity_mut().add_walker_for_id(
        VERSION,
        "minecraft:area_effect_cloud",
        DataWalkerDynamicTypePaths::new(types::particle_ref(), "Particle"),
    );
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
        DataWalkerMapListPaths::new(types::item_stack_ref(), "Items"),
    );
    v100::register_equipment(VERSION, "minecraft:donkey");
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
    v100::register_equipment(VERSION, "minecraft:horse");
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
        DataWalkerMapListPaths::new(types::item_stack_ref(), "Items"),
    );
    v100::register_equipment(VERSION, "minecraft:mule");
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
        map_data_walker(move |data, from_version, to_version| {
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
    v100::register_equipment(VERSION, "minecraft:villager");
    register_mob("minecraft:villager_golem");
    register_mob("minecraft:witch");
    register_mob("minecraft:wither");
    register_mob("minecraft:wither_skeleton");
    register_throwable_projectile("minecraft:wither_skull");
    register_mob("minecraft:wolf");
    register_throwable_projectile("minecraft:xp_bottle");
    register_mob("minecraft:zombie");
    v100::register_equipment(VERSION, "minecraft:zombie_horse");
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
        DataWalkerMapListPaths::new(types::item_stack_ref(), "Items"),
    );
    v100::register_equipment(VERSION, "minecraft:llama");
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

fn register_mob(id: impl Into<JavaString>) {
    v100::register_equipment(VERSION, id);
}

fn register_throwable_projectile(id: impl Into<JavaString>) {
    types::entity_mut().add_walker_for_id(
        VERSION,
        id,
        DataWalkerObjectTypePaths::new(types::block_name_ref(), "inTile"),
    );
}
