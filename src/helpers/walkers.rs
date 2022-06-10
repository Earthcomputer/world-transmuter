use std::cell::RefCell;
use rust_dataconverter_engine::{convert_object_in_map, DataVersion, DataWalker, MapType, ObjectDataType, Types};

pub(crate) struct GameEventListenerWalker<'a, T: Types + ?Sized> {
    game_event_type: &'a RefCell<ObjectDataType<'a, T>>,
}

impl<'a, T: Types + ?Sized> GameEventListenerWalker<'a, T> {
    pub(crate) fn new(game_event_type: &'a RefCell<ObjectDataType<'a, T>>) -> Self {
        Self { game_event_type }
    }
}

impl<'a, T: Types + ?Sized> DataWalker<T> for GameEventListenerWalker<'a, T> {
    fn walk(&self, data: &mut T::Map, from_version: DataVersion, to_version: DataVersion) {
        if let Some(listener) = data.get_map_mut("listener") {
            if let Some(event) = listener.get_map_mut("event") {
                convert_object_in_map::<_, T>(self.game_event_type, event, "game_event", from_version, to_version);
            }
        }
    }
}
