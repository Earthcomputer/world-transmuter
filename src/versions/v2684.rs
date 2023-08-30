use crate::helpers::walkers::GameEventListenerWalker;
use crate::MinecraftTypes;

const VERSION: u32 = 2684;

pub(crate) fn register<'a>(types: &'a MinecraftTypes<'a>) {
    types.tile_entity.borrow_mut().add_walker_for_id(
        VERSION,
        "minecraft:sculk_sensor",
        GameEventListenerWalker::new(&types.game_event_name),
    );
}
