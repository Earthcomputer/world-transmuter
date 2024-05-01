use crate::types;
use java_string::JavaStr;
use world_transmuter_engine::{
    convert_map_in_map, convert_map_list_in_map, map_data_converter_func, map_data_walker,
    DataVersion, JCompound, JList, JValue,
};

const VERSION: u32 = 3807;

pub(crate) fn register() {
    // Step 0
    types::tile_entity_mut().add_walker_for_id(
        VERSION,
        "minecraft:vault",
        map_data_walker(|data, from_version, to_version| {
            if let Some(JValue::Compound(config)) = data.get_mut("config") {
                convert_map_in_map(
                    types::item_stack_ref(),
                    config,
                    "key_item",
                    from_version,
                    to_version,
                );
            }
            if let Some(JValue::Compound(server_data)) = data.get_mut("server_data") {
                convert_map_list_in_map(
                    types::item_stack_ref(),
                    server_data,
                    "items_to_eject",
                    from_version,
                    to_version,
                );
            }
            if let Some(JValue::Compound(shared_data)) = data.get_mut("shared_data") {
                convert_map_in_map(
                    types::item_stack_ref(),
                    shared_data,
                    "display_item",
                    from_version,
                    to_version,
                );
            }
        }),
    );

    // Step 1
    types::saved_data_map_data_mut().add_structure_converter(
        DataVersion::new(VERSION, 1),
        map_data_converter_func(|data, _from_version, _to_version| {
            let Some(JValue::Compound(data)) = data.get_mut("data") else {
                return;
            };
            let Some(JValue::List(JList::Compound(banners))) = data.get_mut("banners") else {
                return;
            };
            for banner in banners {
                flatten_block_pos(banner, "Pos");
            }
        }),
    );
}

pub(crate) fn flatten_block_pos(data: &mut JCompound, path: &(impl AsRef<JavaStr> + ?Sized)) {
    let Some(pos) = data.get_mut(path.as_ref()) else {
        return;
    };

    let JValue::Compound(pos_compound) = pos else {
        return;
    };

    let Some(x) = pos_compound.get("X").and_then(|o| o.as_i32()) else {
        return;
    };
    let Some(y) = pos_compound.get("Y").and_then(|o| o.as_i32()) else {
        return;
    };
    let Some(z) = pos_compound.get("Z").and_then(|o| o.as_i32()) else {
        return;
    };

    *pos = JValue::IntArray(vec![x, y, z]);
}
