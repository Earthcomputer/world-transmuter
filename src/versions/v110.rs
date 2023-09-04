use crate::MinecraftTypesMut;
use valence_nbt::{compound, Value};
use world_transmuter_engine::map_data_converter_func;

const VERSION: u32 = 110;

pub(crate) fn register(types: MinecraftTypesMut) {
    types.entity().borrow_mut().add_converter_for_id(
        "EntityHorse",
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            if data.get("Saddle").and_then(|v| v.as_bool()) != Some(true)
                || matches!(data.get("SaddleItem"), Some(Value::Compound(_)))
            {
                return;
            }

            let saddle_item = compound! {
                "id" => "minecraft:saddle",
                "Count" => 1i8,
                "Damage" => 0i16,
            };

            data.remove("Saddle");
            data.insert("SaddleItem", saddle_item);
        }),
    );
}
