use rust_dataconverter_engine::{convert_object_in_map, DataVersion, DataWalker, ObjectDataType};
use std::cell::RefCell;
use valence_nbt::{Compound, Value};

pub(crate) struct GameEventListenerWalker<'a> {
    game_event_type: &'a RefCell<ObjectDataType<'a>>,
}

impl<'a> GameEventListenerWalker<'a> {
    pub(crate) fn new(game_event_type: &'a RefCell<ObjectDataType<'a>>) -> Self {
        Self { game_event_type }
    }
}

impl<'a> DataWalker for GameEventListenerWalker<'a> {
    fn walk(&self, data: &mut Compound, from_version: DataVersion, to_version: DataVersion) {
        if let Some(Value::Compound(listener)) = data.get_mut("listener") {
            if let Some(Value::Compound(event)) = listener.get_mut("event") {
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
