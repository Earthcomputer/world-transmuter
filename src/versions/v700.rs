use crate::types;
use java_string::JavaString;
use world_transmuter_engine::{map_data_converter_func, DataWalkerMapListPaths};

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
    types::entity_mut().add_walker_for_id(
        VERSION,
        id,
        DataWalkerMapListPaths::new_multi(
            types::item_stack_ref(),
            vec!["ArmorItems".to_owned(), "HandItems".to_owned()],
        ),
    );
}
