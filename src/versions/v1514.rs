use rust_dataconverter_engine::{data_converter_func, MapType, ObjectType, Types};
use crate::MinecraftTypesMut;

const VERSION: u32 = 1514;

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    types.objective.borrow_mut().add_structure_converter(VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        if let Some(display_name) = data.get_string("DisplayName") {
            let update = format!("{{\"text\":\"{}\"}}", display_name.replace('\\', "\\\\").replace('"', "\\\""));
            data.set("DisplayName", T::Object::create_string(update));
        }

        if data.get_string("RenderType").is_none() {
            let criteria_name = match data.get_string("CriteriaName") {
                Some("health") => "hearts",
                _ => "integer",
            };
            data.set("RenderType", T::Object::create_string(criteria_name.to_owned()));
        }
    }));

    types.team.borrow_mut().add_structure_converter(VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        if let Some(display_name) = data.get_string("DisplayName") {
            let update = format!("{{\"text\":\"{}\"}}", display_name.replace('\\', "\\\\").replace('"', "\\\""));
            data.set("DisplayName", T::Object::create_string(update));
        }
    }));
}
