use crate::types::MinecraftTypesMut;
use rust_dataconverter_engine::DataWalkerMapListPaths;

const VERSION: u32 = 3326;

fn register_mob(types: MinecraftTypesMut, id: impl Into<String>) {
    types.entity().borrow_mut().add_walker_for_id(
        VERSION,
        id,
        DataWalkerMapListPaths::new_multi(
            types.item_stack(),
            vec!["ArmorItems".to_owned(), "HandItems".to_owned()],
        ),
    );
}

pub(crate) fn register(types: MinecraftTypesMut) {
    register_mob(types, "minecraft:sniffer");
}
