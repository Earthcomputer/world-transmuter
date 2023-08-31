use crate::helpers::rename::rename_criteria;
use crate::types::MinecraftTypesMut;
use rust_dataconverter_engine::{DataVersion, MapDataConverterFunc};
use std::collections::BTreeMap;
use std::sync::OnceLock;
use valence_nbt::Compound;

const VERSION: u32 = 3086;

const CAT_ID_CONVERSION: [&str; 11] = [
    "minecraft:tabby",
    "minecraft:black",
    "minecraft:red",
    "minecraft:siamese",
    "minecraft:british",
    "minecraft:calico",
    "minecraft:persian",
    "minecraft:ragdoll",
    "minecraft:white",
    "minecraft:jellie",
    "minecraft:all_black",
];

static CAT_ADVANCEMENTS_CONVERSION: OnceLock<BTreeMap<&str, &str>> = OnceLock::new();

fn cat_advancements_conversion() -> &'static BTreeMap<&'static str, &'static str> {
    CAT_ADVANCEMENTS_CONVERSION.get_or_init(|| {
        let mut map = BTreeMap::new();
        map.insert("textures/entity/cat/tabby.png", "minecraft:tabby");
        map.insert("textures/entity/cat/black.png", "minecraft:black");
        map.insert("textures/entity/cat/red.png", "minecraft:red");
        map.insert("textures/entity/cat/siamese.png", "minecraft:siamese");
        map.insert(
            "textures/entity/cat/british_shorthair.png",
            "minecraft:british",
        );
        map.insert("textures/entity/cat/calico.png", "minecraft:calico");
        map.insert("textures/entity/cat/persian.png", "minecraft:persian");
        map.insert("textures/entity/cat/ragdoll.png", "minecraft:ragdoll");
        map.insert("textures/entity/cat/white.png", "minecraft:white");
        map.insert("textures/entity/cat/jellie.png", "minecraft:jellie");
        map.insert("textures/entity/cat/all_black.png", "minecraft:all_black");
        map
    })
}

pub(crate) fn register(types: MinecraftTypesMut) {
    types.entity().borrow_mut().add_converter_for_id(
        "minecraft:cat",
        VERSION,
        ConverterEntityToVariant::new("CatType", |id: i32| {
            CAT_ID_CONVERSION
                .get(id as usize)
                .copied()
                .unwrap_or("minecraft:tabby")
        }),
    );
    rename_criteria(
        types,
        VERSION,
        "minecraft:husbandry/complete_catalogue",
        |name| {
            cat_advancements_conversion()
                .get(name)
                .copied()
                .map(|new_name| new_name.to_owned())
        },
    );
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
    F: Fn(i32) -> &'static str,
{
    fn convert(&self, data: &mut Compound, _from_version: DataVersion, _to_version: DataVersion) {
        let Some(value) = data.get(self.path).and_then(|v| v.as_i32()) else {
            // nothing to do, DFU does the same
            return;
        };

        let converted = (self.conversion)(value);

        // DFU doesn't appear to remove the old field, so why should we?

        data.insert("variant", converted);
    }
}
