use crate::MinecraftTypesMut;
use rust_dataconverter_engine::map_data_converter_func;
use valence_nbt::Value;

const VERSION: u32 = 1936;

pub(crate) fn register(types: MinecraftTypesMut) {
    types.options().borrow_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            if let Some(Value::String(chat_opacity)) = data.get("chatOpacity") {
                // Vanilla uses createDouble here, but options is always string -> string. I presume they made
                // a mistake with this converter.
                let background = calculate_background(chat_opacity);
                data.insert("textBackgroundOpacity", background);
            }
        }),
    );
}

fn calculate_background(opacity: &str) -> String {
    opacity
        .parse::<f64>()
        .map(|d| (0.9 * d + 0.1) / 2.0)
        .unwrap_or(0.5)
        .to_string()
}
