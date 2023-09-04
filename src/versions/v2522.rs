use crate::MinecraftTypesMut;
use world_transmuter_engine::DataWalkerMapListPaths;

const VERSION: u32 = 2522;

pub(crate) fn register(types: MinecraftTypesMut) {
    types.entity().borrow_mut().add_walker_for_id(
        VERSION,
        "minecraft:zoglin",
        DataWalkerMapListPaths::new_multi(
            types.item_stack(),
            vec!["ArmorItems".to_owned(), "HandItems".to_owned()],
        ),
    );
}
