use crate::helpers::rename::rename_item;
use crate::versions::v100;
use crate::{static_string_mc_map, types};
use world_transmuter_engine::DataWalkerMapListPaths;

const VERSION: u32 = 1800;

static_string_mc_map! {
    renamed_item_ids = {
        "cactus_green" => "minecraft:green_dye",
        "rose_red" => "minecraft:red_dye",
        "dandelion_yellow" => "minecraft:yellow_dye",
    }
}

pub(crate) fn register() {
    rename_item(VERSION, |name| {
        renamed_item_ids().get(name).map(|&str| str.to_owned())
    });

    v100::register_equipment(VERSION, "minecraft:panda");
    types::entity_mut().add_walker_for_id(
        VERSION,
        "minecraft:pillager",
        DataWalkerMapListPaths::new(types::item_stack_ref(), "Inventory"),
    );
    v100::register_equipment(VERSION, "minecraft:pillager");
}
