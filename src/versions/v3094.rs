use crate::types;
use world_transmuter_engine::{map_data_converter_func, JValue};

const VERSION: u32 = 3094;

const SOUND_VARIANT_TO_INSTRUMENT: [&str; 8] = [
    "minecraft:ponder_goat_horn",
    "minecraft:sing_goat_horn",
    "minecraft:seek_goat_horn",
    "minecraft:feel_goat_horn",
    "minecraft:admire_goat_horn",
    "minecraft:call_goat_horn",
    "minecraft:yearn_goat_horn",
    "minecraft:dream_goat_horn",
];

pub(crate) fn register() {
    types::item_stack_mut().add_converter_for_id(
        "minecraft:goat_horn",
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            let Some(JValue::Compound(tag)) = data.get_mut("tag") else {
                return;
            };
            let sound_variant = tag
                .remove("SoundVariant")
                .and_then(|v| v.as_i32())
                .unwrap_or(0);
            tag.insert(
                "instrument",
                SOUND_VARIANT_TO_INSTRUMENT
                    .get(sound_variant as usize)
                    .copied()
                    .unwrap_or("minecraft:ponder_goat_horn"),
            );
        }),
    );
}
