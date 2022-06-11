use rust_dataconverter_engine::{data_converter_func, DataWalkerMapListPaths, MapType, ObjectType, Types};
use crate::MinecraftTypesMut;

const VERSION: u32 = 2707;

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    types.world_gen_settings.borrow_mut().add_structure_converter(VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        if !data.has_key("has_increased_height_already") {
            data.set("has_increased_height_already", T::Object::create_bool(true));
        }
    }));

    types.entity.borrow_mut().add_walker_for_id(VERSION, "minecraft:marker", DataWalkerMapListPaths::new_multi(types.item_stack, vec!["ArmorItems".to_owned(), "HandItems".to_owned()])); // ?????????????
}
