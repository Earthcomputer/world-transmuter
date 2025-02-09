use crate::helpers::rename::rename_attribute;
use crate::static_string_mc_set;
use java_string::{format_java, JavaStr};

const VERSION: u32 = 4055;

const PREFIXES_TO_REMOVE: [&JavaStr; 4] = [
    JavaStr::from_str("generic."),
    JavaStr::from_str("horse."),
    JavaStr::from_str("player."),
    JavaStr::from_str("zombie."),
];

pub(crate) fn register() {
    rename_attribute(VERSION, |mut attribute| {
        if attribute.starts_with("minecraft:") {
            attribute = &attribute[10..];
        }
        for prefix in PREFIXES_TO_REMOVE {
            if attribute.starts_with(prefix) {
                return Some(format_java!("minecraft:{}", &attribute[prefix.len()..]));
            }
        }
        None
    });
}
