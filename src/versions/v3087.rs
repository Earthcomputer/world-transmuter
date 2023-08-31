use crate::types::MinecraftTypesMut;
use crate::versions::v3086::ConverterEntityToVariant;

const VERSION: u32 = 3087;

const FROG_ID_CONVERSION: [&str; 3] = ["minecraft:temperate", "minecraft:warm", "minecraft:cold"];

pub(crate) fn register(types: MinecraftTypesMut) {
    types.entity().borrow_mut().add_converter_for_id(
        "minecraft:frog",
        VERSION,
        ConverterEntityToVariant::new("Version", |id: i32| {
            FROG_ID_CONVERSION
                .get(id as usize)
                .copied()
                .unwrap_or("minecraft:temperate")
        }),
    );
}
