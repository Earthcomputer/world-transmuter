use rust_dataconverter_engine::{data_converter_func, ListType, MapType, ObjectType, Types};
use crate::MinecraftTypesMut;

const VERSION: u32 = 113;

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    // Removes "HandDropChances" and "ArmorDropChances" if they're empty.
    types.entity.borrow_mut().add_structure_converter(VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        check_list::<T>(data, "HandDropChances", 2);
        check_list::<T>(data, "ArmorDropChances", 4);
    }));
}

fn check_list<T: Types + ?Sized>(data: &mut T::Map, id: &str, required_length: usize) {
    if let Some(list) = data.get_list(id) {
        if list.size() == required_length {
            for i in 0..required_length {
                if let Some(float) = list.get(i).as_f64() {
                    if float as f32 != 0.0 {
                        return;
                    }
                }
            }
        }
    }

    data.remove(id);
}
