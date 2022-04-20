use rust_dataconverter_engine::{data_converter_func, MapType, ObjectType, Types};
use crate::MinecraftTypesMut;

const VERSION: u32 = 110;

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    types.entity.borrow_mut().add_converter_for_id("EntityHorse", VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        if data.get_bool("Saddle") != Some(true) || data.get_map("SaddleItem").is_some() {
            return;
        }

        let mut saddle_item = T::Map::create_empty();
        saddle_item.set("id", T::Object::create_string("minecraft:saddle".to_owned()));
        saddle_item.set("Count", T::Object::create_byte(1));
        saddle_item.set("Damage", T::Object::create_short(0));

        data.remove("Saddle");
        data.set("SaddleItem", T::Object::create_map(saddle_item));
    }));
}
