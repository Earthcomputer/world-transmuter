use crate::types;
use world_transmuter_engine::{map_data_converter_func, rename_key, JCompound, JList, JValue};

const VERSION: u32 = 2514;

macro_rules! replace_uuid_least_most {
    ($data:expr, $prefix:literal, $new_path:literal) => {
        replace_uuid_from_longs(
            $data,
            concat!($prefix, "Least"),
            concat!($prefix, "Most"),
            $new_path,
        )
    };
}

pub(crate) fn replace_uuid_from_longs(
    data: &mut JCompound,
    least: &str,
    most: &str,
    new_path: &str,
) {
    let least = data.remove(least).and_then(|o| o.as_i64()).unwrap_or(0);
    let most = data.remove(most).and_then(|o| o.as_i64()).unwrap_or(0);
    if least != 0 || most != 0 {
        data.insert(new_path, create_uuid_from_longs(least, most));
    }
}

fn replace_uuid_string(data: &mut JCompound, path: &str, new_path: &str) {
    let Some(JValue::String(uuid)) = data.get(path) else {
        return;
    };
    let Ok(uuid) = uuid.parse::<uuid::Uuid>() else {
        return;
    };
    let (most, least) = uuid.as_u64_pair();
    data.insert(new_path, create_uuid_from_longs(least as i64, most as i64));
}

fn replace_uuid_ml_tag(data: &mut JCompound, path: &str, new_path: &str) {
    if let Some(JValue::Compound(tag)) = data.remove(path) {
        let least = tag.get("L").and_then(|v| v.as_i64()).unwrap_or(0);
        let most = tag.get("M").and_then(|v| v.as_i64()).unwrap_or(0);
        if least != 0 || most != 0 {
            data.insert(new_path, create_uuid_from_longs(least, most));
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

pub(crate) fn register() {
    // Entity UUID fixes

    types::entity_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            replace_uuid_least_most!(data, "UUID", "UUID");
        }),
    );

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
        types::entity_mut().add_converter_for_id(
            id,
            VERSION,
            map_data_converter_func(|data, _from_version, _to_version| {
                update_animal_owner(data);
            }),
        );
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
        types::entity_mut().add_converter_for_id(
            id,
            VERSION,
            map_data_converter_func(|data, _from_version, _to_version| {
                update_animal(data);
            }),
        );
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
        types::entity_mut().add_converter_for_id(
            id,
            VERSION,
            map_data_converter_func(|data, _from_version, _to_version| {
                update_mob(data);
            }),
        );
    }

    types::entity_mut().add_converter_for_id(
        "minecraft:armor_stand",
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            update_living_entity(data);
        }),
    );

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
        types::entity_mut().add_converter_for_id(
            id,
            VERSION,
            map_data_converter_func(|data, _from_version, _to_version| {
                update_projectile(data);
            }),
        );
    }

    for id in ["minecraft:bee", "minecraft:zombified_piglin"] {
        types::entity_mut().add_converter_for_id(
            id,
            VERSION,
            map_data_converter_func(|data, _from_version, _to_version| {
                update_hurt_by(data);
            }),
        );
    }

    types::entity_mut().add_converter_for_id(
        "minecraft:fox",
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            update_fox(data);
        }),
    );
    types::entity_mut().add_converter_for_id(
        "minecraft:item",
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            update_item(data);
        }),
    );
    types::entity_mut().add_converter_for_id(
        "minecraft:shulker_bullet",
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            update_shulker_bullet(data);
        }),
    );
    types::entity_mut().add_converter_for_id(
        "minecraft:area_effect_cloud",
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            update_area_effect_cloud(data);
        }),
    );
    types::entity_mut().add_converter_for_id(
        "minecraft:zombie_villager",
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            update_zombie_villager(data);
        }),
    );
    types::entity_mut().add_converter_for_id(
        "minecraft:evoker_fangs",
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            update_evoker_fangs(data);
        }),
    );
    types::entity_mut().add_converter_for_id(
        "minecraft:piglin",
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            update_piglin(data);
        }),
    );

    // Update TE
    types::tile_entity_mut().add_converter_for_id(
        "minecraft:conduit",
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            replace_uuid_ml_tag(data, "target_uuid", "Target");
        }),
    );
    types::tile_entity_mut().add_converter_for_id(
        "minecraft:skull",
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            if let Some(JValue::Compound(mut owner)) = data.remove("Owner") {
                replace_uuid_string(&mut owner, "Id", "Id");
                data.insert("SkullOwner", owner);
            }
        }),
    );

    // Player UUID
    types::player_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            update_living_entity(data);
            replace_uuid_least_most!(data, "UUID", "UUID");

            if let Some(JValue::Compound(root_vehicle)) = data.get_mut("RootVehicle") {
                replace_uuid_least_most!(root_vehicle, "Attach", "Attach");
            }
        }),
    );

    // Level.dat
    types::level_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            replace_uuid_string(data, "WanderingTraderId", "WanderingTraderId");

            if let Some(JValue::Compound(dimension_data)) = data.get_mut("DimensionData") {
                for dimension_data in dimension_data.values_mut() {
                    let JValue::Compound(dimension_data) = dimension_data else {
                        continue;
                    };
                    if let Some(JValue::Compound(dragon_fight)) =
                        dimension_data.get_mut("DragonFight")
                    {
                        replace_uuid_least_most!(dragon_fight, "DragonUUID", "Dragon");
                    }
                }
            }

            if let Some(JValue::Compound(custom_boss_events)) = data.get_mut("CustomBossEvents") {
                for custom_boss_event in custom_boss_events.values_mut() {
                    if let JValue::Compound(custom_boss_event) = custom_boss_event {
                        if let Some(JValue::List(players)) = custom_boss_event.get_mut("Players") {
                            let new_players: Vec<_> = match players {
                                JList::Compound(players) => players
                                    .iter()
                                    .filter_map(|player| {
                                        let least =
                                            player.get("L").and_then(|v| v.as_i64()).unwrap_or(0);
                                        let most =
                                            player.get("M").and_then(|v| v.as_i64()).unwrap_or(0);
                                        if least != 0 && most != 0 {
                                            Some(create_uuid_from_longs(least, most))
                                        } else {
                                            None
                                        }
                                    })
                                    .collect(),
                                _ => Vec::new(),
                            };
                            *players = if new_players.is_empty() {
                                JList::End
                            } else {
                                JList::IntArray(new_players)
                            };
                        }
                    }
                }
            }
        }),
    );

    types::saved_data_raids_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            let Some(JValue::Compound(data)) = data.get_mut("data") else {
                return;
            };
            let Some(JValue::List(JList::Compound(raids))) = data.get_mut("Raids") else {
                return;
            };
            for raid in raids {
                if let Some(JValue::List(heroes)) = raid.get_mut("HeroesOfTheVillage") {
                    let new_heroes: Vec<_> = match heroes {
                        JList::Compound(heroes) => heroes
                            .iter()
                            .filter_map(|hero| {
                                let least =
                                    hero.get("UUIDLeast").and_then(|v| v.as_i64()).unwrap_or(0);
                                let most =
                                    hero.get("UUIDMost").and_then(|v| v.as_i64()).unwrap_or(0);
                                if least != 0 || most != 0 {
                                    Some(create_uuid_from_longs(least, most))
                                } else {
                                    None
                                }
                            })
                            .collect(),
                        _ => Vec::new(),
                    };
                    *heroes = if new_heroes.is_empty() {
                        JList::End
                    } else {
                        JList::IntArray(new_heroes)
                    };
                }
            }
        }),
    );

    types::item_stack_mut().add_structure_converter(VERSION, map_data_converter_func(|data, _from_version, _to_version| {
        let is_player_head = matches!(data.get("id"), Some(JValue::String(str)) if str == "minecraft:player_head");

        if let Some(JValue::Compound(tag)) = data.get_mut("tag") {
            if let Some(JValue::List(JList::Compound(attributes))) = tag.get_mut("AttributeModifiers") {
                for attribute in attributes {
                    replace_uuid_least_most!(attribute, "UUID", "UUID");
                }
            }

            if is_player_head {
                if let Some(JValue::Compound(skull_owner)) = tag.get_mut("SkullOwner") {
                    replace_uuid_string(skull_owner, "Id", "Id");
                }
            }
        }
    }));
}

fn update_animal_owner(data: &mut JCompound) {
    update_animal(data);

    replace_uuid_string(data, "OwnerUUID", "Owner");
}

fn update_animal(data: &mut JCompound) {
    update_mob(data);

    replace_uuid_least_most!(data, "LoveCause", "LoveCause");
}

fn update_mob(data: &mut JCompound) {
    update_living_entity(data);

    if let Some(JValue::Compound(leash)) = data.get_mut("Leash") {
        replace_uuid_least_most!(leash, "UUID", "UUID");
    }
}

fn update_living_entity(data: &mut JCompound) {
    if let Some(JValue::List(JList::Compound(attributes))) = data.get_mut("Attributes") {
        for attribute in attributes {
            if let Some(JValue::List(JList::Compound(modifiers))) = attribute.get_mut("Modifiers") {
                for modifier in modifiers {
                    replace_uuid_least_most!(modifier, "UUID", "UUID");
                }
            }
        }
    }
}

fn update_projectile(data: &mut JCompound) {
    rename_key(data, "OwnerUUID", "Owner");
}

fn update_hurt_by(data: &mut JCompound) {
    replace_uuid_string(data, "HurtBy", "HurtBy");
}

fn update_fox(data: &mut JCompound) {
    if let Some(JValue::List(trusted_uuids)) = data.remove("TrustedUUIDs") {
        let trusted: Vec<_> = match trusted_uuids {
            JList::Compound(trusted_uuids) => trusted_uuids
                .iter()
                .filter_map(|uuid| {
                    let least = uuid.get("L").and_then(|v| v.as_i64()).unwrap_or(0);
                    let most = uuid.get("M").and_then(|v| v.as_i64()).unwrap_or(0);
                    if least != 0 || most != 0 {
                        Some(create_uuid_from_longs(least, most))
                    } else {
                        None
                    }
                })
                .collect(),
            _ => Vec::new(),
        };
        data.insert(
            "Trusted",
            if trusted.is_empty() {
                JList::End
            } else {
                JList::IntArray(trusted)
            },
        );
    }
}

fn update_item(data: &mut JCompound) {
    replace_uuid_ml_tag(data, "Owner", "Owner");
    replace_uuid_ml_tag(data, "Thrower", "Thrower");
}

fn update_shulker_bullet(data: &mut JCompound) {
    replace_uuid_ml_tag(data, "Owner", "Owner");
    replace_uuid_ml_tag(data, "Target", "Target");
}

fn update_area_effect_cloud(data: &mut JCompound) {
    replace_uuid_least_most!(data, "OwnerUUID", "Owner");
}

fn update_zombie_villager(data: &mut JCompound) {
    replace_uuid_least_most!(data, "ConversionPlayer", "ConversionPlayer");
}

fn update_evoker_fangs(data: &mut JCompound) {
    replace_uuid_least_most!(data, "OwnerUUID", "Owner");
}

fn update_piglin(data: &mut JCompound) {
    let Some(JValue::Compound(brain)) = data.get_mut("Brain") else {
        return;
    };
    let Some(JValue::Compound(memories)) = brain.get_mut("memories") else {
        return;
    };
    let Some(JValue::Compound(angry_at)) = memories.get_mut("minecraft:angry_at") else {
        return;
    };
    replace_uuid_string(angry_at, "value", "value");
}
