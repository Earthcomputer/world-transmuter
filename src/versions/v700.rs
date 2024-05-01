use crate::types;
use crate::versions::v100;
use java_string::JavaString;
use world_transmuter_engine::map_data_converter_func;

const VERSION: u32 = 700;

pub(crate) fn register() {
    types::entity_mut().add_converter_for_id(
        "Guardian",
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            if data.remove("Elder").and_then(|o| o.as_bool()) == Some(true) {
                data.insert("id", "ElderGuardian");
            }
        }),
    );

    register_mob("ElderGuardian");
}

fn register_mob(id: impl Into<JavaString>) {
    v100::register_equipment(VERSION, id);
}
