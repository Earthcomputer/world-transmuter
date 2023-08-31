use crate::helpers::walkers::GameEventListenerWalker;
use crate::MinecraftTypesMut;

const VERSION: u32 = 2684;

pub(crate) fn register(types: MinecraftTypesMut) {
    types.tile_entity().borrow_mut().add_walker_for_id(
        VERSION,
        "minecraft:sculk_sensor",
        GameEventListenerWalker::new(types.game_event_name()),
    );
}
