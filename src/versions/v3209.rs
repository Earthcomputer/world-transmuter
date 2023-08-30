use crate::types::MinecraftTypes;
use crate::versions::v1451::ConverterFlattenSpawnEgg;

const VERSION: u32 = 3209;

pub(crate) fn register<'a>(types: &'a MinecraftTypes<'a>) {
    types.item_stack.borrow_mut().add_converter_for_id(
        "minecraft:spawn_egg",
        VERSION,
        ConverterFlattenSpawnEgg,
    );
}
