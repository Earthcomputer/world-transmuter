use rust_dataconverter_engine::{data_converter_func, ListType, MapType, ObjectType, Types};
use crate::helpers::gson_lenient_fix::{fix_gson_lenient, FixedGsonLenient, JsonType};
use crate::MinecraftTypesMut;

const VERSION: u32 = 165;

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    types.item_stack.borrow_mut().add_structure_converter(VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        if let Some(tag) = data.get_map_mut("tag") {
            if let Some(pages) = tag.get_list_mut("pages") {
                for i in 0..pages.size() {
                    if let Some(page) = pages.get(i).as_string() {
                        let new_page = if page == "null" || page.chars().all(|c| c.is_whitespace()) {
                            "{\"text\":\"\"}".to_owned()
                        } else if (page.starts_with('"') && page.ends_with('"')) || (page.starts_with('{') && page.ends_with('}')) {
                            match fix_gson_lenient(page) {
                                Ok(FixedGsonLenient { value_type, fixed_str }) => {
                                    match value_type {
                                        JsonType::Object | JsonType::Array => fixed_str.into_owned(),
                                        JsonType::String | JsonType::Number => format!("{{\"text\":{}}}", fixed_str),
                                        JsonType::Keyword => {
                                            if fixed_str == "null" {
                                                "{\"text\":\"\"}".to_owned()
                                            } else {
                                                format!("{{\"text\":\"{}\"}}", fixed_str)
                                            }
                                        }
                                    }
                                }
                                Err(_) => {
                                    format!("{{\"text\":\"{}\"}}", page.replace('\\', "\\\\").replace('"', "\\\""))
                                }
                            }
                        } else {
                            format!("{{\"text\":\"{}\"}}", page.replace('\\', "\\\\").replace('"', "\\\""))
                        };

                        pages.set(i, T::Object::create_string(new_page));
                    }
                }
            }
        }
    }));
}
