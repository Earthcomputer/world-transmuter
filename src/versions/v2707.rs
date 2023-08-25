use crate::MinecraftTypesMut;
use rust_dataconverter_engine::{map_data_converter_func, DataWalkerMapListPaths};

const VERSION: u32 = 2707;

pub(crate) fn register(types: &MinecraftTypesMut) {
    types
        .world_gen_settings
        .borrow_mut()
        .add_structure_converter(
            VERSION,
            map_data_converter_func(|data, _from_version, _to_version| {
                if !data.contains_key("has_increased_height_already") {
                    data.insert("has_increased_height_already", true);
                }
            }),
        );

    types.entity.borrow_mut().add_walker_for_id(
        VERSION,
        "minecraft:marker",
        DataWalkerMapListPaths::new_multi(
            types.item_stack,
            vec!["ArmorItems".to_owned(), "HandItems".to_owned()],
        ),
    ); // ?????????????
}
