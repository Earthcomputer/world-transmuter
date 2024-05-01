use crate::versions::v100;

const VERSION: u32 = 2568;

pub(crate) fn register() {
    v100::register_equipment(VERSION, "minecraft:piglin_brute");
}
