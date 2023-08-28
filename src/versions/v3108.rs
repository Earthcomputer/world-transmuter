use crate::types::MinecraftTypesMut;
use rust_dataconverter_engine::map_data_converter_func;
use valence_nbt::Value;

const VERSION: u32 = 3108;

pub(crate) fn register(types: &MinecraftTypesMut) {
    types.chunk.borrow_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            let Some(Value::Compound(context)) = data.get("__context") else {
                return;
            };
            let Some(Value::String(dimension)) = context.get("dimension") else {
                return;
            };
            if dimension != "minecraft:overworld" {
                return;
            }
            data.remove("blending_data");
        }),
    );
}
