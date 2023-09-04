use crate::types;
use crate::versions::v1451::ConverterFlattenSpawnEgg;

const VERSION: u32 = 3209;

pub(crate) fn register() {
    types::item_stack_mut().add_converter_for_id(
        "minecraft:spawn_egg",
        VERSION,
        ConverterFlattenSpawnEgg,
    );
}
