use crate::{static_string_map, types};
use world_transmuter_engine::{map_data_converter_func, rename_keys, JValue};

const VERSION: u32 = 3566;

static_string_map! {
    SLOT_RENAMES, slot_renames, {
        "slot_0" => "list",
        "slot_1" => "sidebar",
        "slot_2" => "below_name",
        "slot_3" => "sidebar.team.black",
        "slot_4" => "sidebar.team.dark_blue",
        "slot_5" => "sidebar.team.dark_green",
        "slot_6" => "sidebar.team.dark_aqua",
        "slot_7" => "sidebar.team.dark_red",
        "slot_8" => "sidebar.team.dark_purple",
        "slot_9" => "sidebar.team.gold",
        "slot_10" => "sidebar.team.gray",
        "slot_11" => "sidebar.team.dark_gray",
        "slot_12" => "sidebar.team.blue",
        "slot_13" => "sidebar.team.green",
        "slot_14" => "sidebar.team.aqua",
        "slot_15" => "sidebar.team.red",
        "slot_16" => "sidebar.team.light_purple",
        "slot_17" => "sidebar.team.yellow",
        "slot_18" => "sidebar.team.white",
    }
}

pub(crate) fn register() {
    types::saved_data_scoreboard_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            let Some(JValue::Compound(data)) = data.get_mut("data") else {
                return;
            };

            if let Some(JValue::Compound(display_slots)) = data.get_mut("DisplaySlots") {
                rename_keys(display_slots, |key| {
                    slot_renames().get(key).copied().map(|val| val.to_owned())
                });
            }
        }),
    );
}
