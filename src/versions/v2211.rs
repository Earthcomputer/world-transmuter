use crate::MinecraftTypesMut;
use rust_dataconverter_engine::map_data_converter_func;

const VERSION: u32 = 2211;

pub(crate) fn register(types: MinecraftTypesMut) {
    types
        .structure_feature()
        .borrow_mut()
        .add_structure_converter(
            VERSION,
            map_data_converter_func(|data, _from_version, _to_version| {
                if let Some(references) = data.get("references").and_then(|v| v.as_i32()) {
                    if references <= 0 {
                        data.insert("references", 1);
                    }
                }
            }),
        );
}
