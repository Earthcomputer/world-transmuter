use crate::types;
use crate::versions::v3086::ConverterEntityToVariant;
use java_string::JavaStr;

const VERSION: u32 = 3087;

const FROG_ID_CONVERSION: [&JavaStr; 3] = [
    JavaStr::from_str("minecraft:temperate"),
    JavaStr::from_str("minecraft:warm"),
    JavaStr::from_str("minecraft:cold"),
];

pub(crate) fn register() {
    types::entity_mut().add_converter_for_id(
        "minecraft:frog",
        VERSION,
        ConverterEntityToVariant::new("Version", |id: i32| {
            FROG_ID_CONVERSION
                .get(id as usize)
                .copied()
                .unwrap_or(JavaStr::from_str("minecraft:temperate"))
        }),
    );
}
