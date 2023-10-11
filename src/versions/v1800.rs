use crate::helpers::rename::rename_item;
use crate::{static_string_mc_map, types};
use world_transmuter_engine::DataWalkerMapListPaths;

const VERSION: u32 = 1800;

static_string_mc_map! {
    RENAMED_ITEM_IDS, renamed_item_ids, {
        "cactus_green" => "minecraft:green_dye",
        "rose_red" => "minecraft:red_dye",
        "dandelion_yellow" => "minecraft:yellow_dye",
    }
}

pub(crate) fn register() {
    rename_item(VERSION, |name| {
        renamed_item_ids().get(name).map(|&str| str.to_owned())
    });

    types::entity_mut().add_walker_for_id(
        VERSION,
        "minecraft:panda",
        DataWalkerMapListPaths::new_multi(
            types::item_stack_ref(),
            vec!["ArmorItems".to_owned(), "HandItems".to_owned()],
        ),
    );
    types::entity_mut().add_walker_for_id(
        VERSION,
        "minecraft:pillager",
        DataWalkerMapListPaths::new_multi(
            types::item_stack_ref(),
            vec![
                "Inventory".to_owned(),
                "ArmorItems".to_owned(),
                "HandItems".to_owned(),
            ],
        ),
    );
}
