use rust_dataconverter_engine::{data_converter_func, DataWalkerMapListPaths, MapType, ObjectType, Types};
use crate::MinecraftTypesMut;

const VERSION: u32 = 700;

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    types.entity.borrow_mut().add_converter_for_id("Guardian", VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        if data.remove("Elder").and_then(|o| o.as_bool()) == Some(true) {
            data.set("id", T::Object::create_string("ElderGuardian".to_owned()));
        }
    }));

    register_mob(types, "ElderGuardian");
}

fn register_mob<T: Types + ?Sized>(types: &MinecraftTypesMut<T>, id: impl Into<String>) {
    types.entity.borrow_mut().add_walker_for_id(VERSION, id, DataWalkerMapListPaths::new_multi(types.item_stack, vec!["ArmorItems".to_owned(), "HandItems".to_owned()]));
}
