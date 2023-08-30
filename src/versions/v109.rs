use crate::MinecraftTypes;
use rust_dataconverter_engine::map_data_converter_func;

const VERSION: u32 = 109;

pub(crate) fn register<'a>(types: &'a MinecraftTypes<'a>) {
    types.entity.borrow_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            if let Some(health) = data
                .remove("HealF")
                .and_then(|o| o.as_f32())
                .or_else(|| data.get("Health").and_then(|o| o.as_f32()))
            {
                data.insert("Health", health);
            }
        }),
    );
}
