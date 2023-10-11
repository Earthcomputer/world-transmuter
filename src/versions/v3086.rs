use crate::helpers::rename::rename_criteria;
use crate::{static_string_map, types};
use java_string::JavaStr;
use world_transmuter_engine::{DataVersion, JCompound, MapDataConverterFunc};

const VERSION: u32 = 3086;

const CAT_ID_CONVERSION: [&JavaStr; 11] = [
    JavaStr::from_str("minecraft:tabby"),
    JavaStr::from_str("minecraft:black"),
    JavaStr::from_str("minecraft:red"),
    JavaStr::from_str("minecraft:siamese"),
    JavaStr::from_str("minecraft:british"),
    JavaStr::from_str("minecraft:calico"),
    JavaStr::from_str("minecraft:persian"),
    JavaStr::from_str("minecraft:ragdoll"),
    JavaStr::from_str("minecraft:white"),
    JavaStr::from_str("minecraft:jellie"),
    JavaStr::from_str("minecraft:all_black"),
];

static_string_map! {
    CAT_ADVANCEMENTS_CONVERSION, cat_advancements_conversion, {
        "textures/entity/cat/tabby.png" => "minecraft:tabby",
        "textures/entity/cat/black.png" => "minecraft:black",
        "textures/entity/cat/red.png" => "minecraft:red",
        "textures/entity/cat/siamese.png" => "minecraft:siamese",
        "textures/entity/cat/british_shorthair.png" => "minecraft:british",
        "textures/entity/cat/calico.png" => "minecraft:calico",
        "textures/entity/cat/persian.png" => "minecraft:persian",
        "textures/entity/cat/ragdoll.png" => "minecraft:ragdoll",
        "textures/entity/cat/white.png" => "minecraft:white",
        "textures/entity/cat/jellie.png" => "minecraft:jellie",
        "textures/entity/cat/all_black.png" => "minecraft:all_black",
    }
}

pub(crate) fn register() {
    types::entity_mut().add_converter_for_id(
        "minecraft:cat",
        VERSION,
        ConverterEntityToVariant::new("CatType", |id: i32| {
            CAT_ID_CONVERSION
                .get(id as usize)
                .copied()
                .unwrap_or(JavaStr::from_str("minecraft:tabby"))
        }),
    );
    rename_criteria(VERSION, "minecraft:husbandry/complete_catalogue", |name| {
        cat_advancements_conversion()
            .get(name)
            .copied()
            .map(|new_name| new_name.to_owned())
    });
}

pub(crate) struct ConverterEntityToVariant<F> {
    path: &'static str,
    conversion: F,
}

impl<F> ConverterEntityToVariant<F> {
    pub(crate) fn new(path: &'static str, conversion: F) -> Self {
        Self { path, conversion }
    }
}

impl<F> MapDataConverterFunc for ConverterEntityToVariant<F>
where
    F: Fn(i32) -> &'static JavaStr,
{
    fn convert(&self, data: &mut JCompound, _from_version: DataVersion, _to_version: DataVersion) {
        let Some(value) = data.get(self.path).and_then(|v| v.as_i32()) else {
            // nothing to do, DFU does the same
            return;
        };

        let converted = (self.conversion)(value);

        // DFU doesn't appear to remove the old field, so why should we?

        data.insert("variant", converted);
    }
}
