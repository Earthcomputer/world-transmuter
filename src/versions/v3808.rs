use crate::types;
use crate::versions::v100;
use java_string::JavaStr;
use world_transmuter_engine::{
    DataVersion, DataWalkerMapListPaths, DataWalkerMapTypePaths, JCompound, JList, JValue,
    MapDataConverterFunc,
};

const VERSION: u32 = 3808;

pub(crate) fn register() {
    // Step 0
    types::entity_mut().add_converter_for_id(
        "minecraft:horse",
        VERSION,
        BodyArmorConverter::new("ArmorItem", true),
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
        BodyArmorConverter::new("DecorItem", false),
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

    // Step 2
    types::entity_mut().add_converter_for_id(
        "minecraft:trader_llama",
        DataVersion::new(VERSION, 2),
        BodyArmorConverter::new("DecorItem", false),
    );
    types::entity_mut().add_walker_for_id(
        DataVersion::new(VERSION, 2),
        "minecraft:trader_llama",
        DataWalkerMapListPaths::new(types::item_stack_ref(), "Items"),
    );
    types::entity_mut().add_walker_for_id(
        DataVersion::new(VERSION, 2),
        "minecraft:trader_llama",
        DataWalkerMapTypePaths::new(types::item_stack_ref(), "SaddleItem"),
    );
    v100::register_equipment(DataVersion::new(VERSION, 200), "minecraft:trader_llama");
}

struct BodyArmorConverter {
    path: &'static JavaStr,
    clear_armor: bool,
}

impl BodyArmorConverter {
    fn new(path: &'static (impl AsRef<JavaStr> + ?Sized), clear_armor: bool) -> Self {
        Self {
            path: path.as_ref(),
            clear_armor,
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

        if self.clear_armor {
            if let Some(JValue::List(JList::Compound(armor))) = data.get_mut("ArmorItems") {
                if armor.len() > 2 {
                    armor[2].clear();
                }
            }
            if let Some(JValue::List(JList::Float(chances))) = data.get_mut("ArmorDropChances") {
                if chances.len() > 2 {
                    chances[2] = 0.085;
                }
            }
        }
    }
}
