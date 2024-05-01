use crate::versions::v100;

const VERSION: u32 = 1801;

pub(crate) fn register() {
    v100::register_equipment(VERSION, "minecraft:illager_beast");
}
