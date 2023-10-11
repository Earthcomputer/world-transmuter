use crate::types;
use java_string::JavaString;
use world_transmuter_engine::{map_data_converter_func, DataWalkerMapListPaths};

const VERSION: u32 = 701;

pub(crate) fn register() {
    types::entity_mut().add_converter_for_id(
        "Skeleton",
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            match data.remove("SkeletonType").and_then(|o| o.as_i64()) {
                Some(1) => {
                    data.insert("id", "WitherSkeleton");
                }
                Some(2) => {
                    data.insert("id", "Stray");
                }
                _ => {}
            }
        }),
    );

    register_mob("WitherSkeleton");
    register_mob("Stray");
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
