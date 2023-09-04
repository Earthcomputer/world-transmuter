use crate::types;
use world_transmuter_engine::map_data_converter_func;

const VERSION: u32 = 3319;

pub(crate) fn register() {
    types::options_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            data.insert("onboardAccessibility", true);
        }),
    );
}
