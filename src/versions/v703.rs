use rust_dataconverter_engine::{data_converter_func, DataWalkerMapListPaths, DataWalkerMapTypePaths, MapType, ObjectType, Types};
use crate::MinecraftTypesMut;

const VERSION: u32 = 703;

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    types.entity.borrow_mut().add_converter_for_id("EntityHorse", VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        match data.remove("Type").and_then(|o| o.as_i64()) {
            Some(1) => data.set("id", T::Object::create_string("Donkey".to_owned())),
            Some(2) => data.set("id", T::Object::create_string("Mule".to_owned())),
            Some(3) => data.set("id", T::Object::create_string("ZombieHorse".to_owned())),
            Some(4) => data.set("id", T::Object::create_string("SkeletonHorse".to_owned())),
            _ => data.set("id", T::Object::create_string("Horse".to_owned()))
        }
    }));

    types.entity.borrow_mut().add_walker_for_id(VERSION, "Horse", DataWalkerMapTypePaths::new_multi(types.item_stack, vec!["ArmorItem".to_owned(), "SaddleItem".to_owned()]));
    types.entity.borrow_mut().add_walker_for_id(VERSION, "Horse", DataWalkerMapListPaths::new_multi(types.item_stack, vec!["ArmorItems".to_owned(), "HandItems".to_owned()]));

    types.entity.borrow_mut().add_walker_for_id(VERSION, "Donkey", DataWalkerMapTypePaths::new(types.item_stack, "SaddleItem"));
    types.entity.borrow_mut().add_walker_for_id(VERSION, "Donkey", DataWalkerMapListPaths::new_multi(types.item_stack, vec!["Items".to_owned(), "ArmorItems".to_owned(), "HandItems".to_owned()]));

    types.entity.borrow_mut().add_walker_for_id(VERSION, "Mule", DataWalkerMapTypePaths::new(types.item_stack, "SaddleItem"));
    types.entity.borrow_mut().add_walker_for_id(VERSION, "Mule", DataWalkerMapListPaths::new_multi(types.item_stack, vec!["Items".to_owned(), "ArmorItems".to_owned(), "HandItems".to_owned()]));

    types.entity.borrow_mut().add_walker_for_id(VERSION, "ZombieHorse", DataWalkerMapTypePaths::new(types.item_stack, "SaddleItem"));
    types.entity.borrow_mut().add_walker_for_id(VERSION, "ZombieHorse", DataWalkerMapListPaths::new_multi(types.item_stack, vec!["ArmorItems".to_owned(), "HandItems".to_owned()]));

    types.entity.borrow_mut().add_walker_for_id(VERSION, "SkeletonHorse", DataWalkerMapTypePaths::new(types.item_stack, "SaddleItem"));
    types.entity.borrow_mut().add_walker_for_id(VERSION, "SkeletonHorse", DataWalkerMapListPaths::new_multi(types.item_stack, vec!["ArmorItems".to_owned(), "HandItems".to_owned()]));
}
