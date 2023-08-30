use crate::MinecraftTypes;
use rust_dataconverter_engine::{
    map_data_converter_func, value_data_converter_func, AbstractValueDataType, DataVersion,
};
use valence_nbt::value::ValueMut;
use valence_nbt::{Compound, List, Value};

pub(crate) fn rename_entity<'a>(
    types: &'a MinecraftTypes<'a>,
    version: impl Into<DataVersion>,
    renamer: impl 'a + Copy + Fn(&str) -> Option<String>,
) {
    let version = version.into();
    types.entity.borrow_mut().add_structure_converter(
        version,
        map_data_converter_func(move |data, _from_version, _to_version| {
            if let Some(Value::String(id)) = data.get_mut("id") {
                if let Some(new_id) = renamer(id) {
                    *id = new_id;
                }
            }
        }),
    );
    types.entity_name.borrow_mut().add_structure_converter(
        version,
        value_data_converter_func(move |data, _from_version, _to_version| {
            if let ValueMut::String(id) = data {
                if let Some(new_id) = renamer(&id[..]) {
                    **id = new_id;
                }
            }
        }),
    );
}

pub(crate) fn rename_tile_entity<'a>(
    types: &'a MinecraftTypes<'a>,
    version: impl Into<DataVersion>,
    renamer: impl 'a + Copy + Fn(&str) -> Option<String>,
) {
    types.tile_entity.borrow_mut().add_structure_converter(
        version,
        map_data_converter_func(move |data, _from_version, _to_version| {
            let Some(Value::String(id)) = data.get_mut("id") else {
                return;
            };
            if let Some(new_name) = renamer(id) {
                *id = new_name;
            }
        }),
    );
}

pub(crate) fn rename_block<'a>(
    types: &'a MinecraftTypes<'a>,
    version: impl Into<DataVersion>,
    renamer: impl 'a + Copy + Fn(&str) -> Option<String>,
) {
    let version = version.into();
    types.block_state.borrow_mut().add_structure_converter(
        version,
        map_data_converter_func(move |data, _from_version, _to_version| {
            if let Some(Value::String(name)) = data.get_mut("Name") {
                if let Some(new_name) = renamer(name) {
                    *name = new_name;
                }
            }
        }),
    );
    types.block_name.borrow_mut().add_structure_converter(
        version,
        value_data_converter_func(move |data, _from_verison, _to_version| {
            if let ValueMut::String(id) = data {
                if let Some(new_id) = renamer(&id[..]) {
                    **id = new_id;
                }
            }
        }),
    );
}

pub(crate) fn rename_block_and_fix_jigsaw<'a>(
    types: &'a MinecraftTypes<'a>,
    version: impl Into<DataVersion>,
    renamer: impl 'a + Copy + Fn(&str) -> Option<String>,
) {
    let version = version.into();
    rename_block(types, version, renamer);

    types.tile_entity.borrow_mut().add_converter_for_id(
        "minecraft:jigsaw",
        version,
        map_data_converter_func(move |data, _from_version, _to_version| {
            if let Some(Value::String(final_state)) = data.get_mut("final_state") {
                if !final_state.is_empty() {
                    let state_name_end = if let Some(nbt_start) = final_state[1..].find(['[', '{'])
                    {
                        nbt_start + 1
                    } else {
                        final_state.len()
                    };

                    if let Some(mut converted) = renamer(&final_state[..state_name_end]) {
                        converted.push_str(&final_state[state_name_end..]);
                        *final_state = converted;
                    }
                }
            }
        }),
    );
}

pub(crate) fn rename_item<'a>(
    types: &'a MinecraftTypes<'a>,
    version: impl Into<DataVersion>,
    renamer: impl 'a + Copy + Fn(&str) -> Option<String>,
) {
    types.item_name.borrow_mut().add_structure_converter(
        version,
        value_data_converter_func(move |data, _from_version, _to_version| {
            if let ValueMut::String(name) = data {
                if let Some(new_name) = renamer(&name[..]) {
                    **name = new_name;
                }
            }
        }),
    );
}

pub(crate) fn rename_advancement<'a>(
    types: &'a MinecraftTypes<'a>,
    version: impl Into<DataVersion>,
    renamer: impl 'a + Copy + Fn(&str) -> Option<String>,
) {
    types.advancements.borrow_mut().add_structure_converter(
        version,
        map_data_converter_func(move |data, _from_version, _to_version| {
            rust_dataconverter_engine::rename_keys(data, renamer);
        }),
    );
}

pub(crate) fn rename_criteria<'a>(
    types: &'a MinecraftTypes<'a>,
    version: impl Into<DataVersion>,
    advancement: &'a str,
    renamer: impl 'a + Copy + Fn(&str) -> Option<String>,
) {
    types.advancements.borrow_mut().add_structure_converter(
        version,
        map_data_converter_func(move |data, _from_version, _to_version| {
            let Some(Value::Compound(advancement)) = data.get_mut(advancement) else {
                return;
            };
            let Some(Value::Compound(criteria)) = advancement.get_mut("criteria") else {
                return;
            };
            rust_dataconverter_engine::rename_keys(criteria, renamer);
        }),
    );
}

pub(crate) fn rename_recipe<'a>(
    types: &'a MinecraftTypes<'a>,
    version: impl Into<DataVersion>,
    renamer: impl 'a + Copy + Fn(&str) -> Option<String>,
) {
    types.recipe.borrow_mut().add_structure_converter(
        version,
        value_data_converter_func(move |data, _from_version, _to_version| {
            if let ValueMut::String(name) = data {
                if let Some(new_name) = renamer(&name[..]) {
                    **name = new_name;
                }
            }
        }),
    );
}

pub(crate) fn rename_stat<'a>(
    types: &'a MinecraftTypes<'a>,
    version: impl Into<DataVersion>,
    renamer: impl 'a + Copy + Fn(&str) -> Option<String>,
) {
    let version = version.into();
    types.objective.borrow_mut().add_structure_converter(version, map_data_converter_func(move |data, _from_version, _to_version| {
        let Some(Value::Compound(criteria_type)) = data.get_mut("CriteriaType") else { return };
        if matches!(criteria_type.get("type"), Some(Value::String(typ)) if typ == "minecraft:custom") {
            let Some(Value::String(id)) = criteria_type.get("id") else { return };
            let Some(new_id) = renamer(id) else { return };
            criteria_type.insert("id", new_id);
        }
    }));
    types.stats.borrow_mut().add_structure_converter(
        version,
        map_data_converter_func(move |data, _from_version, _to_version| {
            let Some(Value::Compound(stats)) = data.get_mut("stats") else {
                return;
            };
            let Some(Value::Compound(custom)) = stats.get_mut("minecraft:custom") else {
                return;
            };
            rust_dataconverter_engine::rename_keys(custom, renamer);
        }),
    );
}

pub(crate) fn rename_option<'a>(
    types: &'a MinecraftTypes<'a>,
    version: impl Into<DataVersion>,
    renamer: impl 'a + Copy + Fn(&str) -> Option<String>,
) {
    types.options.borrow_mut().add_structure_converter(
        version,
        map_data_converter_func(move |data, _from_version, _to_version| {
            rust_dataconverter_engine::rename_keys(data, renamer);
        }),
    );
}

pub(crate) fn rename_poi<'a>(
    types: &'a MinecraftTypes<'a>,
    version: impl Into<DataVersion>,
    renamer: impl 'a + Copy + Fn(&str) -> Option<String>,
) {
    types.poi_chunk.borrow_mut().add_structure_converter(
        version,
        map_data_converter_func(move |data, _from_version, _to_version| {
            let Some(Value::Compound(sections)) = data.get_mut("Sections") else {
                return;
            };
            for section in sections.values_mut() {
                let Value::Compound(section) = section else {
                    continue;
                };
                let Some(Value::List(List::Compound(records))) = section.get_mut("Records") else {
                    continue;
                };
                for record in records {
                    let Some(Value::String(typ)) = record.get_mut("type") else {
                        continue;
                    };
                    let Some(new_type) = renamer(typ) else {
                        continue;
                    };
                    *typ = new_type;
                }
            }
        }),
    );
}

pub(crate) fn simple_rename<'a>(
    from: &'a str,
    to: &'a str,
) -> impl 'a + Copy + Fn(&str) -> Option<String> {
    move |name| {
        if name == from {
            Some(to.to_owned())
        } else {
            None
        }
    }
}

pub(crate) fn rename_keys_in_map(
    typ: impl AbstractValueDataType,
    owning_map: &mut Compound,
    key: &str,
    from_version: DataVersion,
    to_version: DataVersion,
) {
    if let Some(Value::Compound(map)) = owning_map.get_mut(key) {
        rename_keys(typ, map, from_version, to_version);
    }
}

pub(crate) fn rename_keys(
    typ: impl AbstractValueDataType,
    map: &mut Compound,
    from_version: DataVersion,
    to_version: DataVersion,
) {
    rust_dataconverter_engine::rename_keys(map, move |key| {
        let mut new_key = key.to_owned();
        typ.convert(
            &mut ValueMut::String(&mut new_key),
            from_version,
            to_version,
        );
        if new_key != key {
            Some(new_key)
        } else {
            None
        }
    });
}
