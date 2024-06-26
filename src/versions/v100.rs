use crate::types;
use java_string::JavaString;
use world_transmuter_engine::*;

const VERSION: u32 = 100;

pub(crate) fn register_equipment(
    version: impl Into<DataVersion>,
    entity_id: impl Into<JavaString>,
) {
    let version = version.into();
    let entity_id = entity_id.into();
    types::entity_mut().add_walker_for_id(
        version,
        &entity_id,
        DataWalkerMapListPaths::new_multi(
            types::item_stack_ref(),
            vec!["ArmorItems".to_owned(), "HandItems".to_owned()],
        ),
    );
    types::entity_mut().add_walker_for_id(
        version,
        entity_id,
        DataWalkerMapTypePaths::new(types::item_stack_ref(), "body_armor_item"),
    );
}

fn register_mob(id: impl Into<JavaString>) {
    register_equipment(VERSION, id);
}

pub(crate) fn register() {
    types::entity_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            if let Some(JValue::List(equipment)) = data.remove("Equipment") {
                let mut equipment = equipment.into_iter();
                if let Some(hand_item) = equipment.next() {
                    if !matches!(data.get("HandItems"), Some(JValue::List(_))) {
                        if let JValue::Compound(hand_item) = hand_item {
                            data.insert(
                                "HandItems",
                                JList::Compound(vec![hand_item, JCompound::new()]),
                            );
                        }
                    }
                    if !matches!(data.get("ArmorItems"), Some(JValue::List(_))) {
                        let mut armor_items = JList::new();
                        for armor_item in equipment.take(4) {
                            let _ = armor_items.try_push(armor_item);
                        }
                        if !armor_items.is_empty() {
                            data.insert("ArmorItems", armor_items);
                        }
                    }
                }
            }

            if let Some(JValue::List(drop_chances)) = data.remove("DropChances") {
                let mut drop_chances = drop_chances.into_iter();
                let mut hand_drop_chances = JList::new();
                let mut armor_drop_chances = JList::new();
                if let Some(drop_chance) = drop_chances.next() {
                    let _ = hand_drop_chances.try_push(drop_chance);
                } else {
                    let _ = hand_drop_chances.try_push(0.0f32);
                }
                let _ = hand_drop_chances.try_push(0.0f32);
                for drop_chance in drop_chances.take(4) {
                    let _ = armor_drop_chances.try_push(drop_chance);
                }
                if let JList::Float(armor_drop_chances) = &mut armor_drop_chances {
                    while armor_drop_chances.len() < 4 {
                        armor_drop_chances.push(0.0);
                    }
                }

                if !matches!(data.get("HandDropChances"), Some(JValue::List(_))) {
                    data.insert("HandDropChances", hand_drop_chances);
                }
                if !matches!(data.get("ArmorDropChances"), Some(JValue::List(_))) {
                    data.insert("ArmorDropChances", armor_drop_chances);
                }
            }
        }),
    );

    register_mob("ArmorStand");
    register_mob("Creeper");
    register_mob("Skeleton");
    register_mob("Spider");
    register_mob("Giant");
    register_mob("Zombie");
    register_mob("Slime");
    register_mob("Ghast");
    register_mob("PigZombie");
    register_mob("Enderman");
    types::entity_mut().add_walker_for_id(
        VERSION,
        "Enderman",
        DataWalkerObjectTypePaths::new(types::block_name_ref(), "carried"),
    );
    register_mob("CaveSpider");
    register_mob("Silverfish");
    register_mob("Blaze");
    register_mob("LavaSlime");
    register_mob("EnderDragon");
    register_mob("WitherBoss");
    register_mob("Bat");
    register_mob("Witch");
    register_mob("Endermite");
    register_mob("Guardian");
    register_mob("Pig");
    register_mob("Sheep");
    register_mob("Cow");
    register_mob("Chicken");
    register_mob("Squid");
    register_mob("Wolf");
    register_mob("MushroomCow");
    register_mob("SnowMan");
    register_mob("Ozelot");
    register_mob("VillagerGolem");
    types::entity_mut().add_walker_for_id(
        VERSION,
        "EntityHorse",
        DataWalkerMapListPaths::new_multi(
            types::item_stack_ref(),
            vec![
                "Items".to_owned(),
                "HandItems".to_owned(),
                "ArmorItems".to_owned(),
            ],
        ),
    );
    types::entity_mut().add_walker_for_id(
        VERSION,
        "EntityHorse",
        DataWalkerMapTypePaths::new_multi(
            types::item_stack_ref(),
            vec!["ArmorItem".to_owned(), "SaddleItem".to_owned()],
        ),
    );
    register_mob("Rabbit");
    types::entity_mut().add_walker_for_id(
        VERSION,
        "Villager",
        map_data_walker(move |data, from_version, to_version| {
            if let Some(JValue::Compound(offers)) = data.get_mut("Offers") {
                if let Some(JValue::List(JList::Compound(recipes))) = offers.get_mut("Recipes") {
                    for recipe in recipes {
                        convert_map_in_map(
                            types::item_stack_ref(),
                            recipe,
                            "buy",
                            from_version,
                            to_version,
                        );
                        convert_map_in_map(
                            types::item_stack_ref(),
                            recipe,
                            "buyB",
                            from_version,
                            to_version,
                        );
                        convert_map_in_map(
                            types::item_stack_ref(),
                            recipe,
                            "sell",
                            from_version,
                            to_version,
                        );
                    }
                }
            }

            convert_map_list_in_map(
                types::item_stack_ref(),
                data,
                "ArmorItems",
                from_version,
                to_version,
            );
            convert_map_list_in_map(
                types::item_stack_ref(),
                data,
                "HandItems",
                from_version,
                to_version,
            );
        }),
    );
    register_mob("Shulker");

    types::structure_mut().add_structure_walker(
        VERSION,
        map_data_walker(move |data, from_version, to_version| {
            if let Some(JValue::List(JList::Compound(entities))) = data.get_mut("entities") {
                for entity in entities {
                    convert_map_in_map(
                        types::entity_ref(),
                        entity,
                        "nbt",
                        from_version,
                        to_version,
                    );
                }
            }

            if let Some(JValue::List(JList::Compound(blocks))) = data.get_mut("blocks") {
                for block in blocks {
                    convert_map_in_map(
                        types::tile_entity_ref(),
                        block,
                        "nbt",
                        from_version,
                        to_version,
                    );
                }
            }

            convert_map_list_in_map(
                types::block_state_ref(),
                data,
                "palette",
                from_version,
                to_version,
            );
        }),
    );
}
