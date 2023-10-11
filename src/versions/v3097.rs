use crate::helpers::rename::rename_criteria;
use crate::types;
use java_string::JavaString;
use world_transmuter_engine::{map_data_converter_func, JValue, JValueMut};

const VERSION: u32 = 3097;

pub(crate) fn register() {
    for item_id in ["minecraft:writable_book", "minecraft:written_book"] {
        types::item_stack_mut().add_converter_for_id(
            item_id,
            VERSION,
            map_data_converter_func(|data, _from_version, _to_version| {
                let Some(JValue::Compound(tag)) = data.get_mut("tag") else {
                    return;
                };
                tag.remove("filtered_title");
                tag.remove("filtered_pages");
            }),
        );
    }

    types::tile_entity_mut().add_converter_for_id(
        "minecraft:sign",
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            data.remove("FilteredText1");
            data.remove("FilteredText2");
            data.remove("FilteredText3");
            data.remove("FilteredText4");
        }),
    );

    types::entity_mut().add_converter_for_id(
        "minecraft:cat",
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            let Some(JValue::String(variant)) = data.get_mut("variant") else {
                return;
            };
            if variant == "minecraft:british" {
                *variant = JavaString::from("minecraft:british_shorthair");
            }
        }),
    );
    rename_criteria(VERSION, "minecraft:husbandry/complete_catalogue", |name| {
        if name == "minecraft:british" {
            Some(JavaString::from("minecraft:british_shorthair"))
        } else {
            None
        }
    });

    types::poi_chunk_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            let Some(JValue::Compound(sections)) = data.get_mut("Sections") else {
                return;
            };
            for section in sections.values_mut() {
                let JValue::Compound(section) = section else {
                    continue;
                };
                let Some(JValue::List(records)) = section.get_mut("Records") else {
                    continue;
                };
                records.retain(|record| {
                    let JValueMut::Compound(record) = record else {
                        return false;
                    };
                    if let Some(JValue::String(typ)) = record.get("type") {
                        if typ == "minecraft:unemployed" || typ == "minecraft:nitwit" {
                            return false;
                        }
                    }
                    true
                });
            }
        }),
    );
}
