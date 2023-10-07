use crate::types;
use std::collections::BTreeMap;
use std::sync::OnceLock;
use valence_nbt::Value;
use world_transmuter_engine::{map_data_converter_func, rename_keys};

const VERSION: u32 = 3566;

static SLOT_RENAMES: OnceLock<BTreeMap<&str, &str>> = OnceLock::new();

fn slot_renames() -> &'static BTreeMap<&'static str, &'static str> {
    SLOT_RENAMES.get_or_init(|| {
        let mut map = BTreeMap::new();
        map.insert("slot_0", "list");
        map.insert("slot_1", "sidebar");
        map.insert("slot_2", "below_name");
        map.insert("slot_3", "sidebar.team.black");
        map.insert("slot_4", "sidebar.team.dark_blue");
        map.insert("slot_5", "sidebar.team.dark_green");
        map.insert("slot_6", "sidebar.team.dark_aqua");
        map.insert("slot_7", "sidebar.team.dark_red");
        map.insert("slot_8", "sidebar.team.dark_purple");
        map.insert("slot_9", "sidebar.team.gold");
        map.insert("slot_10", "sidebar.team.gray");
        map.insert("slot_11", "sidebar.team.dark_gray");
        map.insert("slot_12", "sidebar.team.blue");
        map.insert("slot_13", "sidebar.team.green");
        map.insert("slot_14", "sidebar.team.aqua");
        map.insert("slot_15", "sidebar.team.red");
        map.insert("slot_16", "sidebar.team.light_purple");
        map.insert("slot_17", "sidebar.team.yellow");
        map.insert("slot_18", "sidebar.team.white");
        map
    })
}

pub(crate) fn register() {
    types::saved_data_scoreboard_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            let Some(Value::Compound(data)) = data.get_mut("data") else {
                return;
            };

            if let Some(Value::Compound(display_slots)) = data.get_mut("DisplaySlots") {
                rename_keys(display_slots, |key| {
                    slot_renames().get(key).copied().map(|val| val.to_owned())
                });
            }
        }),
    );
}
