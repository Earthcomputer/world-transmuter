use rust_dataconverter_engine::{data_converter_func, ListType, MapType, ObjectType, Types};
use crate::MinecraftTypesMut;

const VERSION: u32 = 2514;

macro_rules! replace_uuid_least_most {
    ($t:ty, $data:expr, $prefix:literal, $new_path:literal) => {
        replace_uuid_from_longs::<$t>($data, concat!($prefix, "Least"), concat!($prefix, "Most"), $new_path)
    }
}

pub(crate) fn replace_uuid_from_longs<T: Types + ?Sized>(data: &mut T::Map, least: &str, most: &str, new_path: &str) {
    let least = data.remove(least).and_then(|o| o.as_i64()).unwrap_or(0);
    let most = data.remove(most).and_then(|o| o.as_i64()).unwrap_or(0);
    if least != 0 || most != 0 {
        data.set(new_path, T::Object::create_int_array(create_uuid_from_longs(least, most)));
    }
}

fn replace_uuid_string<T: Types + ?Sized>(data: &mut T::Map, path: &str, new_path: &str) {
    if let Some(uuid) = data.get_string(path).and_then(|str| str.parse::<uuid::Uuid>().ok()) {
        let (most, least) = uuid.as_u64_pair();
        data.set(new_path, T::Object::create_int_array(create_uuid_from_longs(least as i64, most as i64)));
    }
}

fn replace_uuid_ml_tag<T: Types + ?Sized>(data: &mut T::Map, path: &str, new_path: &str) {
    if let Some(tag) = data.remove(path).and_then(|o| o.into_map()) {
        let least = tag.get_i64("L").unwrap_or(0);
        let most = tag.get_i64("M").unwrap_or(0);
        if least != 0 || most != 0 {
            data.set(new_path, T::Object::create_int_array(create_uuid_from_longs(least, most)));
        }
    }
}

fn create_uuid_from_longs(least: i64, most: i64) -> Vec<i32> {
    vec![
        (most as u64 >> 32) as i32,
        most as i32,
        (least as u64 >> 32) as i32,
        least as i32,
    ]
}

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    // Entity UUID fixes

    types.entity.borrow_mut().add_structure_converter(VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        replace_uuid_least_most!(T, data, "UUID", "UUID");
    }));

    for id in [
        "minecraft:donkey",
        "minecraft:horse",
        "minecraft:llama",
        "minecraft:mule",
        "minecraft:skeleton_horse",
        "minecraft:trader_llama",
        "minecraft:zombie_horse",
        "minecraft:cat",
        "minecraft:parrot",
        "minecraft:wolf",
    ] {
        types.entity.borrow_mut().add_converter_for_id(id, VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
            update_animal_owner::<T>(data);
        }));
    }

    for id in [
        "minecraft:bee",
        "minecraft:chicken",
        "minecraft:cow",
        "minecraft:fox",
        "minecraft:mooshroom",
        "minecraft:ocelot",
        "minecraft:panda",
        "minecraft:pig",
        "minecraft:polar_bear",
        "minecraft:rabbit",
        "minecraft:sheep",
        "minecraft:turtle",
        "minecraft:hoglin",
    ] {
        types.entity.borrow_mut().add_converter_for_id(id, VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
            update_animal::<T>(data);
        }));
    }

    for id in [
        "minecraft:bat",
        "minecraft:blaze",
        "minecraft:cave_spider",
        "minecraft:cod",
        "minecraft:creeper",
        "minecraft:dolphin",
        "minecraft:drowned",
        "minecraft:elder_guardian",
        "minecraft:ender_dragon",
        "minecraft:enderman",
        "minecraft:endermite",
        "minecraft:evoker",
        "minecraft:ghast",
        "minecraft:giant",
        "minecraft:guardian",
        "minecraft:husk",
        "minecraft:illusioner",
        "minecraft:magma_cube",
        "minecraft:pufferfish",
        "minecraft:zombified_piglin",
        "minecraft:salmon",
        "minecraft:shulker",
        "minecraft:silverfish",
        "minecraft:skeleton",
        "minecraft:slime",
        "minecraft:snow_golem",
        "minecraft:spider",
        "minecraft:squid",
        "minecraft:stray",
        "minecraft:tropical_fish",
        "minecraft:vex",
        "minecraft:villager",
        "minecraft:iron_golem",
        "minecraft:vindicator",
        "minecraft:pillager",
        "minecraft:wandering_trader",
        "minecraft:witch",
        "minecraft:wither",
        "minecraft:wither_skeleton",
        "minecraft:zombie",
        "minecraft:zombie_villager",
        "minecraft:phantom",
        "minecraft:ravager",
        "minecraft:piglin",
    ] {
        types.entity.borrow_mut().add_converter_for_id(id, VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
            update_mob::<T>(data);
        }));
    }

    types.entity.borrow_mut().add_converter_for_id("minecraft:armor_stand", VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        update_living_entity::<T>(data);
    }));

    for id in [
        "minecraft:arrow",
        "minecraft:dragon_fireball",
        "minecraft:firework_rocket",
        "minecraft:fireball",
        "minecraft:llama_spit",
        "minecraft:small_fireball",
        "minecraft:snowball",
        "minecraft:spectral_arrow",
        "minecraft:egg",
        "minecraft:ender_pearl",
        "minecraft:experience_bottle",
        "minecraft:potion",
        "minecraft:trident",
        "minecraft:wither_skull",
    ] {
        types.entity.borrow_mut().add_converter_for_id(id, VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
            update_projectile::<T>(data);
        }));
    }

    for id in [
        "minecraft:bee",
        "minecraft:zombified_piglin"
    ] {
        types.entity.borrow_mut().add_converter_for_id(id, VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
            update_hurt_by::<T>(data);
        }));
    }

    types.entity.borrow_mut().add_converter_for_id("minecraft:fox", VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        update_fox::<T>(data);
    }));
    types.entity.borrow_mut().add_converter_for_id("minecraft:item", VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        update_item::<T>(data);
    }));
    types.entity.borrow_mut().add_converter_for_id("minecraft:shulker_bullet", VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        update_shulker_bullet::<T>(data);
    }));
    types.entity.borrow_mut().add_converter_for_id("minecraft:area_effect_cloud", VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        update_area_effect_cloud::<T>(data);
    }));
    types.entity.borrow_mut().add_converter_for_id("minecraft:zombie_villager", VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        update_zombie_villager::<T>(data);
    }));
    types.entity.borrow_mut().add_converter_for_id("minecraft:evoker_fangs", VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        update_evoker_fangs::<T>(data);
    }));
    types.entity.borrow_mut().add_converter_for_id("minecraft:piglin", VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        update_piglin::<T>(data);
    }));

    // Update TE
    types.tile_entity.borrow_mut().add_converter_for_id("minecraft:conduit", VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        replace_uuid_ml_tag::<T>(data, "target_uuid", "Target");
    }));
    types.tile_entity.borrow_mut().add_converter_for_id("minecraft:skull", VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        if let Some(mut owner) = data.remove("Owner").and_then(|o| o.into_map()) {
            replace_uuid_string::<T>(&mut owner, "Id", "Id");
            data.set("SkullOwner", T::Object::create_map(owner));
        }
    }));

    // Player UUID
    types.player.borrow_mut().add_structure_converter(VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        update_living_entity::<T>(data);
        replace_uuid_least_most!(T, data, "UUID", "UUID");

        if let Some(root_vehicle) = data.get_map_mut("RootVehicle") {
            replace_uuid_least_most!(T, root_vehicle, "Attach", "Attach");
        }
    }));

    // Level.dat
    types.level.borrow_mut().add_structure_converter(VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        replace_uuid_string::<T>(data, "WanderingTraderId", "WanderingTraderId");

        if let Some(dimension_data) = data.get_map_mut("DimensionData") {
            for dimension_data in dimension_data.values_mut() {
                if let Some(dragon_fight) = dimension_data.as_map_mut().and_then(|o| o.get_map_mut("DragonFight")) {
                    replace_uuid_least_most!(T, dragon_fight, "DragonUUID", "Dragon");
                }
            }
        }

        if let Some(custom_boss_events) = data.get_map_mut("CustomBossEvents") {
            for custom_boss_event in custom_boss_events.values_mut() {
                if let Some(custom_boss_event) = custom_boss_event.as_map_mut() {
                    if let Some(players) = custom_boss_event.get_list("Players") {
                        let mut new_players = T::List::create_empty();
                        for player in players.iter() {
                            if let Some(player) = player.as_map() {
                                let least = player.get_i64("L").unwrap_or(0);
                                let most = player.get_i64("M").unwrap_or(0);
                                if least != 0 && most != 0 {
                                    new_players.add(T::Object::create_int_array(create_uuid_from_longs(least, most)));
                                }
                            }
                        }
                        custom_boss_event.set("Players", T::Object::create_list(new_players));
                    }
                }
            }
        }
    }));

    types.saved_data.borrow_mut().add_structure_converter(VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        if let Some(data) = data.get_map_mut("data") {
            if let Some(raids) = data.get_list_mut("Raids") {
                for raid in raids.iter_mut() {
                    if let Some(raid) = raid.as_map_mut() {
                        let raid: &mut T::Map = raid;
                        if let Some(heroes) = raid.get_list("HeroesOfTheVillage") {
                            let mut new_heroes = T::List::create_empty();
                            for hero in heroes.iter() {
                                if let Some(hero) = hero.as_map() {
                                    let least = hero.get_i64("UUIDLeast").unwrap_or(0);
                                    let most = hero.get_i64("UUIDMost").unwrap_or(0);
                                    if least != 0 || most != 0 {
                                        new_heroes.add(T::Object::create_int_array(create_uuid_from_longs(least, most)));
                                    }
                                }
                            }
                            raid.set("HeroesOfTheVillage", T::Object::create_list(new_heroes));
                        }
                    }
                }
            }
        }
    }));

    types.item_stack.borrow_mut().add_structure_converter(VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        let is_player_head = data.get_string("id") == Some("minecraft:player_head");

        if let Some(tag) = data.get_map_mut("tag") {
            if let Some(attributes) = tag.get_list_mut("AttributeModifiers") {
                for attribute in attributes.iter_mut() {
                    if let Some(attribute) = attribute.as_map_mut() {
                        replace_uuid_least_most!(T, attribute, "UUID", "UUID");
                    }
                }
            }

            if is_player_head {
                if let Some(skull_owner) = tag.get_map_mut("SkullOwner") {
                    replace_uuid_string::<T>(skull_owner, "Id", "Id");
                }
            }
        }
    }));
}

fn update_animal_owner<T: Types + ?Sized>(data: &mut T::Map) {
    update_animal::<T>(data);

    replace_uuid_string::<T>(data, "OwnerUUID", "Owner");
}

fn update_animal<T: Types + ?Sized>(data: &mut T::Map) {
    update_mob::<T>(data);

    replace_uuid_least_most!(T, data, "LoveCause", "LoveCause");
}

fn update_mob<T: Types + ?Sized>(data: &mut T::Map) {
    update_living_entity::<T>(data);

    if let Some(leash) = data.get_map_mut("Leash") {
        replace_uuid_least_most!(T, leash, "UUID", "UUID");
    }
}

fn update_living_entity<T: Types + ?Sized>(data: &mut T::Map) {
    if let Some(attributes) = data.get_list_mut("Attributes") {
        for attribute in attributes.iter_mut() {
            if let Some(modifiers) = attribute.as_map_mut().and_then(|o| o.get_list_mut("Modifiers")) {
                for modifier in modifiers.iter_mut() {
                    if let Some(modifier) = modifier.as_map_mut() {
                        replace_uuid_least_most!(T, modifier, "UUID", "UUID");
                    }
                }
            }
        }
    }
}

fn update_projectile<T: Types + ?Sized>(data: &mut T::Map) {
    data.rename_key("OwnerUUID", "Owner");
}

fn update_hurt_by<T: Types + ?Sized>(data: &mut T::Map) {
    replace_uuid_string::<T>(data, "HurtBy", "HurtBy");
}

fn update_fox<T: Types + ?Sized>(data: &mut T::Map) {
    if let Some(trusted_uuids) = data.remove("TrustedUUIDs").and_then(|o| o.into_list()) {
        let mut trusted = T::List::create_empty();
        for uuid in trusted_uuids.into_iter() {
            if let Some(uuid) = uuid.into_map() {
                let least = uuid.get_i64("L").unwrap_or(0);
                let most = uuid.get_i64("M").unwrap_or(0);
                if least != 0 || most != 0 {
                    trusted.add(T::Object::create_int_array(create_uuid_from_longs(least, most)));
                }
            }
        }
        data.set("Trusted", T::Object::create_list(trusted));
    }
}

fn update_item<T: Types + ?Sized>(data: &mut T::Map) {
    replace_uuid_ml_tag::<T>(data, "Owner", "Owner");
    replace_uuid_ml_tag::<T>(data, "Thrower", "Thrower");
}

fn update_shulker_bullet<T: Types + ?Sized>(data: &mut T::Map) {
    replace_uuid_ml_tag::<T>(data, "Owner", "Owner");
    replace_uuid_ml_tag::<T>(data, "Target", "Target");
}

fn update_area_effect_cloud<T: Types + ?Sized>(data: &mut T::Map) {
    replace_uuid_least_most!(T, data, "OwnerUUID", "Owner");
}

fn update_zombie_villager<T: Types + ?Sized>(data: &mut T::Map) {
    replace_uuid_least_most!(T, data, "ConversionPlayer", "ConversionPlayer");
}

fn update_evoker_fangs<T: Types + ?Sized>(data: &mut T::Map) {
    replace_uuid_least_most!(T, data, "OwnerUUID", "Owner");
}

fn update_piglin<T: Types + ?Sized>(data: &mut T::Map) {
    let _: Option<_> = try {
        let angry_at = data.get_map_mut("Brain")?.get_map_mut("memories")?.get_map_mut("minecraft:angry_at")?;
        replace_uuid_string::<T>(angry_at, "value", "value");
    };
}
