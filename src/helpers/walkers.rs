use std::sync::RwLock;
use world_transmuter_engine::{
    convert_object_in_map, DataVersion, JCompound, JValue, MapDataWalker, ObjectDataType,
};

pub(crate) struct GameEventListenerWalker<'a> {
    game_event_type: &'a RwLock<ObjectDataType<'a>>,
}

impl<'a> GameEventListenerWalker<'a> {
    pub(crate) fn new(game_event_type: &'a RwLock<ObjectDataType<'a>>) -> Self {
        Self { game_event_type }
    }
}

impl<'a> MapDataWalker for GameEventListenerWalker<'a> {
    fn walk(&self, data: &mut JCompound, from_version: DataVersion, to_version: DataVersion) {
        if let Some(JValue::Compound(listener)) = data.get_mut("listener") {
            if let Some(JValue::Compound(event)) = listener.get_mut("event") {
                convert_object_in_map(
                    self.game_event_type,
                    event,
                    "game_event",
                    from_version,
                    to_version,
                );
            }
        }
    }
}
