use crate::{helpers, types};
use world_transmuter_engine::{map_data_converter_func, DataVersion, JCompound, JValue};

const VERSION: u32 = 4054;

pub(crate) fn register() {
    types::tile_entity_mut().add_converter_for_id(
        "minecraft:banner",
        VERSION,
        map_data_converter_func(convert_components),
    );
    types::item_stack_mut().add_converter_for_id(
        "minecraft:white_banner",
        VERSION,
        map_data_converter_func(convert_components),
    );
}

fn convert_components(data: &mut JCompound, _from_version: DataVersion, _to_version: DataVersion) {
    let Some(JValue::Compound(components)) = data.get_mut("components") else {
        return;
    };

    let Some(JValue::String(item_name)) = components.get_mut("minecraft:item_name") else {
        return;
    };
    if helpers::components::retrieve_translation_string(item_name)
        .is_none_or(|translation_key| translation_key != "block.minecraft.ominous_banner")
    {
        return;
    }
    *item_name = helpers::components::make_translatable_component("block.minecraft.ominous_banner");
    components.insert("minecraft:rarity", "uncommon");
}
