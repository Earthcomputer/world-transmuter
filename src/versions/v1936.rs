use crate::types;
use java_string::{JavaStr, JavaString};
use world_transmuter_engine::{map_data_converter_func, JValue};

const VERSION: u32 = 1936;

pub(crate) fn register() {
    types::options_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            if let Some(JValue::String(chat_opacity)) = data.get("chatOpacity") {
                // Vanilla uses createDouble here, but options is always string -> string. I presume they made
                // a mistake with this converter.
                let background = calculate_background(chat_opacity);
                data.insert("textBackgroundOpacity", background);
            }
        }),
    );
}

fn calculate_background(opacity: &JavaStr) -> JavaString {
    let result = opacity
        .parse::<f64>()
        .map(|d| (0.9 * d + 0.1) / 2.0)
        .unwrap_or(0.5)
        .to_string();
    JavaString::from(result)
}
