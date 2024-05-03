use crate::helpers::gson_lenient_fix::{fix_gson_lenient, FixedGsonLenient, JsonType};
use java_string::{format_java, JavaStr, JavaString};
use std::borrow::{Borrow, Cow};

pub(crate) const EMPTY_COMPONENT: &JavaStr = JavaStr::from_str("{\"text\":\"\"}");

pub(crate) fn make_literal_component(value: &JavaStr) -> JavaString {
    format_java!(
        "{{\"text\":\"{}\"}}",
        value.replace('\\', "\\\\").replace('"', "\\\"")
    )
}

pub(crate) fn make_translatable_component(value: &JavaStr) -> JavaString {
    format_java!(
        "{{\"translate\":\"{}\"}}",
        value.replace('\\', "\\\\").replace('"', "\\\"")
    )
}

pub(crate) fn convert_component_from_lenient(lenient_component: &JavaStr) -> JavaString {
    if (lenient_component.starts_with('"') && lenient_component.ends_with('"'))
        || (lenient_component.starts_with('{') && lenient_component.ends_with('}'))
    {
        let FixedGsonLenient {
            value_type,
            fixed_str,
        } = fix_gson_lenient(lenient_component).unwrap_or(FixedGsonLenient {
            value_type: JsonType::String,
            fixed_str: Cow::Owned(format_java!(
                "\"{}\"",
                lenient_component.replace('\\', "\\\\").replace('"', "\\\"")
            )),
        });
        match value_type {
            JsonType::Object | JsonType::Array => fixed_str.into_owned(),
            JsonType::String | JsonType::Number => format_java!("{{\"text\":{}}}", fixed_str),
            JsonType::Keyword => {
                if Borrow::<JavaStr>::borrow(&fixed_str) == "null" {
                    EMPTY_COMPONENT.to_owned()
                } else {
                    make_literal_component(&fixed_str)
                }
            }
        }
    } else {
        make_literal_component(lenient_component)
    }
}
