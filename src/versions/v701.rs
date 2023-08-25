use crate::MinecraftTypesMut;
use rust_dataconverter_engine::{map_data_converter_func, DataWalkerMapListPaths};

const VERSION: u32 = 701;

pub(crate) fn register(types: &MinecraftTypesMut) {
    types.entity.borrow_mut().add_converter_for_id(
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

    register_mob(types, "WitherSkeleton");
    register_mob(types, "Stray");
}

fn register_mob(types: &MinecraftTypesMut, id: impl Into<String>) {
    types.entity.borrow_mut().add_walker_for_id(
        VERSION,
        id,
        DataWalkerMapListPaths::new_multi(
            types.item_stack,
            vec!["ArmorItems".to_owned(), "HandItems".to_owned()],
        ),
    );
}
