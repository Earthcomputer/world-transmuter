use crate::helpers::rename::rename_criteria;
use crate::types::MinecraftTypesMut;
use rust_dataconverter_engine::map_data_converter_func;
use valence_nbt::value::ValueMut;
use valence_nbt::{List, Value};

const VERSION: u32 = 3097;

pub(crate) fn register(types: MinecraftTypesMut) {
    for item_id in ["minecraft:writable_book", "minecraft:written_book"] {
        types.item_stack().borrow_mut().add_converter_for_id(
            item_id,
            VERSION,
            map_data_converter_func(|data, _from_version, _to_version| {
                let Some(Value::Compound(tag)) = data.get_mut("tag") else {
                    return;
                };
                tag.remove("filtered_title");
                tag.remove("filtered_pages");
            }),
        );
    }

    types.tile_entity().borrow_mut().add_converter_for_id(
        "minecraft:sign",
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            data.remove("FilteredText1");
            data.remove("FilteredText2");
            data.remove("FilteredText3");
            data.remove("FilteredText4");
        }),
    );

    types.entity().borrow_mut().add_converter_for_id(
        "minecraft:cat",
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            let Some(Value::String(variant)) = data.get_mut("variant") else {
                return;
            };
            if variant == "minecraft:british" {
                *variant = "minecraft:british_shorthair".to_owned();
            }
        }),
    );
    rename_criteria(
        types,
        VERSION,
        "minecraft:husbandry/complete_catalogue",
        |name| {
            if name == "minecraft:british" {
                Some("minecraft:british_shorthair".to_owned())
            } else {
                None
            }
        },
    );

    types.poi_chunk().borrow_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            let Some(Value::Compound(sections)) = data.get_mut("Sections") else {
                return;
            };
            for section in sections.values_mut() {
                let Value::Compound(section) = section else {
                    continue;
                };
                let Some(Value::List(records)) = section.get_mut("Records") else {
                    continue;
                };
                list_retain(records, |record| {
                    let ValueMut::Compound(record) = record else {
                        return false;
                    };
                    if let Some(Value::String(typ)) = record.get("type") {
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

// TODO: remove when valence_nbt::List::retain is added
fn list_retain<F>(list: &mut List, mut f: F)
where
    F: FnMut(ValueMut) -> bool,
{
    match list {
        List::End => {}
        List::Byte(list) => list.retain_mut(|v| f(ValueMut::Byte(v))),
        List::Short(list) => list.retain_mut(|v| f(ValueMut::Short(v))),
        List::Int(list) => list.retain_mut(|v| f(ValueMut::Int(v))),
        List::Long(list) => list.retain_mut(|v| f(ValueMut::Long(v))),
        List::Float(list) => list.retain_mut(|v| f(ValueMut::Float(v))),
        List::Double(list) => list.retain_mut(|v| f(ValueMut::Double(v))),
        List::ByteArray(list) => list.retain_mut(|v| f(ValueMut::ByteArray(v))),
        List::String(list) => list.retain_mut(|v| f(ValueMut::String(v))),
        List::List(list) => list.retain_mut(|v| f(ValueMut::List(v))),
        List::Compound(list) => list.retain_mut(|v| f(ValueMut::Compound(v))),
        List::IntArray(list) => list.retain_mut(|v| f(ValueMut::IntArray(v))),
        List::LongArray(list) => list.retain_mut(|v| f(ValueMut::LongArray(v))),
    }

    if list.is_empty() {
        *list = List::End;
    }
}
