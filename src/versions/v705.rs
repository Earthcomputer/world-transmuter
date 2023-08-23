use std::collections::BTreeMap;
use std::sync::OnceLock;
use rust_dataconverter_engine::{AbstractMapDataType, convert_map_in_map, data_walker, DataWalkerMapListPaths, DataWalkerMapTypePaths, DataWalkerObjectTypePaths};
use valence_nbt::{List, Value};
use crate::helpers::hooks::{DataHookEnforceNamespacedId, DataHookValueTypeEnforceNamespaced};
use crate::helpers::rename::rename_entity;
use crate::MinecraftTypesMut;

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

pub(crate) fn register(types: &MinecraftTypesMut) {
    rename_entity(types, VERSION, |id| entity_id_update().get(id).copied().map(|id| id.to_owned()));

    register_mob(types, "minecraft:armor_stand");
    register_throwable_projectile(types, "minecraft:arrow");
    register_mob(types, "minecraft:bat");
    register_mob(types, "minecraft:blaze");
    register_mob(types, "minecraft:cave_spider");
    types.entity.borrow_mut().add_walker_for_id(VERSION, "minecraft:chest_minecart", DataWalkerObjectTypePaths::new(types.block_name, "DisplayTile"));
    types.entity.borrow_mut().add_walker_for_id(VERSION, "minecraft:chest_minecart", DataWalkerMapListPaths::new(types.item_stack, "Items"));
    register_mob(types, "minecraft:cow");
    register_mob(types, "minecraft:creeper");
    types.entity.borrow_mut().add_walker_for_id(VERSION, "minecraft:donkey", DataWalkerMapListPaths::new_multi(types.item_stack, vec!["Items".to_owned(), "ArmorItems".to_owned(), "HandItems".to_owned()]));
    types.entity.borrow_mut().add_walker_for_id(VERSION, "minecraft:donkey", DataWalkerMapTypePaths::new(types.item_stack, "SaddleItem"));
    register_throwable_projectile(types, "minecraft:egg");
    register_mob(types, "minecraft:elder_guardian");
    register_mob(types, "minecraft:ender_dragon");
    register_mob(types, "minecraft:enderman");
    types.entity.borrow_mut().add_walker_for_id(VERSION, "minecraft:enderman", DataWalkerObjectTypePaths::new(types.block_name, "carried"));
    register_mob(types, "minecraft:endermite");
    register_throwable_projectile(types, "minecraft:ender_pearl");
    types.entity.borrow_mut().add_walker_for_id(VERSION, "minecraft:falling_block", DataWalkerObjectTypePaths::new(types.block_name, "Block"));
    types.entity.borrow_mut().add_walker_for_id(VERSION, "minecraft:falling_block", DataWalkerMapTypePaths::new(types.tile_entity, "TileEntityData"));
    register_throwable_projectile(types, "minecraft:fireball");
    types.entity.borrow_mut().add_walker_for_id(VERSION, "minecraft:fireworks_rocket", DataWalkerObjectTypePaths::new(types.item_name, "FireworksItem"));
    types.entity.borrow_mut().add_walker_for_id(VERSION, "minecraft:furnace_minecart", DataWalkerObjectTypePaths::new(types.block_name, "DisplayTile"));
    register_mob(types, "minecraft:ghast");
    register_mob(types, "minecraft:giant");
    register_mob(types, "minecraft:guardian");
    types.entity.borrow_mut().add_walker_for_id(VERSION, "minecraft:hopper_minecart", DataWalkerObjectTypePaths::new(types.block_name, "DisplayTile"));
    types.entity.borrow_mut().add_walker_for_id(VERSION, "minecraft:hopper_minecart", DataWalkerMapListPaths::new(types.item_stack, "Items"));
    types.entity.borrow_mut().add_walker_for_id(VERSION, "minecraft:horse", DataWalkerMapListPaths::new_multi(types.item_stack, vec!["ArmorItems".to_owned(), "HandItems".to_owned()]));
    types.entity.borrow_mut().add_walker_for_id(VERSION, "minecraft:horse", DataWalkerMapTypePaths::new_multi(types.item_stack, vec!["ArmorItem".to_owned(), "SaddleItem".to_owned()]));
    register_mob(types, "minecraft:husk");
    types.entity.borrow_mut().add_walker_for_id(VERSION, "minecraft:item", DataWalkerMapTypePaths::new(types.item_stack, "Item"));
    types.entity.borrow_mut().add_walker_for_id(VERSION, "minecraft:item_frame", DataWalkerMapTypePaths::new(types.item_stack, "Item"));
    register_mob(types, "minecraft:magma_cube");
    types.entity.borrow_mut().add_walker_for_id(VERSION, "minecraft:minecart", DataWalkerObjectTypePaths::new(types.block_name, "DisplayTile"));
    register_mob(types, "minecraft:mooshroom");
    types.entity.borrow_mut().add_walker_for_id(VERSION, "minecraft:mule", DataWalkerMapListPaths::new_multi(types.item_stack, vec!["Items".to_owned(), "ArmorItems".to_owned(), "HandItems".to_owned()]));
    types.entity.borrow_mut().add_walker_for_id(VERSION, "minecraft:mule", DataWalkerMapTypePaths::new(types.item_stack, "SaddleItem"));
    register_mob(types, "minecraft:ocelot");
    register_mob(types, "minecraft:pig");
    register_mob(types, "minecraft:polar_bear");
    types.entity.borrow_mut().add_walker_for_id(VERSION, "minecraft:potion", DataWalkerMapTypePaths::new(types.item_stack, "Potion"));
    register_throwable_projectile(types, "minecraft:potion");
    register_mob(types, "minecraft:rabbit");
    register_mob(types, "minecraft:sheep");
    register_mob(types, "minecraft:shulker");
    register_mob(types, "minecraft:silverfish");
    register_mob(types, "minecraft:skeleton");
    types.entity.borrow_mut().add_walker_for_id(VERSION, "minecraft:skeleton_horse", DataWalkerMapListPaths::new_multi(types.item_stack, vec!["ArmorItems".to_owned(), "HandItems".to_owned()]));
    types.entity.borrow_mut().add_walker_for_id(VERSION, "minecraft:skeleton_horse", DataWalkerMapTypePaths::new(types.item_stack, "SaddleItem"));
    register_mob(types, "minecraft:slime");
    register_throwable_projectile(types, "minecraft:small_fireball");
    register_throwable_projectile(types, "minecraft:snowball");
    register_mob(types, "minecraft:snowman");
    types.entity.borrow_mut().add_walker_for_id(VERSION, "minecraft:spawner_minecart", DataWalkerObjectTypePaths::new(types.block_name, "DisplayTile"));
    let untagged_spawner_type = types.untagged_spawner;
    types.entity.borrow_mut().add_walker_for_id(VERSION, "minecraft:spawner_minecart", data_walker(move |data, from_version, to_version| {
        untagged_spawner_type.convert(data, from_version, to_version);
    }));
    register_throwable_projectile(types, "minecraft:spectral_arrow");
    register_mob(types, "minecraft:spider");
    register_mob(types, "minecraft:squid");
    register_mob(types, "minecraft:stray");
    types.entity.borrow_mut().add_walker_for_id(VERSION, "minecraft:tnt_minecart", DataWalkerObjectTypePaths::new(types.block_name, "DisplayTile"));
    types.entity.borrow_mut().add_walker_for_id(VERSION, "minecraft:villager", DataWalkerMapListPaths::new_multi(types.item_stack, vec!["Inventory".to_owned(), "ArmorItems".to_owned(), "HandItems".to_owned()]));
    let item_stack_type = types.item_stack;
    types.entity.borrow_mut().add_walker_for_id(VERSION, "minecraft:villager", data_walker(move |data, from_version, to_version| {
        if let Some(Value::Compound(offers)) = data.get_mut("Offers") {
            if let Some(Value::List(List::Compound(recipes))) = offers.get_mut("Recipes") {
                for recipe in recipes {
                    convert_map_in_map(item_stack_type, recipe, "buy", from_version, to_version);
                    convert_map_in_map(item_stack_type, recipe, "buyB", from_version, to_version);
                    convert_map_in_map(item_stack_type, recipe, "sell", from_version, to_version);
                }
            }
        }
    }));
    register_mob(types, "minecraft:villager_golem");
    register_mob(types, "minecraft:witch");
    register_mob(types, "minecraft:wither");
    register_mob(types, "minecraft:wither_skeleton");
    register_throwable_projectile(types, "minecraft:wither_skull");
    register_mob(types, "minecraft:wolf");
    register_throwable_projectile(types, "minecraft:xp_bottle");
    register_mob(types, "minecraft:zombie");
    types.entity.borrow_mut().add_walker_for_id(VERSION, "minecraft:zombie_horse", DataWalkerMapListPaths::new_multi(types.item_stack, vec!["ArmorItems".to_owned(), "HandItems".to_owned()]));
    types.entity.borrow_mut().add_walker_for_id(VERSION, "minecraft:zombie_horse", DataWalkerMapTypePaths::new(types.item_stack, "SaddleItem"));
    register_mob(types, "minecraft:zombie_pigman");
    register_mob(types, "minecraft:zombie_villager");
    register_mob(types, "minecraft:evocation_illager");
    types.entity.borrow_mut().add_walker_for_id(VERSION, "minecraft:llama", DataWalkerMapListPaths::new_multi(types.item_stack, vec!["Items".to_owned(), "ArmorItems".to_owned(), "HandItems".to_owned()]));
    types.entity.borrow_mut().add_walker_for_id(VERSION, "minecraft:llama", DataWalkerMapTypePaths::new_multi(types.item_stack, vec!["SaddleItem".to_owned(), "DecorItem".to_owned()]));
    register_mob(types, "minecraft:vex");
    register_mob(types, "minecraft:vindication_illager");
    // Don't need to re-register itemstack walker, the V704 will correctly choose the right id for armorstand based on
    // the source version

    // Enforce namespace for ids
    types.entity.borrow_mut().add_structure_hook(VERSION, DataHookEnforceNamespacedId::id());
    types.entity_name.borrow_mut().add_structure_hook(VERSION, DataHookValueTypeEnforceNamespaced);
}

fn register_mob(types: &MinecraftTypesMut, id: impl Into<String>) {
    types.entity.borrow_mut().add_walker_for_id(VERSION, id, DataWalkerMapListPaths::new_multi(types.item_stack, vec!["ArmorItems".to_owned(), "HandItems".to_owned()]));
}

fn register_throwable_projectile(types: &MinecraftTypesMut, id: impl Into<String>) {
    types.entity.borrow_mut().add_walker_for_id(VERSION, id, DataWalkerObjectTypePaths::new(types.block_name, "inTile"));
}
