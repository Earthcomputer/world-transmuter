use ahash::AHashMap;
use std::sync::OnceLock;

fn spawn_egg_names() -> &'static AHashMap<u8, &'static str> {
    static SPAWN_EGG_NAMES: OnceLock<AHashMap<u8, &'static str>> = OnceLock::new();
    SPAWN_EGG_NAMES.get_or_init(|| {
        let mut map = AHashMap::new();
        map.insert(1, "Item");
        map.insert(2, "XPOrb");
        map.insert(7, "ThrownEgg");
        map.insert(8, "LeashKnot");
        map.insert(9, "Painting");
        map.insert(10, "Arrow");
        map.insert(11, "Snowball");
        map.insert(12, "Fireball");
        map.insert(13, "SmallFireball");
        map.insert(14, "ThrownEnderpearl");
        map.insert(15, "EyeOfEnderSignal");
        map.insert(16, "ThrownPotion");
        map.insert(17, "ThrownExpBottle");
        map.insert(18, "ItemFrame");
        map.insert(19, "WitherSkull");
        map.insert(20, "PrimedTnt");
        map.insert(21, "FallingSand");
        map.insert(22, "FireworksRocketEntity");
        map.insert(23, "TippedArrow");
        map.insert(24, "SpectralArrow");
        map.insert(25, "ShulkerBullet");
        map.insert(26, "DragonFireball");
        map.insert(30, "ArmorStand");
        map.insert(41, "Boat");
        map.insert(42, "MinecartRideable");
        map.insert(43, "MinecartChest");
        map.insert(44, "MinecartFurnace");
        map.insert(45, "MinecartTNT");
        map.insert(46, "MinecartHopper");
        map.insert(47, "MinecartSpawner");
        map.insert(40, "MinecartCommandBlock");
        map.insert(50, "Creeper");
        map.insert(51, "Skeleton");
        map.insert(52, "Spider");
        map.insert(53, "Giant");
        map.insert(54, "Zombie");
        map.insert(55, "Slime");
        map.insert(56, "Ghast");
        map.insert(57, "PigZombie");
        map.insert(58, "Enderman");
        map.insert(59, "CaveSpider");
        map.insert(60, "Silverfish");
        map.insert(61, "Blaze");
        map.insert(62, "LavaSlime");
        map.insert(63, "EnderDragon");
        map.insert(64, "WitherBoss");
        map.insert(65, "Bat");
        map.insert(66, "Witch");
        map.insert(67, "Endermite");
        map.insert(68, "Guardian");
        map.insert(69, "Shulker");
        map.insert(90, "Pig");
        map.insert(91, "Sheep");
        map.insert(92, "Cow");
        map.insert(93, "Chicken");
        map.insert(94, "Squid");
        map.insert(95, "Wolf");
        map.insert(96, "MushroomCow");
        map.insert(97, "SnowMan");
        map.insert(98, "Ozelot");
        map.insert(99, "VillagerGolem");
        map.insert(100, "EntityHorse");
        map.insert(101, "Rabbit");
        map.insert(120, "Villager");
        map.insert(200, "EnderCrystal");
        map
    })
}

pub(crate) fn get_spawn_name_from_id(id: u8) -> Option<&'static str> {
    spawn_egg_names().get(&id).copied()
}
