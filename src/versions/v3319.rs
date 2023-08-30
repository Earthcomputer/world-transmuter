use crate::types::MinecraftTypes;
use rust_dataconverter_engine::map_data_converter_func;

const VERSION: u32 = 3319;

pub(crate) fn register<'a>(types: &'a MinecraftTypes<'a>) {
    types.options.borrow_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            data.insert("onboardAccessibility", true);
        }),
    );
}
