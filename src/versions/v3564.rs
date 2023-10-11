use crate::types;
use java_string::JavaStr;
use std::mem;
use world_transmuter_engine::{
    get_mut_multi, DataVersion, JCompound, JList, JValue, MapDataConverterFunc,
};

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

const EMPTY: &JavaStr = JavaStr::from_str("{\"text\":\"\"}");

pub(crate) fn register() {
    types::tile_entity_mut().add_converter_for_id("minecraft:sign", VERSION, SignConverter);
    types::tile_entity_mut().add_converter_for_id("minecraft:hanging_sign", VERSION, SignConverter);
}

struct SignConverter;

impl MapDataConverterFunc for SignConverter {
    fn convert(&self, data: &mut JCompound, _from_version: DataVersion, _to_version: DataVersion) {
        update_text(data.get_mut("front_text"));
        update_text(data.get_mut("back_text"));

        for to_remove in LEGACY_FIELDS {
            data.remove(to_remove);
        }
    }
}

fn update_text(text: Option<&mut JValue>) {
    let Some(JValue::Compound(text)) = text else {
        return;
    };

    if text
        .remove("_filtered_correct")
        .and_then(|v| v.as_bool())
        .unwrap_or(false)
    {
        return;
    }

    let [Some(JValue::List(JList::String(new_filtered_messages))), messages] =
        get_mut_multi(text, ["filtered_messages", "messages"])
    else {
        return;
    };

    if new_filtered_messages.is_empty() {
        return;
    }

    let messages = match messages {
        Some(JValue::List(JList::String(messages))) => Some(&*messages),
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
