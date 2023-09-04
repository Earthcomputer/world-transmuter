use crate::helpers::gson_lenient_fix::{fix_gson_lenient, FixedGsonLenient, JsonType};
use crate::types;
use valence_nbt::{List, Value};
use world_transmuter_engine::map_data_converter_func;

const VERSION: u32 = 165;

pub(crate) fn register() {
    types::item_stack_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            if let Some(Value::Compound(tag)) = data.get_mut("tag") {
                if let Some(Value::List(List::String(pages))) = tag.get_mut("pages") {
                    for page in pages {
                        let new_page = if page == "null" || page.chars().all(|c| c.is_whitespace())
                        {
                            "{\"text\":\"\"}".to_owned()
                        } else if (page.starts_with('"') && page.ends_with('"'))
                            || (page.starts_with('{') && page.ends_with('}'))
                        {
                            match fix_gson_lenient(page) {
                                Ok(FixedGsonLenient {
                                    value_type,
                                    fixed_str,
                                }) => match value_type {
                                    JsonType::Object | JsonType::Array => fixed_str.into_owned(),
                                    JsonType::String | JsonType::Number => {
                                        format!("{{\"text\":{}}}", fixed_str)
                                    }
                                    JsonType::Keyword => {
                                        if fixed_str == "null" {
                                            "{\"text\":\"\"}".to_owned()
                                        } else {
                                            format!("{{\"text\":\"{}\"}}", fixed_str)
                                        }
                                    }
                                },
                                Err(_) => {
                                    format!(
                                        "{{\"text\":\"{}\"}}",
                                        page.replace('\\', "\\\\").replace('"', "\\\"")
                                    )
                                }
                            }
                        } else {
                            format!(
                                "{{\"text\":\"{}\"}}",
                                page.replace('\\', "\\\\").replace('"', "\\\"")
                            )
                        };

                        *page = new_page;
                    }
                }
            }
        }),
    );
}
