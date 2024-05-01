use crate::versions::v100;

const VERSION: u32 = 2686;

pub(crate) fn register() {
    v100::register_equipment(VERSION, "minecraft:axolotl");
}
