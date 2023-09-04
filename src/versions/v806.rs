use crate::types;
use valence_nbt::{Compound, Value};
use world_transmuter_engine::{DataVersion, MapDataConverterFunc};

const VERSION: u32 = 806;

pub(crate) fn register() {
    struct PotionWaterUpdater;
    impl MapDataConverterFunc for PotionWaterUpdater {
        fn convert(
            &self,
            data: &mut Compound,
            _from_version: DataVersion,
            _to_version: DataVersion,
        ) {
            let Value::Compound(tag) = data.entry("tag").or_insert_with(Compound::new) else {
                return;
            };

            if !matches!(tag.get("Potion"), Some(Value::String(_))) {
                tag.insert("Potion", "minecraft:water");
            }
        }
    }

    types::item_stack_mut().add_converter_for_id("minecraft:potion", VERSION, PotionWaterUpdater);
    types::item_stack_mut().add_converter_for_id(
        "minecraft:splash_potion",
        VERSION,
        PotionWaterUpdater,
    );
    types::item_stack_mut().add_converter_for_id(
        "minecraft:lingering_potion",
        VERSION,
        PotionWaterUpdater,
    );
    types::item_stack_mut().add_converter_for_id(
        "minecraft:tipped_arrow",
        VERSION,
        PotionWaterUpdater,
    );
}
