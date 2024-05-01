use crate::types;
use crate::versions::v100;
use world_transmuter_engine::map_data_converter_func;

const VERSION: u32 = 2707;

pub(crate) fn register() {
    types::world_gen_settings_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            if !data.contains_key("has_increased_height_already") {
                data.insert("has_increased_height_already", true);
            }
        }),
    );

    v100::register_equipment(VERSION, "minecraft:marker");
}
