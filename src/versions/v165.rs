use crate::helpers::gson_lenient_fix::{fix_gson_lenient, FixedGsonLenient, JsonType};
use crate::types;
use java_string::{format_java, JavaStr, JavaString};
use std::borrow::Borrow;
use world_transmuter_engine::{map_data_converter_func, JList, JValue};

const VERSION: u32 = 165;

pub(crate) fn register() {
    types::item_stack_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            if let Some(JValue::Compound(tag)) = data.get_mut("tag") {
                if let Some(JValue::List(JList::String(pages))) = tag.get_mut("pages") {
                    for page in pages {
                        let new_page = if page == "null" || page.chars().all(|c| c.is_whitespace())
                        {
                            JavaString::from("{\"text\":\"\"}")
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
                                        format_java!("{{\"text\":{}}}", fixed_str)
                                    }
                                    JsonType::Keyword => {
                                        if Borrow::<JavaStr>::borrow(&fixed_str) == "null" {
                                            JavaString::from("{\"text\":\"\"}")
                                        } else {
                                            format_java!("{{\"text\":\"{}\"}}", fixed_str)
                                        }
                                    }
                                },
                                Err(_) => {
                                    format_java!(
                                        "{{\"text\":\"{}\"}}",
                                        page.replace('\\', "\\\\").replace('"', "\\\"")
                                    )
                                }
                            }
                        } else {
                            format_java!(
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
