use crate::helpers::{block_flattening_v1450, flatten_item_stack_v1451};
use crate::versions::v1451::pack_with_dot;
use crate::{static_string_map, static_string_set};
use java_string::{format_java, JavaStr, JavaString};
use world_transmuter_engine::{DataVersion, JCompound, JValue, MapDataConverterFunc};

static_string_set! {
    SPECIAL_OBJECTIVE_CRITERIA, special_objective_criteria, {
        "dummy",
        "trigger",
        "deathCount",
        "playerKillCount",
        "totalKillCount",
        "health",
        "food",
        "air",
        "armor",
        "xp",
        "level",
        "killedByTeam.aqua",
        "killedByTeam.black",
        "killedByTeam.blue",
        "killedByTeam.dark_aqua",
        "killedByTeam.dark_blue",
        "killedByTeam.dark_gray",
        "killedByTeam.dark_green",
        "killedByTeam.dark_purple",
        "killedByTeam.dark_red",
        "killedByTeam.gold",
        "killedByTeam.gray",
        "killedByTeam.green",
        "killedByTeam.light_purple",
        "killedByTeam.red",
        "killedByTeam.white",
        "killedByTeam.yellow",
        "teamkill.aqua",
        "teamkill.black",
        "teamkill.blue",
        "teamkill.dark_aqua",
        "teamkill.dark_blue",
        "teamkill.dark_gray",
        "teamkill.dark_green",
        "teamkill.dark_purple",
        "teamkill.dark_red",
        "teamkill.gold",
        "teamkill.gray",
        "teamkill.green",
        "teamkill.light_purple",
        "teamkill.red",
        "teamkill.white",
        "teamkill.yellow",
    }
}

static_string_set! {
    SKIP_STATS, skip_stats, {
        "stat.craftItem.minecraft.spawn_egg",
        "stat.useItem.minecraft.spawn_egg",
        "stat.breakItem.minecraft.spawn_egg",
        "stat.pickup.minecraft.spawn_egg",
        "stat.drop.minecraft.spawn_egg",
    }
}

static_string_map! {
    CUSTOM_STATS, custom_stats, {
        "stat.leaveGame" => "minecraft:leave_game",
        "stat.playOneMinute" => "minecraft:play_one_minute",
        "stat.timeSinceDeath" => "minecraft:time_since_death",
        "stat.sneakTime" => "minecraft:sneak_time",
        "stat.walkOneCm" => "minecraft:walk_one_cm",
        "stat.crouchOneCm" => "minecraft:crouch_one_cm",
        "stat.sprintOneCm" => "minecraft:sprint_one_cm",
        "stat.swimOneCm" => "minecraft:swim_one_cm",
        "stat.fallOneCm" => "minecraft:fall_one_cm",
        "stat.climbOneCm" => "minecraft:climb_one_cm",
        "stat.flyOneCm" => "minecraft:fly_one_cm",
        "stat.diveOneCm" => "minecraft:dive_one_cm",
        "stat.minecartOneCm" => "minecraft:minecart_one_cm",
        "stat.boatOneCm" => "minecraft:boat_one_cm",
        "stat.pigOneCm" => "minecraft:pig_one_cm",
        "stat.horseOneCm" => "minecraft:horse_one_cm",
        "stat.aviateOneCm" => "minecraft:aviate_one_cm",
        "stat.jump" => "minecraft:jump",
        "stat.drop" => "minecraft:drop",
        "stat.damageDealt" => "minecraft:damage_dealt",
        "stat.damageTaken" => "minecraft:damage_taken",
        "stat.deaths" => "minecraft:deaths",
        "stat.mobKills" => "minecraft:mob_kills",
        "stat.animalsBred" => "minecraft:animals_bred",
        "stat.playerKills" => "minecraft:player_kills",
        "stat.fishCaught" => "minecraft:fish_caught",
        "stat.talkedToVillager" => "minecraft:talked_to_villager",
        "stat.tradedWithVillager" => "minecraft:traded_with_villager",
        "stat.cakeSlicesEaten" => "minecraft:eat_cake_slice",
        "stat.cauldronFilled" => "minecraft:fill_cauldron",
        "stat.cauldronUsed" => "minecraft:use_cauldron",
        "stat.armorCleaned" => "minecraft:clean_armor",
        "stat.bannerCleaned" => "minecraft:clean_banner",
        "stat.brewingstandInteraction" => "minecraft:interact_with_brewingstand",
        "stat.beaconInteraction" => "minecraft:interact_with_beacon",
        "stat.dropperInspected" => "minecraft:inspect_dropper",
        "stat.hopperInspected" => "minecraft:inspect_hopper",
        "stat.dispenserInspected" => "minecraft:inspect_dispenser",
        "stat.noteblockPlayed" => "minecraft:play_noteblock",
        "stat.noteblockTuned" => "minecraft:tune_noteblock",
        "stat.flowerPotted" => "minecraft:pot_flower",
        "stat.trappedChestTriggered" => "minecraft:trigger_trapped_chest",
        "stat.enderchestOpened" => "minecraft:open_enderchest",
        "stat.itemEnchanted" => "minecraft:enchant_item",
        "stat.recordPlayed" => "minecraft:play_record",
        "stat.furnaceInteraction" => "minecraft:interact_with_furnace",
        "stat.craftingTableInteraction" => "minecraft:interact_with_crafting_table",
        "stat.chestOpened" => "minecraft:open_chest",
        "stat.sleepInBed" => "minecraft:sleep_in_bed",
        "stat.shulkerBoxOpened" => "minecraft:open_shulker_box",
    }
}

static_string_map! {
    ITEM_STATS, item_stats, {
        "stat.craftItem" => "minecraft:crafted",
        "stat.useItem" => "minecraft:used",
        "stat.breakItem" => "minecraft:broken",
        "stat.pickup" => "minecraft:picked_up",
        "stat.drop" => "minecraft:dropped",
    }
}

static_string_map! {
    ENTITY_STATS, entity_stats, {
        "stat.entityKilledBy" => "minecraft:killed_by",
        "stat.killEntity" => "minecraft:killed",
    }
}

static_string_map! {
    ENTITY_MAP, entity_map, {
        "Bat" => "minecraft:bat",
        "Blaze" => "minecraft:blaze",
        "CaveSpider" => "minecraft:cave_spider",
        "Chicken" => "minecraft:chicken",
        "Cow" => "minecraft:cow",
        "Creeper" => "minecraft:creeper",
        "Donkey" => "minecraft:donkey",
        "ElderGuardian" => "minecraft:elder_guardian",
        "Enderman" => "minecraft:enderman",
        "Endermite" => "minecraft:endermite",
        "EvocationIllager" => "minecraft:evocation_illager",
        "Ghast" => "minecraft:ghast",
        "Guardian" => "minecraft:guardian",
        "Horse" => "minecraft:horse",
        "Husk" => "minecraft:husk",
        "Llama" => "minecraft:llama",
        "LavaSlime" => "minecraft:magma_cube",
        "MushroomCow" => "minecraft:mooshroom",
        "Mule" => "minecraft:mule",
        "Ozelot" => "minecraft:ocelot",
        "Parrot" => "minecraft:parrot",
        "Pig" => "minecraft:pig",
        "PolarBear" => "minecraft:polar_bear",
        "Rabbit" => "minecraft:rabbit",
        "Sheep" => "minecraft:sheep",
        "Shulker" => "minecraft:shulker",
        "Silverfish" => "minecraft:silverfish",
        "SkeletonHorse" => "minecraft:skeleton_horse",
        "Skeleton" => "minecraft:skeleton",
        "Slime" => "minecraft:slime",
        "Spider" => "minecraft:spider",
        "Squid" => "minecraft:squid",
        "Stray" => "minecraft:stray",
        "Vex" => "minecraft:vex",
        "Villager" => "minecraft:villager",
        "VindicationIllager" => "minecraft:vindication_illager",
        "Witch" => "minecraft:witch",
        "WitherSkeleton" => "minecraft:wither_skeleton",
        "Wolf" => "minecraft:wolf",
        "ZombieHorse" => "minecraft:zombie_horse",
        "PigZombie" => "minecraft:zombie_pigman",
        "ZombieVillager" => "minecraft:zombie_villager",
        "Zombie" => "minecraft:zombie",
    }
}

struct StatType<'a> {
    category: &'a JavaStr,
    key: JavaString,
}

impl<'a> StatType<'a> {
    fn new(category: &'a JavaStr, key: JavaString) -> Self {
        StatType { category, key }
    }
}

fn convert_legacy_key(key: &JavaStr) -> Option<StatType> {
    if skip_stats().contains(key) {
        return None;
    }

    if let Some(stat_key) = custom_stats().get(key).copied() {
        Some(StatType::new(
            JavaStr::from_str("minecraft:custom"),
            stat_key.to_owned(),
        ))
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
                JavaStr::from_str("minecraft:mined"),
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
    fn convert(&self, data: &mut JCompound, _from_version: DataVersion, _to_version: DataVersion) {
        let mut stats = JCompound::new();

        for (stat_key, value) in data.iter_mut() {
            let Some(value) = value.as_i32() else {
                continue;
            };

            let Some(converted) = convert_legacy_key(stat_key) else {
                continue;
            };

            if !stats.contains_key(converted.category) {
                stats.insert(converted.category, JCompound::new());
            }
            let Some(JValue::Compound(stat_type_map)) = stats.get_mut(converted.category) else {
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
    fn convert(&self, data: &mut JCompound, _from_version: DataVersion, _to_version: DataVersion) {
        let Some(JValue::String(criteria_name)) = data.get_mut("CriteriaName") else {
            return;
        };

        if special_objective_criteria().contains(&criteria_name[..]) {
            return;
        }

        let converted = convert_legacy_key(criteria_name);
        *criteria_name = converted
            .map(|converted| {
                format_java!(
                    "{}:{}",
                    pack_with_dot(converted.category),
                    pack_with_dot(&converted.key)
                )
            })
            .unwrap_or_else(|| JavaString::from("dummy"));
    }
}
