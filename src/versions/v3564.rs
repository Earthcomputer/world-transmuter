use crate::types;
use std::mem;
use valence_nbt::{Compound, List, Value};
use world_transmuter_engine::{get_mut_multi, DataVersion, MapDataConverterFunc};

const VERSION: u32 = 3564;

const LEGACY_FIELDS: [&str; 10] = [
    "Text1",
    "Text2",
    "Text3",
    "Text4",
    "FilteredText1",
    "FilteredText2",
    "FilteredText3",
    "FilteredText4",
    "Color",
    "GlowingText",
];

const EMPTY: &str = "{\"text\":\"\"}";

pub(crate) fn register() {
    types::tile_entity_mut().add_converter_for_id("minecraft:sign", VERSION, SignConverter);
    types::tile_entity_mut().add_converter_for_id("minecraft:hanging_sign", VERSION, SignConverter);
}

struct SignConverter;

impl MapDataConverterFunc for SignConverter {
    fn convert(&self, data: &mut Compound, _from_version: DataVersion, _to_version: DataVersion) {
        update_text(data.get_mut("front_text"));
        update_text(data.get_mut("back_text"));

        for to_remove in LEGACY_FIELDS {
            data.remove(to_remove);
        }
    }
}

fn update_text(text: Option<&mut Value>) {
    let Some(Value::Compound(text)) = text else {
        return;
    };

    if text
        .remove("_filtered_correct")
        .and_then(|v| v.as_bool())
        .unwrap_or(false)
    {
        return;
    }

    let [Some(Value::List(List::String(new_filtered_messages))), messages] =
        get_mut_multi(text, ["filtered_messages", "messages"])
    else {
        return;
    };

    let messages = match messages {
        Some(Value::List(List::String(messages))) => Some(&*messages),
        _ => None,
    };

    let filtered_messages = mem::take(new_filtered_messages);
    let mut new_filtered_is_empty = true;

    for (i, filtered) in filtered_messages.into_iter().enumerate() {
        let message = messages
            .and_then(|messages| messages.get(i))
            .map(|str| &str[..])
            .unwrap_or(EMPTY);

        if filtered == EMPTY {
            new_filtered_messages.push(message.to_owned());
            new_filtered_is_empty = new_filtered_is_empty && message == EMPTY;
        } else {
            new_filtered_messages.push(filtered);
            new_filtered_is_empty = false;
        }
    }

    if new_filtered_is_empty {
        text.remove("filtered_messages");
    }
}
