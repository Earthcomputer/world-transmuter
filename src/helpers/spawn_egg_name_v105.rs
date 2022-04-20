use std::lazy::SyncOnceCell;

static SPAWN_EGG_NAMES: SyncOnceCell<rust_dataconverter_engine::Map<u8, String>> = SyncOnceCell::new();

fn spawn_egg_names() -> &'static rust_dataconverter_engine::Map<u8, String> {
    SPAWN_EGG_NAMES.get_or_init(|| {
        let mut map = rust_dataconverter_engine::Map::new();
        map.insert(1, "Item".to_owned());
        map.insert(2, "XPOrb".to_owned());
        map.insert(7, "ThrownEgg".to_owned());
        map.insert(8, "LeashKnot".to_owned());
        map.insert(9, "Painting".to_owned());
        map.insert(10, "Arrow".to_owned());
        map.insert(11, "Snowball".to_owned());
        map.insert(12, "Fireball".to_owned());
        map.insert(13, "SmallFireball".to_owned());
        map.insert(14, "ThrownEnderpearl".to_owned());
        map.insert(15, "EyeOfEnderSignal".to_owned());
        map.insert(16, "ThrownPotion".to_owned());
        map.insert(17, "ThrownExpBottle".to_owned());
        map.insert(18, "ItemFrame".to_owned());
        map.insert(19, "WitherSkull".to_owned());
        map.insert(20, "PrimedTnt".to_owned());
        map.insert(21, "FallingSand".to_owned());
        map.insert(22, "FireworksRocketEntity".to_owned());
        map.insert(23, "TippedArrow".to_owned());
        map.insert(24, "SpectralArrow".to_owned());
        map.insert(25, "ShulkerBullet".to_owned());
        map.insert(26, "DragonFireball".to_owned());
        map.insert(30, "ArmorStand".to_owned());
        map.insert(41, "Boat".to_owned());
        map.insert(42, "MinecartRideable".to_owned());
        map.insert(43, "MinecartChest".to_owned());
        map.insert(44, "MinecartFurnace".to_owned());
        map.insert(45, "MinecartTNT".to_owned());
        map.insert(46, "MinecartHopper".to_owned());
        map.insert(47, "MinecartSpawner".to_owned());
        map.insert(40, "MinecartCommandBlock".to_owned());
        map.insert(48, "Mob".to_owned());
        map.insert(49, "Monster".to_owned());
        map.insert(50, "Creeper".to_owned());
        map.insert(51, "Skeleton".to_owned());
        map.insert(52, "Spider".to_owned());
        map.insert(53, "Giant".to_owned());
        map.insert(54, "Zombie".to_owned());
        map.insert(55, "Slime".to_owned());
        map.insert(56, "Ghast".to_owned());
        map.insert(57, "PigZombie".to_owned());
        map.insert(58, "Enderman".to_owned());
        map.insert(59, "CaveSpider".to_owned());
        map.insert(60, "Silverfish".to_owned());
        map.insert(61, "Blaze".to_owned());
        map.insert(62, "LavaSlime".to_owned());
        map.insert(63, "EnderDragon".to_owned());
        map.insert(64, "WitherBoss".to_owned());
        map.insert(65, "Bat".to_owned());
        map.insert(66, "Witch".to_owned());
        map.insert(67, "Endermite".to_owned());
        map.insert(68, "Guardian".to_owned());
        map.insert(69, "Shulker".to_owned());
        map.insert(90, "Pig".to_owned());
        map.insert(91, "Sheep".to_owned());
        map.insert(92, "Cow".to_owned());
        map.insert(93, "Chicken".to_owned());
        map.insert(94, "Squid".to_owned());
        map.insert(95, "Wolf".to_owned());
        map.insert(96, "MushroomCow".to_owned());
        map.insert(97, "SnowMan".to_owned());
        map.insert(98, "Ozelot".to_owned());
        map.insert(99, "VillagerGolem".to_owned());
        map.insert(100, "EntityHorse".to_owned());
        map.insert(101, "Rabbit".to_owned());
        map.insert(120, "Villager".to_owned());
        map.insert(200, "EnderCrystal".to_owned());
        map
    })
}

pub(crate) fn get_spawn_name_from_id(id: u8) -> Option<&'static String> {
    spawn_egg_names().get(&id)
}
