use rust_dataconverter_engine::{data_converter_func, MapType, Types};
use crate::MinecraftTypesMut;

const VERSION: u32 = 147;

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    types.entity.borrow_mut().add_converter_for_id("ArmorStand", VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        if data.get_bool("Silent") == Some(true) && data.get_bool("Marker") != Some(true) {
            data.remove("Silent");
        }
    }));
}
