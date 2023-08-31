use crate::MinecraftTypesMut;
use rust_dataconverter_engine::*;
use valence_nbt::{Compound, List, Value};

const VERSION: u32 = 100;

fn register_mob(types: MinecraftTypesMut, id: impl Into<String>) {
    types.entity().borrow_mut().add_walker_for_id(
        VERSION,
        id,
        DataWalkerMapListPaths::new_multi(
            types.item_stack(),
            vec!["ArmorItems".to_owned(), "HandItems".to_owned()],
        ),
    );
}

pub(crate) fn register(types: MinecraftTypesMut) {
    types.entity().borrow_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            if let Some(Value::List(equipment)) = data.remove("Equipment") {
                let mut equipment = equipment.into_iter();
                if let Some(hand_item) = equipment.next() {
                    if !matches!(data.get("HandItems"), Some(Value::List(_))) {
                        if let Value::Compound(hand_item) = hand_item {
                            data.insert(
                                "HandItems",
                                List::Compound(vec![hand_item, Compound::new()]),
                            );
                        }
                    }
                    if !matches!(data.get("ArmorItems"), Some(Value::List(_))) {
                        let mut armor_items = List::new();
                        for armor_item in equipment.take(4) {
                            let _ = armor_items.try_push(armor_item);
                        }
                        if !armor_items.is_empty() {
                            data.insert("ArmorItems", armor_items);
                        }
                    }
                }
            }

            if let Some(Value::List(drop_chances)) = data.remove("DropChances") {
                let mut drop_chances = drop_chances.into_iter();
                let mut hand_drop_chances = List::new();
                let mut armor_drop_chances = List::new();
                if let Some(drop_chance) = drop_chances.next() {
                    let _ = hand_drop_chances.try_push(drop_chance);
                } else {
                    let _ = hand_drop_chances.try_push(0.0f32);
                }
                let _ = hand_drop_chances.try_push(0.0f32);
                for drop_chance in drop_chances.take(4) {
                    let _ = armor_drop_chances.try_push(drop_chance);
                }
                if let List::Float(armor_drop_chances) = &mut armor_drop_chances {
                    while armor_drop_chances.len() < 4 {
                        armor_drop_chances.push(0.0);
                    }
                }

                if !matches!(data.get("HandDropChances"), Some(Value::List(_))) {
                    data.insert("HandDropChances", hand_drop_chances);
                }
                if !matches!(data.get("ArmorDropChances"), Some(Value::List(_))) {
                    data.insert("ArmorDropChances", armor_drop_chances);
                }
            }
        }),
    );

    register_mob(types, "ArmorStand");
    register_mob(types, "Creeper");
    register_mob(types, "Skeleton");
    register_mob(types, "Spider");
    register_mob(types, "Giant");
    register_mob(types, "Zombie");
    register_mob(types, "Slime");
    register_mob(types, "Ghast");
    register_mob(types, "PigZombie");
    register_mob(types, "Enderman");
    types.entity().borrow_mut().add_walker_for_id(
        VERSION,
        "Enderman",
        DataWalkerObjectTypePaths::new(types.block_name(), "carried"),
    );
    register_mob(types, "CaveSpider");
    register_mob(types, "Silverfish");
    register_mob(types, "Blaze");
    register_mob(types, "LavaSlime");
    register_mob(types, "EnderDragon");
    register_mob(types, "WitherBoss");
    register_mob(types, "Bat");
    register_mob(types, "Witch");
    register_mob(types, "Endermite");
    register_mob(types, "Guardian");
    register_mob(types, "Pig");
    register_mob(types, "Sheep");
    register_mob(types, "Cow");
    register_mob(types, "Chicken");
    register_mob(types, "Squid");
    register_mob(types, "Wolf");
    register_mob(types, "MushroomCow");
    register_mob(types, "SnowMan");
    register_mob(types, "Ozelot");
    register_mob(types, "VillagerGolem");
    types.entity().borrow_mut().add_walker_for_id(
        VERSION,
        "EntityHorse",
        DataWalkerMapListPaths::new_multi(
            types.item_stack(),
            vec![
                "Items".to_owned(),
                "HandItems".to_owned(),
                "ArmorItems".to_owned(),
            ],
        ),
    );
    types.entity().borrow_mut().add_walker_for_id(
        VERSION,
        "EntityHorse",
        DataWalkerMapTypePaths::new_multi(
            types.item_stack(),
            vec!["ArmorItem".to_owned(), "SaddleItem".to_owned()],
        ),
    );
    register_mob(types, "Rabbit");
    let item_stack_type = types.item_stack();
    types.entity().borrow_mut().add_walker_for_id(
        VERSION,
        "Villager",
        data_walker(move |data, from_version, to_version| {
            if let Some(Value::Compound(offers)) = data.get_mut("Offers") {
                if let Some(Value::List(List::Compound(recipes))) = offers.get_mut("Recipes") {
                    for recipe in recipes {
                        convert_map_in_map(
                            item_stack_type,
                            recipe,
                            "buy",
                            from_version,
                            to_version,
                        );
                        convert_map_in_map(
                            item_stack_type,
                            recipe,
                            "buyB",
                            from_version,
                            to_version,
                        );
                        convert_map_in_map(
                            item_stack_type,
                            recipe,
                            "sell",
                            from_version,
                            to_version,
                        );
                    }
                }
            }

            convert_map_list_in_map(
                item_stack_type,
                data,
                "ArmorItems",
                from_version,
                to_version,
            );
            convert_map_list_in_map(item_stack_type, data, "HandItems", from_version, to_version);
        }),
    );
    register_mob(types, "Shulker");

    let block_state_type = types.block_state();
    let entity_type = types.entity();
    let tile_entity_type = types.tile_entity();
    types.structure().borrow_mut().add_structure_walker(
        VERSION,
        data_walker(move |data, from_version, to_version| {
            if let Some(Value::List(List::Compound(entities))) = data.get_mut("entities") {
                for entity in entities {
                    convert_map_in_map(entity_type, entity, "nbt", from_version, to_version);
                }
            }

            if let Some(Value::List(List::Compound(blocks))) = data.get_mut("blocks") {
                for block in blocks {
                    convert_map_in_map(tile_entity_type, block, "nbt", from_version, to_version);
                }
            }

            convert_map_list_in_map(block_state_type, data, "palette", from_version, to_version);
        }),
    );
}
