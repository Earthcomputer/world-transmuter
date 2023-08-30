use crate::helpers::walkers::GameEventListenerWalker;
use crate::types::MinecraftTypes;
use rust_dataconverter_engine::DataWalkerMapListPaths;

const VERSION: u32 = 3083;

fn register_mob<'a>(types: &'a MinecraftTypes<'a>, id: &str) {
    types.entity.borrow_mut().add_walker_for_id(
        VERSION,
        id,
        DataWalkerMapListPaths::new_multi(
            &types.item_stack,
            vec!["ArmorItems".to_owned(), "HandItems".to_owned()],
        ),
    );
}

pub(crate) fn register<'a>(types: &'a MinecraftTypes<'a>) {
    register_mob(types, "minecraft:allay");
    types.entity.borrow_mut().add_walker_for_id(
        VERSION,
        "minecraft:allay",
        GameEventListenerWalker::new(&types.game_event_name),
    );
}
