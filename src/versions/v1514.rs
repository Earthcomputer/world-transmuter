use crate::MinecraftTypesMut;
use valence_nbt::Value;
use world_transmuter_engine::map_data_converter_func;

const VERSION: u32 = 1514;

pub(crate) fn register(types: MinecraftTypesMut) {
    types.objective().borrow_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            if let Some(Value::String(display_name)) = data.get_mut("DisplayName") {
                let update = format!(
                    "{{\"text\":\"{}\"}}",
                    display_name.replace('\\', "\\\\").replace('"', "\\\"")
                );
                *display_name = update;
            }

            if !matches!(data.get("RenderType"), Some(Value::String(_))) {
                let criteria_name = match data.get("CriteriaName") {
                    Some(Value::String(str)) if str == "health" => "hearts",
                    _ => "integer",
                };
                data.insert("RenderType", criteria_name);
            }
        }),
    );

    types.team().borrow_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            if let Some(Value::String(display_name)) = data.get_mut("DisplayName") {
                let update = format!(
                    "{{\"text\":\"{}\"}}",
                    display_name.replace('\\', "\\\\").replace('"', "\\\"")
                );
                *display_name = update;
            }
        }),
    );
}
