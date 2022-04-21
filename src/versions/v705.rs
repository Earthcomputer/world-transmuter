use std::lazy::SyncOnceCell;
use rust_dataconverter_engine::{convert_map_in_map, data_walker, DataType, DataWalkerMapListPaths, DataWalkerMapTypePaths, DataWalkerObjectTypePaths, ListType, MapType, ObjectType, Types};
use crate::helpers::hooks::{DataHookEnforceNamespacedId, DataHookValueTypeEnforceNamespaced};
use crate::helpers::rename::rename_entity;
use crate::MinecraftTypesMut;

const VERSION: u32 = 705;

static ENTITY_ID_UPDATE: SyncOnceCell<rust_dataconverter_engine::Map<String, String>> = SyncOnceCell::new();

fn entity_id_update() -> &'static rust_dataconverter_engine::Map<String, String> {
    ENTITY_ID_UPDATE.get_or_init(|| {
        let mut map = rust_dataconverter_engine::Map::new();
        map.insert("AreaEffectCloud".to_owned(), "minecraft:area_effect_cloud".to_owned());
        map.insert("ArmorStand".to_owned(), "minecraft:armor_stand".to_owned());
        map.insert("Arrow".to_owned(), "minecraft:arrow".to_owned());
        map.insert("Bat".to_owned(), "minecraft:bat".to_owned());
        map.insert("Blaze".to_owned(), "minecraft:blaze".to_owned());
        map.insert("Boat".to_owned(), "minecraft:boat".to_owned());
        map.insert("CaveSpider".to_owned(), "minecraft:cave_spider".to_owned());
        map.insert("Chicken".to_owned(), "minecraft:chicken".to_owned());
        map.insert("Cow".to_owned(), "minecraft:cow".to_owned());
        map.insert("Creeper".to_owned(), "minecraft:creeper".to_owned());
        map.insert("Donkey".to_owned(), "minecraft:donkey".to_owned());
        map.insert("DragonFireball".to_owned(), "minecraft:dragon_fireball".to_owned());
        map.insert("ElderGuardian".to_owned(), "minecraft:elder_guardian".to_owned());
        map.insert("EnderCrystal".to_owned(), "minecraft:ender_crystal".to_owned());
        map.insert("EnderDragon".to_owned(), "minecraft:ender_dragon".to_owned());
        map.insert("Enderman".to_owned(), "minecraft:enderman".to_owned());
        map.insert("Endermite".to_owned(), "minecraft:endermite".to_owned());
        map.insert("EyeOfEnderSignal".to_owned(), "minecraft:eye_of_ender_signal".to_owned());
        map.insert("FallingSand".to_owned(), "minecraft:falling_block".to_owned());
        map.insert("Fireball".to_owned(), "minecraft:fireball".to_owned());
        map.insert("FireworksRocketEntity".to_owned(), "minecraft:fireworks_rocket".to_owned());
        map.insert("Ghast".to_owned(), "minecraft:ghast".to_owned());
        map.insert("Giant".to_owned(), "minecraft:giant".to_owned());
        map.insert("Guardian".to_owned(), "minecraft:guardian".to_owned());
        map.insert("Horse".to_owned(), "minecraft:horse".to_owned());
        map.insert("Husk".to_owned(), "minecraft:husk".to_owned());
        map.insert("Item".to_owned(), "minecraft:item".to_owned());
        map.insert("ItemFrame".to_owned(), "minecraft:item_frame".to_owned());
        map.insert("LavaSlime".to_owned(), "minecraft:magma_cube".to_owned());
        map.insert("LeashKnot".to_owned(), "minecraft:leash_knot".to_owned());
        map.insert("MinecartChest".to_owned(), "minecraft:chest_minecart".to_owned());
        map.insert("MinecartCommandBlock".to_owned(), "minecraft:commandblock_minecart".to_owned());
        map.insert("MinecartFurnace".to_owned(), "minecraft:furnace_minecart".to_owned());
        map.insert("MinecartHopper".to_owned(), "minecraft:hopper_minecart".to_owned());
        map.insert("MinecartRideable".to_owned(), "minecraft:minecart".to_owned());
        map.insert("MinecartSpawner".to_owned(), "minecraft:spawner_minecart".to_owned());
        map.insert("MinecartTNT".to_owned(), "minecraft:tnt_minecart".to_owned());
        map.insert("Mule".to_owned(), "minecraft:mule".to_owned());
        map.insert("MushroomCow".to_owned(), "minecraft:mooshroom".to_owned());
        map.insert("Ozelot".to_owned(), "minecraft:ocelot".to_owned());
        map.insert("Painting".to_owned(), "minecraft:painting".to_owned());
        map.insert("Pig".to_owned(), "minecraft:pig".to_owned());
        map.insert("PigZombie".to_owned(), "minecraft:zombie_pigman".to_owned());
        map.insert("PolarBear".to_owned(), "minecraft:polar_bear".to_owned());
        map.insert("PrimedTnt".to_owned(), "minecraft:tnt".to_owned());
        map.insert("Rabbit".to_owned(), "minecraft:rabbit".to_owned());
        map.insert("Sheep".to_owned(), "minecraft:sheep".to_owned());
        map.insert("Shulker".to_owned(), "minecraft:shulker".to_owned());
        map.insert("ShulkerBullet".to_owned(), "minecraft:shulker_bullet".to_owned());
        map.insert("Silverfish".to_owned(), "minecraft:silverfish".to_owned());
        map.insert("Skeleton".to_owned(), "minecraft:skeleton".to_owned());
        map.insert("SkeletonHorse".to_owned(), "minecraft:skeleton_horse".to_owned());
        map.insert("Slime".to_owned(), "minecraft:slime".to_owned());
        map.insert("SmallFireball".to_owned(), "minecraft:small_fireball".to_owned());
        map.insert("SnowMan".to_owned(), "minecraft:snowman".to_owned());
        map.insert("Snowball".to_owned(), "minecraft:snowball".to_owned());
        map.insert("SpectralArrow".to_owned(), "minecraft:spectral_arrow".to_owned());
        map.insert("Spider".to_owned(), "minecraft:spider".to_owned());
        map.insert("Squid".to_owned(), "minecraft:squid".to_owned());
        map.insert("Stray".to_owned(), "minecraft:stray".to_owned());
        map.insert("ThrownEgg".to_owned(), "minecraft:egg".to_owned());
        map.insert("ThrownEnderpearl".to_owned(), "minecraft:ender_pearl".to_owned());
        map.insert("ThrownExpBottle".to_owned(), "minecraft:xp_bottle".to_owned());
        map.insert("ThrownPotion".to_owned(), "minecraft:potion".to_owned());
        map.insert("Villager".to_owned(), "minecraft:villager".to_owned());
        map.insert("VillagerGolem".to_owned(), "minecraft:villager_golem".to_owned());
        map.insert("Witch".to_owned(), "minecraft:witch".to_owned());
        map.insert("WitherBoss".to_owned(), "minecraft:wither".to_owned());
        map.insert("WitherSkeleton".to_owned(), "minecraft:wither_skeleton".to_owned());
        map.insert("WitherSkull".to_owned(), "minecraft:wither_skull".to_owned());
        map.insert("Wolf".to_owned(), "minecraft:wolf".to_owned());
        map.insert("XPOrb".to_owned(), "minecraft:xp_orb".to_owned());
        map.insert("Zombie".to_owned(), "minecraft:zombie".to_owned());
        map.insert("ZombieHorse".to_owned(), "minecraft:zombie_horse".to_owned());
        map.insert("ZombieVillager".to_owned(), "minecraft:zombie_villager".to_owned());
        map
    })
}

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    rename_entity(types, VERSION, |id| entity_id_update().get(id).cloned());

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
    types.entity.borrow_mut().add_walker_for_id(VERSION, "minecraft:spawner_minecart", data_walker::<T, _>(move |data, from_version, to_version| {
        untagged_spawner_type.convert(data, from_version, to_version);
    }));
    register_throwable_projectile(types, "minecraft:spectral_arrow");
    register_mob(types, "minecraft:spider");
    register_mob(types, "minecraft:squid");
    register_mob(types, "minecraft:stray");
    types.entity.borrow_mut().add_walker_for_id(VERSION, "minecraft:tnt_minecart", DataWalkerObjectTypePaths::new(types.block_name, "DisplayTile"));
    types.entity.borrow_mut().add_walker_for_id(VERSION, "minecraft:villager", DataWalkerMapListPaths::new_multi(types.item_stack, vec!["Inventory".to_owned(), "ArmorItems".to_owned(), "HandItems".to_owned()]));
    let item_stack_type = types.item_stack;
    types.entity.borrow_mut().add_walker_for_id(VERSION, "minecraft:villager", data_walker::<T, _>(move |data, from_version, to_version| {
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
    types.entity.borrow_mut().add_structure_hook(VERSION, DataHookEnforceNamespacedId::<T>::id());
    types.entity_name.borrow_mut().add_structure_hook(VERSION, DataHookValueTypeEnforceNamespaced::<T>::new());
}

fn register_mob<T: Types + ?Sized>(types: &MinecraftTypesMut<T>, id: impl Into<String>) {
    types.entity.borrow_mut().add_walker_for_id(VERSION, id, DataWalkerMapListPaths::new_multi(types.item_stack, vec!["ArmorItems".to_owned(), "HandItems".to_owned()]));
}

fn register_throwable_projectile<T: Types + ?Sized>(types: &MinecraftTypesMut<T>, id: impl Into<String>) {
    types.entity.borrow_mut().add_walker_for_id(VERSION, id, DataWalkerObjectTypePaths::new(types.block_name, "inTile"));
}
