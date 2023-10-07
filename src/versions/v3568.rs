use crate::helpers::mc_namespace_map::McNamespaceSet;
use crate::types;
use std::sync::OnceLock;
use valence_nbt::{Compound, List, Value};
use world_transmuter_engine::{
    get_mut_multi, map_data_converter_func, DataVersion, MapDataConverterFunc,
};

const VERSION: u32 = 3568;

const EFFECT_ID_MAP: [&str; 33] = [
    "minecraft:speed",
    "minecraft:slowness",
    "minecraft:haste",
    "minecraft:mining_fatigue",
    "minecraft:strength",
    "minecraft:instant_health",
    "minecraft:instant_damage",
    "minecraft:jump_boost",
    "minecraft:nausea",
    "minecraft:regeneration",
    "minecraft:resistance",
    "minecraft:fire_resistance",
    "minecraft:water_breathing",
    "minecraft:invisibility",
    "minecraft:blindness",
    "minecraft:night_vision",
    "minecraft:hunger",
    "minecraft:weakness",
    "minecraft:poison",
    "minecraft:wither",
    "minecraft:health_boost",
    "minecraft:absorption",
    "minecraft:saturation",
    "minecraft:glowing",
    "minecraft:levitation",
    "minecraft:luck",
    "minecraft:unluck",
    "minecraft:slow_falling",
    "minecraft:conduit_power",
    "minecraft:dolphins_grace",
    "minecraft:bad_omen",
    "minecraft:hero_of_the_village",
    "minecraft:darkness",
];

fn get_effect_id(id: i32) -> Option<&'static str> {
    EFFECT_ID_MAP.get(id.wrapping_sub(1) as usize).copied()
}

static EFFECT_ITEMS: OnceLock<McNamespaceSet> = OnceLock::new();

fn effect_items() -> &'static McNamespaceSet<'static> {
    EFFECT_ITEMS.get_or_init(|| {
        let mut set = McNamespaceSet::new();
        set.insert_mc("potion");
        set.insert_mc("splash_potion");
        set.insert_mc("lingering_potion");
        set.insert_mc("tipped_arrow");
        set
    })
}

fn convert_legacy_effect(data: &mut Compound, legacy_path: &str, new_path: &str) {
    let Some(id) = data.remove(legacy_path).and_then(|id| id.as_i32()) else {
        return;
    };

    let Some(new_id) = get_effect_id(id) else {
        return;
    };

    data.insert(new_path, new_id);
}

static MOB_EFFECT_RENAMES: [(&str, &str); 7] = [
    ("Ambient", "ambient"),
    ("Amplifier", "amplifier"),
    ("Duration", "duration"),
    ("ShowParticles", "show_particles"),
    ("ShowIcon", "show_icon"),
    ("FactorCalculationData", "factor_calculation_data"),
    ("HiddenEffect", "hidden_effect"),
];

fn convert_mob_effect(mob_effect: &mut Compound) {
    convert_legacy_effect(mob_effect, "Id", "id");

    for (old_key, new_key) in MOB_EFFECT_RENAMES {
        if let Some(value) = mob_effect.remove(old_key) {
            mob_effect.insert(new_key, value);
        }
    }

    if let Some(Value::Compound(hidden_effect)) = mob_effect.get_mut("hidden_effect") {
        convert_mob_effect(hidden_effect);
    }
}

fn convert_mob_effect_list(data: &mut Compound, old_path: &str, new_path: &str) {
    let Some(Value::List(List::Compound(mut effects))) = data.remove(old_path) else {
        return;
    };

    for effect in &mut effects {
        convert_mob_effect(effect);
    }

    data.insert(new_path, List::Compound(effects));
}

fn update_suspicious_stew(from: &mut Compound) -> Compound {
    let mut into = Compound::new();
    if let Some(effect) = from
        .remove("EffectId")
        .and_then(|v| v.as_i32())
        .and_then(get_effect_id)
    {
        into.insert("id", effect);
    }
    if let Some(duration) = from.remove("EffectDuration") {
        into.insert("duration", duration);
    }
    into
}

pub(crate) fn register() {
    types::tile_entity_mut().add_converter_for_id(
        "minecraft:beacon",
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            convert_legacy_effect(data, "Primary", "primary_effect");
            convert_legacy_effect(data, "Secondary", "secondary_effect");
        }),
    );

    types::entity_mut().add_converter_for_id(
        "minecraft:mooshroom",
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            let new_effect = update_suspicious_stew(data);

            if !new_effect.is_empty() {
                data.insert("stew_effects", List::Compound(vec![new_effect]));
            }
        }),
    );
    types::entity_mut().add_converter_for_id(
        "minecraft:arrow",
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            convert_mob_effect_list(data, "CustomPotionEffects", "custom_potion_effects");
        }),
    );
    types::entity_mut().add_converter_for_id(
        "minecraft:area_effect_cloud",
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            convert_mob_effect_list(data, "Effects", "effects");
        }),
    );
    types::entity_mut().add_structure_converter(VERSION, LivingEntityConverter);

    types::player_mut().add_structure_converter(VERSION, LivingEntityConverter);

    types::item_stack_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            let [Some(Value::String(id)), Some(Value::Compound(tag))] =
                get_mut_multi(data, ["id", "tag"])
            else {
                return;
            };

            if id == "minecraft:suspicious_stew" {
                if let Some(effects) = tag.remove("Effects") {
                    if let Value::List(List::Compound(effects)) = effects {
                        tag.insert(
                            "effects",
                            List::Compound(
                                effects
                                    .into_iter()
                                    .map(|mut effect| update_suspicious_stew(&mut effect))
                                    .collect(),
                            ),
                        );
                    } else {
                        tag.insert("effects", effects);
                    }
                }
            }

            if effect_items().contains(id) {
                convert_mob_effect_list(tag, "CustomPotionEffects", "custom_potion_effects");
            }
        }),
    );
}

struct LivingEntityConverter;

impl MapDataConverterFunc for LivingEntityConverter {
    fn convert(&self, data: &mut Compound, _from_version: DataVersion, _to_version: DataVersion) {
        convert_mob_effect_list(data, "ActiveEffects", "active_effects");
    }
}
