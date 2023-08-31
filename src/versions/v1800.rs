use crate::helpers::mc_namespace_map::McNamespaceMap;
use crate::helpers::rename::rename_item;
use crate::MinecraftTypesMut;
use rust_dataconverter_engine::DataWalkerMapListPaths;
use std::sync::OnceLock;

const VERSION: u32 = 1800;

static RENAMED_ITEM_IDS: OnceLock<McNamespaceMap<&'static str>> = OnceLock::new();

fn renamed_item_ids() -> &'static McNamespaceMap<'static, &'static str> {
    RENAMED_ITEM_IDS.get_or_init(|| {
        let mut map = McNamespaceMap::new();
        map.insert_mc("cactus_green", "minecraft:green_dye");
        map.insert_mc("rose_red", "minecraft:red_dye");
        map.insert_mc("dandelion_yellow", "minecraft:yellow_dye");
        map
    })
}

pub(crate) fn register(types: MinecraftTypesMut) {
    rename_item(types, VERSION, |name| {
        renamed_item_ids().get(name).map(|&str| str.to_owned())
    });

    types.entity().borrow_mut().add_walker_for_id(
        VERSION,
        "minecraft:panda",
        DataWalkerMapListPaths::new_multi(
            types.item_stack(),
            vec!["ArmorItems".to_owned(), "HandItems".to_owned()],
        ),
    );
    types.entity().borrow_mut().add_walker_for_id(
        VERSION,
        "minecraft:pillager",
        DataWalkerMapListPaths::new_multi(
            types.item_stack(),
            vec![
                "Inventory".to_owned(),
                "ArmorItems".to_owned(),
                "HandItems".to_owned(),
            ],
        ),
    );
}
