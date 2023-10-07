use crate::helpers::{block_flattening_v1450, flatten_item_stack_v1451};
use crate::versions::v1451::pack_with_dot;
use std::collections::{BTreeMap, BTreeSet};
use std::sync::OnceLock;
use valence_nbt::{Compound, Value};
use world_transmuter_engine::{DataVersion, MapDataConverterFunc};

static SPECIAL_OBJECTIVE_CRITERIA: OnceLock<BTreeSet<&str>> = OnceLock::new();

fn special_objective_criteria() -> &'static BTreeSet<&'static str> {
    SPECIAL_OBJECTIVE_CRITERIA.get_or_init(|| {
        let mut set = BTreeSet::new();
        set.insert("dummy");
        set.insert("trigger");
        set.insert("deathCount");
        set.insert("playerKillCount");
        set.insert("totalKillCount");
        set.insert("health");
        set.insert("food");
        set.insert("air");
        set.insert("armor");
        set.insert("xp");
        set.insert("level");
        set.insert("killedByTeam.aqua");
        set.insert("killedByTeam.black");
        set.insert("killedByTeam.blue");
        set.insert("killedByTeam.dark_aqua");
        set.insert("killedByTeam.dark_blue");
        set.insert("killedByTeam.dark_gray");
        set.insert("killedByTeam.dark_green");
        set.insert("killedByTeam.dark_purple");
        set.insert("killedByTeam.dark_red");
        set.insert("killedByTeam.gold");
        set.insert("killedByTeam.gray");
        set.insert("killedByTeam.green");
        set.insert("killedByTeam.light_purple");
        set.insert("killedByTeam.red");
        set.insert("killedByTeam.white");
        set.insert("killedByTeam.yellow");
        set.insert("teamkill.aqua");
        set.insert("teamkill.black");
        set.insert("teamkill.blue");
        set.insert("teamkill.dark_aqua");
        set.insert("teamkill.dark_blue");
        set.insert("teamkill.dark_gray");
        set.insert("teamkill.dark_green");
        set.insert("teamkill.dark_purple");
        set.insert("teamkill.dark_red");
        set.insert("teamkill.gold");
        set.insert("teamkill.gray");
        set.insert("teamkill.green");
        set.insert("teamkill.light_purple");
        set.insert("teamkill.red");
        set.insert("teamkill.white");
        set.insert("teamkill.yellow");
        set
    })
}

static SKIP_STATS: OnceLock<BTreeSet<&'static str>> = OnceLock::new();

fn skip_stats() -> &'static BTreeSet<&'static str> {
    SKIP_STATS.get_or_init(|| {
        let mut set = BTreeSet::new();
        set.insert("stat.craftItem.minecraft.spawn_egg");
        set.insert("stat.useItem.minecraft.spawn_egg");
        set.insert("stat.breakItem.minecraft.spawn_egg");
        set.insert("stat.pickup.minecraft.spawn_egg");
        set.insert("stat.drop.minecraft.spawn_egg");
        set
    })
}

static CUSTOM_STATS: OnceLock<BTreeMap<&'static str, &'static str>> = OnceLock::new();

fn custom_stats() -> &'static BTreeMap<&'static str, &'static str> {
    CUSTOM_STATS.get_or_init(|| {
        let mut map = BTreeMap::new();
        map.insert("stat.leaveGame", "minecraft:leave_game");
        map.insert("stat.playOneMinute", "minecraft:play_one_minute");
        map.insert("stat.timeSinceDeath", "minecraft:time_since_death");
        map.insert("stat.sneakTime", "minecraft:sneak_time");
        map.insert("stat.walkOneCm", "minecraft:walk_one_cm");
        map.insert("stat.crouchOneCm", "minecraft:crouch_one_cm");
        map.insert("stat.sprintOneCm", "minecraft:sprint_one_cm");
        map.insert("stat.swimOneCm", "minecraft:swim_one_cm");
        map.insert("stat.fallOneCm", "minecraft:fall_one_cm");
        map.insert("stat.climbOneCm", "minecraft:climb_one_cm");
        map.insert("stat.flyOneCm", "minecraft:fly_one_cm");
        map.insert("stat.diveOneCm", "minecraft:dive_one_cm");
        map.insert("stat.minecartOneCm", "minecraft:minecart_one_cm");
        map.insert("stat.boatOneCm", "minecraft:boat_one_cm");
        map.insert("stat.pigOneCm", "minecraft:pig_one_cm");
        map.insert("stat.horseOneCm", "minecraft:horse_one_cm");
        map.insert("stat.aviateOneCm", "minecraft:aviate_one_cm");
        map.insert("stat.jump", "minecraft:jump");
        map.insert("stat.drop", "minecraft:drop");
        map.insert("stat.damageDealt", "minecraft:damage_dealt");
        map.insert("stat.damageTaken", "minecraft:damage_taken");
        map.insert("stat.deaths", "minecraft:deaths");
        map.insert("stat.mobKills", "minecraft:mob_kills");
        map.insert("stat.animalsBred", "minecraft:animals_bred");
        map.insert("stat.playerKills", "minecraft:player_kills");
        map.insert("stat.fishCaught", "minecraft:fish_caught");
        map.insert("stat.talkedToVillager", "minecraft:talked_to_villager");
        map.insert("stat.tradedWithVillager", "minecraft:traded_with_villager");
        map.insert("stat.cakeSlicesEaten", "minecraft:eat_cake_slice");
        map.insert("stat.cauldronFilled", "minecraft:fill_cauldron");
        map.insert("stat.cauldronUsed", "minecraft:use_cauldron");
        map.insert("stat.armorCleaned", "minecraft:clean_armor");
        map.insert("stat.bannerCleaned", "minecraft:clean_banner");
        map.insert(
            "stat.brewingstandInteraction",
            "minecraft:interact_with_brewingstand",
        );
        map.insert("stat.beaconInteraction", "minecraft:interact_with_beacon");
        map.insert("stat.dropperInspected", "minecraft:inspect_dropper");
        map.insert("stat.hopperInspected", "minecraft:inspect_hopper");
        map.insert("stat.dispenserInspected", "minecraft:inspect_dispenser");
        map.insert("stat.noteblockPlayed", "minecraft:play_noteblock");
        map.insert("stat.noteblockTuned", "minecraft:tune_noteblock");
        map.insert("stat.flowerPotted", "minecraft:pot_flower");
        map.insert(
            "stat.trappedChestTriggered",
            "minecraft:trigger_trapped_chest",
        );
        map.insert("stat.enderchestOpened", "minecraft:open_enderchest");
        map.insert("stat.itemEnchanted", "minecraft:enchant_item");
        map.insert("stat.recordPlayed", "minecraft:play_record");
        map.insert("stat.furnaceInteraction", "minecraft:interact_with_furnace");
        map.insert(
            "stat.craftingTableInteraction",
            "minecraft:interact_with_crafting_table",
        );
        map.insert("stat.chestOpened", "minecraft:open_chest");
        map.insert("stat.sleepInBed", "minecraft:sleep_in_bed");
        map.insert("stat.shulkerBoxOpened", "minecraft:open_shulker_box");
        map
    })
}

static ITEM_STATS: OnceLock<BTreeMap<&'static str, &'static str>> = OnceLock::new();

fn item_stats() -> &'static BTreeMap<&'static str, &'static str> {
    ITEM_STATS.get_or_init(|| {
        let mut map = BTreeMap::new();
        map.insert("stat.craftItem", "minecraft:crafted");
        map.insert("stat.useItem", "minecraft:used");
        map.insert("stat.breakItem", "minecraft:broken");
        map.insert("stat.pickup", "minecraft:picked_up");
        map.insert("stat.drop", "minecraft:dropped");
        map
    })
}

static ENTITY_STATS: OnceLock<BTreeMap<&'static str, &'static str>> = OnceLock::new();

fn entity_stats() -> &'static BTreeMap<&'static str, &'static str> {
    ENTITY_STATS.get_or_init(|| {
        let mut map = BTreeMap::new();
        map.insert("stat.entityKilledBy", "minecraft:killed_by");
        map.insert("stat.killEntity", "minecraft:killed");
        map
    })
}

static ENTITY_MAP: OnceLock<BTreeMap<&'static str, &'static str>> = OnceLock::new();

fn entity_map() -> &'static BTreeMap<&'static str, &'static str> {
    ENTITY_MAP.get_or_init(|| {
        let mut map = BTreeMap::new();
        map.insert("Bat", "minecraft:bat");
        map.insert("Blaze", "minecraft:blaze");
        map.insert("CaveSpider", "minecraft:cave_spider");
        map.insert("Chicken", "minecraft:chicken");
        map.insert("Cow", "minecraft:cow");
        map.insert("Creeper", "minecraft:creeper");
        map.insert("Donkey", "minecraft:donkey");
        map.insert("ElderGuardian", "minecraft:elder_guardian");
        map.insert("Enderman", "minecraft:enderman");
        map.insert("Endermite", "minecraft:endermite");
        map.insert("EvocationIllager", "minecraft:evocation_illager");
        map.insert("Ghast", "minecraft:ghast");
        map.insert("Guardian", "minecraft:guardian");
        map.insert("Horse", "minecraft:horse");
        map.insert("Husk", "minecraft:husk");
        map.insert("Llama", "minecraft:llama");
        map.insert("LavaSlime", "minecraft:magma_cube");
        map.insert("MushroomCow", "minecraft:mooshroom");
        map.insert("Mule", "minecraft:mule");
        map.insert("Ozelot", "minecraft:ocelot");
        map.insert("Parrot", "minecraft:parrot");
        map.insert("Pig", "minecraft:pig");
        map.insert("PolarBear", "minecraft:polar_bear");
        map.insert("Rabbit", "minecraft:rabbit");
        map.insert("Sheep", "minecraft:sheep");
        map.insert("Shulker", "minecraft:shulker");
        map.insert("Silverfish", "minecraft:silverfish");
        map.insert("SkeletonHorse", "minecraft:skeleton_horse");
        map.insert("Skeleton", "minecraft:skeleton");
        map.insert("Slime", "minecraft:slime");
        map.insert("Spider", "minecraft:spider");
        map.insert("Squid", "minecraft:squid");
        map.insert("Stray", "minecraft:stray");
        map.insert("Vex", "minecraft:vex");
        map.insert("Villager", "minecraft:villager");
        map.insert("VindicationIllager", "minecraft:vindication_illager");
        map.insert("Witch", "minecraft:witch");
        map.insert("WitherSkeleton", "minecraft:wither_skeleton");
        map.insert("Wolf", "minecraft:wolf");
        map.insert("ZombieHorse", "minecraft:zombie_horse");
        map.insert("PigZombie", "minecraft:zombie_pigman");
        map.insert("ZombieVillager", "minecraft:zombie_villager");
        map.insert("Zombie", "minecraft:zombie");
        map
    })
}

struct StatType<'a> {
    category: &'a str,
    key: String,
}

impl<'a> StatType<'a> {
    fn new(category: &'a str, key: String) -> Self {
        StatType { category, key }
    }
}

fn convert_legacy_key(key: &str) -> Option<StatType> {
    if skip_stats().contains(key) {
        return None;
    }

    if let Some(stat_key) = custom_stats().get(key).copied() {
        Some(StatType::new("minecraft:custom", stat_key.to_owned()))
    } else {
        let split_index = match key.match_indices('.').nth(1) {
            Some((index, _)) => index,
            None => return None,
        };

        let (key, item) = key.split_at(split_index);
        let item = item.strip_prefix('.').unwrap().replace('.', ":");

        #[allow(clippy::manual_map)]
        if key == "stat.mineBlock" {
            Some(StatType::new(
                "minecraft:mined",
                block_flattening_v1450::get_new_block_name(&item).to_owned(),
            ))
        } else if let Some(stat_key) = item_stats().get(key).copied() {
            Some(StatType::new(
                stat_key,
                flatten_item_stack_v1451::flatten_item(&item, 0)
                    .map(|str| str.to_owned())
                    .unwrap_or(item),
            ))
        } else if let Some(stat_key) = entity_stats().get(key).copied() {
            Some(StatType::new(
                stat_key,
                entity_map()
                    .get(&item[..])
                    .copied()
                    .map(|str| str.to_owned())
                    .unwrap_or(item),
            ))
        } else {
            None
        }
    }
}

pub(crate) struct StatsConverter;

impl MapDataConverterFunc for StatsConverter {
    fn convert(&self, data: &mut Compound, _from_version: DataVersion, _to_version: DataVersion) {
        let mut stats = Compound::new();

        for (stat_key, value) in data.iter_mut() {
            let Some(value) = value.as_i32() else {
                continue;
            };

            let Some(converted) = convert_legacy_key(stat_key) else {
                continue;
            };

            if !stats.contains_key(converted.category) {
                stats.insert(converted.category, Compound::new());
            }
            let Some(Value::Compound(stat_type_map)) = stats.get_mut(converted.category) else {
                unreachable!()
            };
            stat_type_map.insert(converted.key, value);
        }

        data.clear();
        data.insert("stats", stats);
    }
}

pub(crate) struct ObjectiveConverter;

impl MapDataConverterFunc for ObjectiveConverter {
    fn convert(&self, data: &mut Compound, _from_version: DataVersion, _to_version: DataVersion) {
        let Some(Value::String(criteria_name)) = data.get_mut("CriteriaName") else {
            return;
        };

        if special_objective_criteria().contains(&criteria_name[..]) {
            return;
        }

        let converted = convert_legacy_key(criteria_name);
        *criteria_name = converted
            .map(|converted| {
                format!(
                    "{}:{}",
                    pack_with_dot(converted.category),
                    pack_with_dot(&converted.key)
                )
            })
            .unwrap_or_else(|| "dummy".to_owned());
    }
}
