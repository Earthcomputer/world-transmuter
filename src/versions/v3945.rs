use crate::{static_string_map, types};
use ahash::AHashMap;
use indexmap::IndexMap;
use java_string::{format_java, JavaStr};
use std::borrow::Cow;
use std::sync::OnceLock;
use uuid::{uuid, Uuid};
use world_transmuter_engine::{
    map_data_converter_func, rename_key, DataVersion, JCompound, JList, JValue,
};

const VERSION: u32 = 3945;

fn uuid_to_id() -> &'static AHashMap<Uuid, &'static str> {
    static UUID_TO_ID: OnceLock<AHashMap<Uuid, &'static str>> = OnceLock::new();
    UUID_TO_ID.get_or_init(|| {
        let mut m = AHashMap::new();
        m.insert(
            uuid!("736565d2-e1a7-403d-a3f8-1aeb3e302542"),
            "minecraft:creative_mode_block_range",
        );
        m.insert(
            uuid!("98491ef6-97b1-4584-ae82-71a8cc85cf73"),
            "minecraft:creative_mode_entity_range",
        );
        m.insert(
            uuid!("91AEAA56-376B-4498-935B-2F7F68070635"),
            "minecraft:effect.speed",
        );
        m.insert(
            uuid!("7107DE5E-7CE8-4030-940E-514C1F160890"),
            "minecraft:effect.slowness",
        );
        m.insert(
            uuid!("AF8B6E3F-3328-4C0A-AA36-5BA2BB9DBEF3"),
            "minecraft:effect.haste",
        );
        m.insert(
            uuid!("55FCED67-E92A-486E-9800-B47F202C4386"),
            "minecraft:effect.mining_fatigue",
        );
        m.insert(
            uuid!("648D7064-6A60-4F59-8ABE-C2C23A6DD7A9"),
            "minecraft:effect.strength",
        );
        m.insert(
            uuid!("C0105BF3-AEF8-46B0-9EBC-92943757CCBE"),
            "minecraft:effect.jump_boost",
        );
        m.insert(
            uuid!("22653B89-116E-49DC-9B6B-9971489B5BE5"),
            "minecraft:effect.weakness",
        );
        m.insert(
            uuid!("5D6F0BA2-1186-46AC-B896-C61C5CEE99CC"),
            "minecraft:effect.health_boost",
        );
        m.insert(
            uuid!("EAE29CF0-701E-4ED6-883A-96F798F3DAB5"),
            "minecraft:effect.absorption",
        );
        m.insert(
            uuid!("03C3C89D-7037-4B42-869F-B146BCB64D2E"),
            "minecraft:effect.luck",
        );
        m.insert(
            uuid!("CC5AF142-2BD2-4215-B636-2605AED11727"),
            "minecraft:effect.unluck",
        );
        m.insert(
            uuid!("6555be74-63b3-41f1-a245-77833b3c2562"),
            "minecraft:evil",
        );
        m.insert(
            uuid!("1eaf83ff-7207-4596-b37a-d7a07b3ec4ce"),
            "minecraft:powder_snow",
        );
        m.insert(
            uuid!("662A6B8D-DA3E-4C1C-8813-96EA6097278D"),
            "minecraft:sprinting",
        );
        m.insert(
            uuid!("020E0DFB-87AE-4653-9556-831010E291A0"),
            "minecraft:attacking",
        );
        m.insert(
            uuid!("766bfa64-11f3-11ea-8d71-362b9e155667"),
            "minecraft:baby",
        );
        m.insert(
            uuid!("7E0292F2-9434-48D5-A29F-9583AF7DF27F"),
            "minecraft:covered",
        );
        m.insert(
            uuid!("9e362924-01de-4ddd-a2b2-d0f7a405a174"),
            "minecraft:suffocating",
        );
        m.insert(
            uuid!("5CD17E52-A79A-43D3-A529-90FDE04B181E"),
            "minecraft:drinking",
        );
        m.insert(
            uuid!("B9766B59-9566-4402-BC1F-2EE2A276D836"),
            "minecraft:baby",
        );
        m.insert(
            uuid!("49455A49-7EC5-45BA-B886-3B90B23A1718"),
            "minecraft:attacking",
        );
        m.insert(
            uuid!("845DB27C-C624-495F-8C9F-6020A9A58B6B"),
            "minecraft:armor.boots",
        );
        m.insert(
            uuid!("D8499B04-0E66-4726-AB29-64469D734E0D"),
            "minecraft:armor.leggings",
        );
        m.insert(
            uuid!("9F3D476D-C118-4544-8365-64846904B48E"),
            "minecraft:armor.chestplate",
        );
        m.insert(
            uuid!("2AD3F246-FEE1-4E67-B886-69FD380BB150"),
            "minecraft:armor.helmet",
        );
        m.insert(
            uuid!("C1C72771-8B8E-BA4A-ACE0-81A93C8928B2"),
            "minecraft:armor.body",
        );
        m.insert(
            uuid!("b572ecd2-ac0c-4071-abde-9594af072a37"),
            "minecraft:enchantment.fire_protection",
        );
        m.insert(
            uuid!("40a9968f-5c66-4e2f-b7f4-2ec2f4b3e450"),
            "minecraft:enchantment.blast_protection",
        );
        m.insert(
            uuid!("07a65791-f64d-4e79-86c7-f83932f007ec"),
            "minecraft:enchantment.respiration",
        );
        m.insert(
            uuid!("60b1b7db-fffd-4ad0-817c-d6c6a93d8a45"),
            "minecraft:enchantment.aqua_affinity",
        );
        m.insert(
            uuid!("11dc269a-4476-46c0-aff3-9e17d7eb6801"),
            "minecraft:enchantment.depth_strider",
        );
        m.insert(
            uuid!("87f46a96-686f-4796-b035-22e16ee9e038"),
            "minecraft:enchantment.soul_speed",
        );
        m.insert(
            uuid!("b9716dbd-50df-4080-850e-70347d24e687"),
            "minecraft:enchantment.soul_speed",
        );
        m.insert(
            uuid!("92437d00-c3a7-4f2e-8f6c-1f21585d5dd0"),
            "minecraft:enchantment.swift_sneak",
        );
        m.insert(
            uuid!("5d3d087b-debe-4037-b53e-d84f3ff51f17"),
            "minecraft:enchantment.sweeping_edge",
        );
        m.insert(
            uuid!("3ceb37c0-db62-46b5-bd02-785457b01d96"),
            "minecraft:enchantment.efficiency",
        );
        m.insert(
            uuid!("CB3F55D3-645C-4F38-A497-9C13A33DB5CF"),
            "minecraft:base_attack_damage",
        );
        m.insert(
            uuid!("FA233E1C-4180-4865-B01B-BCCE9785ACA3"),
            "minecraft:base_attack_speed",
        );
        m
    })
}

static_string_map! {
    name_to_id = {
        "Random spawn bonus" => "minecraft:random_spawn_bonus",
        "Random zombie-spawn bonus" => "minecraft:zombie_random_spawn_bonus",
        "Leader zombie bonus" => "minecraft:leader_zombie_bonus",
        "Zombie reinforcement callee charge" => "minecraft:reinforcement_callee_charge",
        "Zombie reinforcement caller charge" => "minecraft:reinforcement_caller_charge",
    }
}

fn make_uuid(arr: &[i32]) -> Option<Uuid> {
    if arr.len() != 4 {
        return None;
    }
    let most = ((arr[0] as u32 as u64) << 32) | (arr[1] as u32 as u64);
    let least = ((arr[2] as u32 as u64) << 32) | (arr[3] as u32 as u64);
    Some(Uuid::from_u64_pair(most, least))
}

fn remap_modifiers(modifiers: &mut Vec<JCompound>) {
    let mut ret = IndexMap::with_capacity_and_hasher(modifiers.len(), ahash::RandomState::new());

    for mut modifier in modifiers.drain(..) {
        let uuid = modifier.get("uuid").and_then(|o| {
            if let JValue::IntArray(arr) = o {
                make_uuid(arr)
            } else {
                None
            }
        });

        if let Some(remapped_uuid) = uuid.and_then(|uuid| uuid_to_id().get(&uuid)).copied() {
            modifier.remove("uuid");
            modifier.remove("name");
            modifier.insert("id", remapped_uuid);
            ret.insert(Cow::Borrowed(JavaStr::from_str(remapped_uuid)), modifier);
        } else {
            let name = modifier.get("name").and_then(|o| {
                if let JValue::String(s) = o {
                    Some(s)
                } else {
                    None
                }
            });
            if let Some(remapped_name) = name.and_then(|name| name_to_id().get(&name[..])).copied()
            {
                if let Some(existing) = ret.get_mut(remapped_name) {
                    let existing_amount = existing
                        .get("amount")
                        .and_then(|v| v.as_f64())
                        .unwrap_or_default();
                    let modifier_amount = modifier
                        .get("amount")
                        .and_then(|v| v.as_f64())
                        .unwrap_or_default();
                    existing.insert("amount", existing_amount + modifier_amount);
                } else {
                    modifier.remove("uuid");
                    modifier.remove("name");
                    modifier.insert("id", remapped_name);
                    ret.insert(Cow::Borrowed(remapped_name), modifier);
                }
            } else {
                let id = uuid.map_or_else(
                    || Cow::Borrowed(JavaStr::from_str("minecraft:unknown")),
                    |uuid| Cow::Owned(format_java!("minecraft:{}", uuid.as_hyphenated())),
                );
                modifier.insert("id", id.clone());
                ret.insert(id, modifier);
            }
        }
    }

    modifiers.extend(ret.into_values());
}

pub(crate) fn register() {
    types::item_stack_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            let Some(JValue::Compound(components)) = data.get_mut("components") else {
                return;
            };
            let Some(JValue::Compound(attribute_modifiers)) =
                components.get_mut("minecraft:attribute_modifiers")
            else {
                return;
            };
            let Some(JValue::List(JList::Compound(modifiers))) =
                attribute_modifiers.get_mut("modifiers")
            else {
                return;
            };
            remap_modifiers(modifiers);
        }),
    );

    let entity_converter =
        |data: &mut JCompound, _from_version: DataVersion, _to_version: DataVersion| {
            rename_key(data, "Attributes", "attributes");
            let Some(JValue::List(JList::Compound(attributes))) = data.get_mut("attributes") else {
                return;
            };
            for attribute in attributes {
                rename_key(attribute, "Name", "id");
                rename_key(attribute, "Base", "base");
                rename_key(attribute, "Modifiers", "modifiers");

                let Some(JValue::List(JList::Compound(modifiers))) = attribute.get_mut("modifiers")
                else {
                    continue;
                };
                for modifier in modifiers.iter_mut() {
                    rename_key(modifier, "UUID", "uuid");
                    rename_key(modifier, "Name", "name");
                    rename_key(modifier, "Amount", "amount");

                    let new_op = match modifier
                        .remove("Operation")
                        .and_then(|v| v.as_i32())
                        .unwrap_or_default()
                    {
                        0 => "add_value",
                        1 => "add_multiplied_base",
                        2 => "add_multiplied_total",
                        _ => "invalid",
                    };
                    modifier.insert("operation", new_op);
                }
                remap_modifiers(modifiers);
            }
        };

    types::player_mut().add_structure_converter(VERSION, map_data_converter_func(entity_converter));
    types::entity_mut().add_structure_converter(VERSION, map_data_converter_func(entity_converter));

    types::tile_entity_mut().add_converter_for_id(
        "minecraft:jukebox",
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            let playing_for = data
                .remove("TickCount")
                .and_then(|v| v.as_i64())
                .unwrap_or_default()
                - data
                    .remove("RecordStartTick")
                    .and_then(|v| v.as_i64())
                    .unwrap_or_default();
            data.remove("IsPlaying");

            if playing_for > 0 {
                data.insert("ticks_since_song_started", playing_for);
            }
        }),
    );
}
