use crate::types;
use crate::versions::v100;
use java_string::JavaStr;
use world_transmuter_engine::{
    DataVersion, DataWalkerMapListPaths, DataWalkerMapTypePaths, JCompound, JValue,
    MapDataConverterFunc,
};

const VERSION: u32 = 3808;

pub(crate) fn register() {
    // Step 0
    types::entity_mut().add_converter_for_id(
        "minecraft:horse",
        VERSION,
        BodyArmorConverter::new("ArmorItem"),
    );
    types::entity_mut().add_walker_for_id(
        VERSION,
        "minecraft:horse",
        DataWalkerMapTypePaths::new(types::item_stack_ref(), "SaddleItem"),
    );
    v100::register_equipment(VERSION, "minecraft:horse");

    // Step 1
    types::entity_mut().add_converter_for_id(
        "minecraft:llama",
        DataVersion::new(VERSION, 1),
        BodyArmorConverter::new("DecorItem"),
    );
    types::entity_mut().add_walker_for_id(
        DataVersion::new(VERSION, 1),
        "minecraft:llama",
        DataWalkerMapListPaths::new(types::item_stack_ref(), "Items"),
    );
    types::entity_mut().add_walker_for_id(
        DataVersion::new(VERSION, 1),
        "minecraft:llama",
        DataWalkerMapTypePaths::new(types::item_stack_ref(), "SaddleItem"),
    );
    v100::register_equipment(DataVersion::new(VERSION, 1), "minecraft:llama");
}

struct BodyArmorConverter {
    path: &'static JavaStr,
}

impl BodyArmorConverter {
    fn new(path: &'static (impl AsRef<JavaStr> + ?Sized)) -> Self {
        Self {
            path: path.as_ref(),
        }
    }
}

impl MapDataConverterFunc for BodyArmorConverter {
    fn convert(&self, data: &mut JCompound, _from_version: DataVersion, _to_version: DataVersion) {
        let Some(JValue::Compound(prev)) = data.remove(self.path) else {
            return;
        };
        data.insert("body_armor_item", prev);
        data.insert("body_armor_drop_chance", 2f32);
    }
}
