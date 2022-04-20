use rust_dataconverter_engine::{data_converter_func, DataWalkerMapListPaths, MapType, ObjectType, Types};
use crate::MinecraftTypesMut;

const VERSION: u32 = 701;

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    types.entity.borrow_mut().add_converter_for_id("Skeleton", VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        match data.remove("SkeletonType").and_then(|o| o.as_i64()) {
            Some(1) => data.set("id", T::Object::create_string("WitherSkeleton".to_owned())),
            Some(2) => data.set("id", T::Object::create_string("Stray".to_owned())),
            _ => {}
        }
    }));

    register_mob(types, "WitherSkeleton");
    register_mob(types, "Stray");
}

fn register_mob<T: Types + ?Sized>(types: &MinecraftTypesMut<T>, id: impl Into<String>) {
    types.entity.borrow_mut().add_walker_for_id(VERSION, id, DataWalkerMapListPaths::new_multi(types.item_stack, vec!["ArmorItems".to_owned(), "HandItems".to_owned()]));
}
