use crate::helpers::rename::rename_block;
use java_string::JavaString;

const VERSION: u32 = 1475;

pub(crate) fn register() {
    rename_block(VERSION, |old_name| match old_name.as_bytes() {
        b"minecraft:flowing_water" => Some(JavaString::from("minecraft:water")),
        b"minecraft:flowing_lava" => Some(JavaString::from("minecraft:lava")),
        _ => None,
    });
}
