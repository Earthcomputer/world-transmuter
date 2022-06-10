use rust_dataconverter_engine::Types;
use crate::helpers::walkers::GameEventListenerWalker;
use crate::MinecraftTypesMut;

const VERSION: u32 = 2684;

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    types.tile_entity.borrow_mut().add_walker_for_id(VERSION, "minecraft:sculk_sensor", GameEventListenerWalker::new(types.game_event_name));
}
