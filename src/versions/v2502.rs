use crate::versions::v100;

const VERSION: u32 = 2502;

pub(crate) fn register() {
    v100::register_equipment(VERSION, "minecraft:hoglin");
}
