use crate::types;
use java_string::format_java;
use world_transmuter_engine::{map_data_converter_func, JValue};

const VERSION: u32 = 1514;

pub(crate) fn register() {
    types::objective_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            if let Some(JValue::String(display_name)) = data.get_mut("DisplayName") {
                let update = format_java!(
                    "{{\"text\":\"{}\"}}",
                    display_name.replace('\\', "\\\\").replace('"', "\\\"")
                );
                *display_name = update;
            }

            if !matches!(data.get("RenderType"), Some(JValue::String(_))) {
                let criteria_name = match data.get("CriteriaName") {
                    Some(JValue::String(str)) if str == "health" => "hearts",
                    _ => "integer",
                };
                data.insert("RenderType", criteria_name);
            }
        }),
    );

    types::team_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            if let Some(JValue::String(display_name)) = data.get_mut("DisplayName") {
                let update = format_java!(
                    "{{\"text\":\"{}\"}}",
                    display_name.replace('\\', "\\\\").replace('"', "\\\"")
                );
                *display_name = update;
            }
        }),
    );
}
