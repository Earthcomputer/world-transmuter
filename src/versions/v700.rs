use crate::MinecraftTypes;
use rust_dataconverter_engine::{map_data_converter_func, DataWalkerMapListPaths};

const VERSION: u32 = 700;

pub(crate) fn register<'a>(types: &'a MinecraftTypes<'a>) {
    types.entity.borrow_mut().add_converter_for_id(
        "Guardian",
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            if data.remove("Elder").and_then(|o| o.as_bool()) == Some(true) {
                data.insert("id", "ElderGuardian");
            }
        }),
    );

    register_mob(types, "ElderGuardian");
}

fn register_mob<'a>(types: &'a MinecraftTypes<'a>, id: impl Into<String>) {
    types.entity.borrow_mut().add_walker_for_id(
        VERSION,
        id,
        DataWalkerMapListPaths::new_multi(
            &types.item_stack,
            vec!["ArmorItems".to_owned(), "HandItems".to_owned()],
        ),
    );
}
