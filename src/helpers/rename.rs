use crate::types;
use java_string::{format_java, JavaStr, JavaString};
use world_transmuter_engine::{
    map_data_converter_func, value_data_converter_func, AbstractValueDataType, DataVersion,
    JCompound, JList, JValue, JValueMut,
};

pub(crate) fn rename_entity(
    version: impl Into<DataVersion>,
    renamer: impl 'static + Copy + Fn(&JavaStr) -> Option<JavaString>,
) {
    let version = version.into();
    types::entity_mut().add_structure_converter(
        version,
        map_data_converter_func(move |data, _from_version, _to_version| {
            if let Some(JValue::String(id)) = data.get_mut("id") {
                if let Some(new_id) = renamer(id) {
                    *id = new_id;
                }
            }
        }),
    );
    types::entity_name_mut().add_structure_converter(
        version,
        value_data_converter_func(move |data, _from_version, _to_version| {
            if let JValueMut::String(id) = data {
                if let Some(new_id) = renamer(&id[..]) {
                    **id = new_id;
                }
            }
        }),
    );
}

pub(crate) fn rename_tile_entity(
    version: impl Into<DataVersion>,
    renamer: impl 'static + Copy + Fn(&JavaStr) -> Option<JavaString>,
) {
    types::tile_entity_mut().add_structure_converter(
        version,
        map_data_converter_func(move |data, _from_version, _to_version| {
            let Some(JValue::String(id)) = data.get_mut("id") else {
                return;
            };
            if let Some(new_name) = renamer(id) {
                *id = new_name;
            }
        }),
    );
}

pub(crate) fn rename_block(
    version: impl Into<DataVersion>,
    renamer: impl 'static + Copy + Fn(&JavaStr) -> Option<JavaString>,
) {
    let version = version.into();
    types::block_state_mut().add_structure_converter(
        version,
        map_data_converter_func(move |data, _from_version, _to_version| {
            if let Some(JValue::String(name)) = data.get_mut("Name") {
                if let Some(new_name) = renamer(name) {
                    *name = new_name;
                }
            }
        }),
    );
    types::block_name_mut().add_structure_converter(
        version,
        value_data_converter_func(move |data, _from_verison, _to_version| {
            if let JValueMut::String(id) = data {
                if let Some(new_id) = renamer(&id[..]) {
                    **id = new_id;
                }
            }
        }),
    );
    types::flat_block_state_mut().add_structure_converter(
        version,
        value_data_converter_func(move |data, _from_version, _to_version| {
            let JValueMut::String(string) = data else {
                return;
            };
            if string.is_empty() {
                return;
            }
            if let Some(nbt_index) = string.find(['[', '{'].as_slice()) {
                if let Some(new_name) = renamer(&string[..nbt_index]) {
                    string.replace_range_java(..nbt_index, &new_name);
                }
            } else {
                if let Some(new_name) = renamer(string) {
                    **string = new_name;
                }
            }
        }),
    );
}

pub(crate) fn rename_item(
    version: impl Into<DataVersion>,
    renamer: impl 'static + Copy + Fn(&JavaStr) -> Option<JavaString>,
) {
    types::item_name_mut().add_structure_converter(
        version,
        value_data_converter_func(move |data, _from_version, _to_version| {
            if let JValueMut::String(name) = data {
                if let Some(new_name) = renamer(&name[..]) {
                    **name = new_name;
                }
            }
        }),
    );
}

pub(crate) fn rename_advancement(
    version: impl Into<DataVersion>,
    renamer: impl 'static + Copy + Fn(&JavaStr) -> Option<JavaString>,
) {
    types::advancements_mut().add_structure_converter(
        version,
        map_data_converter_func(move |data, _from_version, _to_version| {
            world_transmuter_engine::rename_keys(data, renamer);
        }),
    );
}

pub(crate) fn rename_attribute(
    version: impl Into<DataVersion>,
    renamer: impl 'static + Copy + Fn(&JavaStr) -> Option<JavaString>,
) {
    let version = version.into();

    types::data_components_mut().add_structure_converter(
        version,
        map_data_converter_func(move |data, _from_version, _to_version| {
            let Some(JValue::Compound(attribute_modifiers)) =
                data.get_mut("minecraft:attribute_modifiers")
            else {
                return;
            };
            let Some(JValue::List(JList::Compound(modifiers))) =
                attribute_modifiers.get_mut("modifiers")
            else {
                return;
            };
            for modifier in modifiers {
                if let Some(JValue::String(typ)) = modifier.get_mut("type") {
                    if let Some(new_type) = renamer(typ) {
                        *typ = new_type;
                    }
                }
            }
        }),
    );

    let entity_converter =
        |data: &mut JCompound, _from_version: DataVersion, _to_version: DataVersion| {
            let Some(JValue::List(JList::Compound(modifiers))) = data.get_mut("attributes") else {
                return;
            };
            for modifier in modifiers {
                if let Some(JValue::String(id)) = modifier.get_mut("id") {
                    if let Some(new_id) = renamer(id) {
                        *id = new_id;
                    }
                }
            }
        };
}

pub(crate) fn rename_attribute_old(
    version: impl Into<DataVersion>,
    renamer: impl 'static + Copy + Fn(&JavaStr) -> Option<JavaString>,
) {
    let version = version.into();

    let entity_converter =
        move |data: &mut JCompound, _from_version: DataVersion, _to_version: DataVersion| {
            if let Some(JValue::List(JList::Compound(attributes))) = data.get_mut("Attributes") {
                for attribute in attributes {
                    if let Some(JValue::String(name)) = attribute.get_mut("Name") {
                        if let Some(new_name) = renamer(name) {
                            *name = new_name;
                        }
                    }
                }
            }
        };

    types::entity_mut().add_structure_converter(version, map_data_converter_func(entity_converter));
    types::player_mut().add_structure_converter(version, map_data_converter_func(entity_converter));

    types::item_stack_mut().add_structure_converter(
        version,
        map_data_converter_func(move |data, _from_version, _to_version| {
            if let Some(JValue::List(JList::Compound(attribute_modifiers))) =
                data.get_mut("AttributeModifiers")
            {
                for attribute_modifier in attribute_modifiers {
                    if let Some(JValue::String(attribute_name)) =
                        attribute_modifier.get_mut("AttributeName")
                    {
                        if let Some(new_name) = renamer(attribute_name) {
                            *attribute_name = new_name;
                        }
                    }
                }
            }
        }),
    );
}

pub(crate) fn rename_criteria(
    version: impl Into<DataVersion>,
    advancement: &'static str,
    renamer: impl 'static + Copy + Fn(&JavaStr) -> Option<JavaString>,
) {
    types::advancements_mut().add_structure_converter(
        version,
        map_data_converter_func(move |data, _from_version, _to_version| {
            let Some(JValue::Compound(advancement)) = data.get_mut(advancement) else {
                return;
            };
            let Some(JValue::Compound(criteria)) = advancement.get_mut("criteria") else {
                return;
            };
            world_transmuter_engine::rename_keys(criteria, renamer);
        }),
    );
}

pub(crate) fn rename_enchantment(
    version: impl Into<DataVersion>,
    renamer: impl 'static + Copy + Fn(&JavaStr) -> Option<JavaString>,
) {
    fn rename_enchantment_id(
        enchantment: &mut JCompound,
        renamer: impl FnOnce(&JavaStr) -> Option<JavaString>,
    ) {
        if let Some(JValue::String(id)) = enchantment.get_mut("id") {
            let new_id = if id.contains(':') {
                renamer(id)
            } else {
                renamer(&format_java!("minecraft:{id}"))
            };
            if let Some(new_id) = new_id {
                *id = new_id;
            }
        }
    }

    types::item_stack_mut().add_structure_converter(
        version,
        map_data_converter_func(move |data, _from_version, _to_version| {
            let Some(JValue::Compound(tag)) = data.get_mut("tag") else {
                return;
            };
            if let Some(JValue::List(JList::Compound(enchantments))) = tag.get_mut("Enchantments") {
                for enchantment in enchantments {
                    rename_enchantment_id(enchantment, renamer);
                }
            }
            if let Some(JValue::List(JList::Compound(enchantments))) =
                tag.get_mut("StoredEnchantments")
            {
                for enchantment in enchantments {
                    if let Some(JValue::String(id)) = enchantment.get_mut("id") {
                        rename_enchantment_id(enchantment, renamer);
                    }
                }
            }
        }),
    );
}

pub(crate) fn rename_recipe(
    version: impl Into<DataVersion>,
    renamer: impl 'static + Copy + Fn(&JavaStr) -> Option<JavaString>,
) {
    types::recipe_mut().add_structure_converter(
        version,
        value_data_converter_func(move |data, _from_version, _to_version| {
            if let JValueMut::String(name) = data {
                if let Some(new_name) = renamer(&name[..]) {
                    **name = new_name;
                }
            }
        }),
    );
}

pub(crate) fn rename_stat(
    version: impl Into<DataVersion>,
    renamer: impl 'static + Copy + Fn(&JavaStr) -> Option<JavaString>,
) {
    let version = version.into();
    types::objective_mut().add_structure_converter(version, map_data_converter_func(move |data, _from_version, _to_version| {
        let Some(JValue::Compound(criteria_type)) = data.get_mut("CriteriaType") else { return };
        if matches!(criteria_type.get("type"), Some(JValue::String(typ)) if typ == "minecraft:custom") {
            let Some(JValue::String(id)) = criteria_type.get("id") else { return };
            let Some(new_id) = renamer(id) else { return };
            criteria_type.insert("id", new_id);
        }
    }));
    types::stats_mut().add_structure_converter(
        version,
        map_data_converter_func(move |data, _from_version, _to_version| {
            let Some(JValue::Compound(stats)) = data.get_mut("stats") else {
                return;
            };
            let Some(JValue::Compound(custom)) = stats.get_mut("minecraft:custom") else {
                return;
            };
            world_transmuter_engine::rename_keys(custom, renamer);
        }),
    );
}

pub(crate) fn rename_option(
    version: impl Into<DataVersion>,
    renamer: impl 'static + Copy + Fn(&JavaStr) -> Option<JavaString>,
) {
    types::options_mut().add_structure_converter(
        version,
        map_data_converter_func(move |data, _from_version, _to_version| {
            world_transmuter_engine::rename_keys(data, renamer);
        }),
    );
}

pub(crate) fn rename_poi(
    version: impl Into<DataVersion>,
    renamer: impl 'static + Copy + Fn(&JavaStr) -> Option<JavaString>,
) {
    types::poi_chunk_mut().add_structure_converter(
        version,
        map_data_converter_func(move |data, _from_version, _to_version| {
            let Some(JValue::Compound(sections)) = data.get_mut("Sections") else {
                return;
            };
            for section in sections.values_mut() {
                let JValue::Compound(section) = section else {
                    continue;
                };
                let Some(JValue::List(JList::Compound(records))) = section.get_mut("Records")
                else {
                    continue;
                };
                for record in records {
                    let Some(JValue::String(typ)) = record.get_mut("type") else {
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
    from: &'a (impl AsRef<JavaStr> + ?Sized),
    to: &'a (impl AsRef<JavaStr> + ?Sized),
) -> impl 'a + Copy + Fn(&JavaStr) -> Option<JavaString> {
    let from = from.as_ref();
    let to = to.as_ref();
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
    owning_map: &mut JCompound,
    key: &(impl AsRef<JavaStr> + ?Sized),
    from_version: DataVersion,
    to_version: DataVersion,
) {
    if let Some(JValue::Compound(map)) = owning_map.get_mut(key.as_ref()) {
        rename_keys(typ, map, from_version, to_version);
    }
}

pub(crate) fn rename_keys(
    typ: impl AbstractValueDataType,
    map: &mut JCompound,
    from_version: DataVersion,
    to_version: DataVersion,
) {
    world_transmuter_engine::rename_keys(map, move |key| {
        let mut new_key = key.to_owned();
        typ.convert(
            &mut JValueMut::String(&mut new_key),
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
