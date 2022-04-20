use rust_dataconverter_engine::*;
use crate::MinecraftTypesMut;

const VERSION: u32 = 100;

fn register_mob<T: Types + ?Sized>(types: &MinecraftTypesMut<T>, id: impl Into<String>) {
    types.entity.borrow_mut().add_walker_for_id(VERSION, id, DataWalkerMapListPaths::new_multi(types.item_stack, vec!["ArmorItems".to_owned(), "HandItems".to_owned()]));
}

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    types.entity.borrow_mut().add_structure_converter(VERSION, data_converter_func::<T::Map, _>(move |data: &mut T::Map, _from_version, _to_version| {
        if let Some(equipment) = data.remove("Equipment") {
            if let Some(equipment) = equipment.into_list() {
                let mut equipment = equipment.into_iter();
                if let Some(hand_item) = equipment.next() {
                    if data.get_list("HandItems").is_none() {
                        let mut hand_items = T::List::create_empty();
                        hand_items.add(hand_item);
                        hand_items.add(T::Object::create_map(T::Map::create_empty()));
                        data.set("HandItems", T::Object::create_list(hand_items));
                    }
                    if data.get_list("ArmorItems").is_none() {
                        let mut armor_items = T::List::create_empty();
                        for armor_item in equipment.take(4) {
                            armor_items.add(armor_item);
                        }
                        if !armor_items.is_empty() {
                            data.set("ArmorItems", T::Object::create_list(armor_items));
                        }
                    }
                }
            }
        }

        if let Some(drop_chances) = data.remove("DropChances") {
            if let Some(drop_chances) = drop_chances.into_list() {
                let mut drop_chances = drop_chances.into_iter();
                let mut hand_drop_chances = T::List::create_empty();
                let mut armor_drop_chances = T::List::create_empty();
                if let Some(drop_chance) = drop_chances.next() {
                    hand_drop_chances.add(drop_chance);
                } else {
                    hand_drop_chances.add(T::Object::create_float(0.0));
                }
                hand_drop_chances.add(T::Object::create_float(0.0));
                for drop_chance in drop_chances.take(4) {
                    armor_drop_chances.add(drop_chance);
                }
                while armor_drop_chances.size() < 4 {
                    armor_drop_chances.add(T::Object::create_float(0.0));
                }

                if data.get_list("HandDropChances").is_none() {
                    data.set("HandDropChances", T::Object::create_list(hand_drop_chances));
                }
                if data.get_list("ArmorDropChances").is_none() {
                    data.set("ArmorDropChances", T::Object::create_list(armor_drop_chances));
                }
            }
        }
    }));

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
    types.entity.borrow_mut().add_walker_for_id(VERSION, "Enderman", DataWalkerObjectTypePaths::new(types.block_name, "carried"));
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
    types.entity.borrow_mut().add_walker_for_id(VERSION, "EntityHorse", DataWalkerMapListPaths::new_multi(types.item_stack, vec!["Items".to_owned(), "HandItems".to_owned(), "ArmorItems".to_owned()]));
    types.entity.borrow_mut().add_walker_for_id(VERSION, "EntityHorse", DataWalkerMapTypePaths::new_multi(types.item_stack, vec!["ArmorItem".to_owned(), "SaddleItem".to_owned()]));
    register_mob(types, "Rabbit");
    let item_stack_type = types.item_stack;
    types.entity.borrow_mut().add_walker_for_id(VERSION, "Villager", data_walker::<T, _>(move |data, from_version, to_version| {
        if let Some(offers) = data.get_map_mut("Offers") {
            if let Some(recipes) = offers.get_list_mut("Recipes") {
                for recipe in recipes.iter_mut() {
                    if let Some(recipe_map) = recipe.as_map_mut() {
                        convert_map_in_map::<_, T>(item_stack_type, recipe_map, "buy", from_version, to_version);
                        convert_map_in_map::<_, T>(item_stack_type, recipe_map, "buyB", from_version, to_version);
                        convert_map_in_map::<_, T>(item_stack_type, recipe_map, "sell", from_version, to_version);
                    }
                }
            }
        }

        convert_map_list_in_map::<_, T>(item_stack_type, data, "ArmorItems", from_version, to_version);
        convert_map_list_in_map::<_, T>(item_stack_type, data, "HandItems", from_version, to_version);
    }));
    register_mob(types, "Shulker");

    let block_state_type = types.block_state;
    let entity_type = types.entity;
    let tile_entity_type = types.tile_entity;
    types.structure.borrow_mut().add_structure_walker(VERSION, data_walker::<T, _>(move |data, from_version, to_version| {
        if let Some(entities) = data.get_list_mut("entities") {
            for entity in entities.iter_mut() {
                if let Some(entity) = entity.as_map_mut() {
                    convert_map_in_map::<_, T>(entity_type, entity, "nbt", from_version, to_version);
                }
            }
        }

        if let Some(blocks) = data.get_list_mut("blocks") {
            for block in blocks.iter_mut() {
                if let Some(block) = block.as_map_mut() {
                    convert_map_in_map::<_, T>(tile_entity_type, block, "nbt", from_version, to_version);
                }
            }
        }

        convert_map_list_in_map::<_, T>(block_state_type, data, "palette", from_version, to_version);
    }));
}
