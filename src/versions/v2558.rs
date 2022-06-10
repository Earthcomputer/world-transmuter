use rust_dataconverter_engine::{data_converter_func, MapType, ObjectType, Types};
use crate::helpers::rename::{rename_option, simple_rename};
use crate::MinecraftTypesMut;
use crate::versions::v2550;

const VERSION: u32 = 2558;

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    rename_option(types, VERSION, simple_rename("key_key.swapHands", "key_key.swapOffhand"));

    types.world_gen_settings.borrow_mut().add_structure_converter(VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        if data.get_map("dimensions").map(|o| o.is_empty()).unwrap_or(true) {
            let new_dimensions = recreate_settings::<T>(data);
            data.set("dimensions", T::Object::create_map(new_dimensions));
        }
    }));
}

fn recreate_settings<T: Types + ?Sized>(data: &T::Map) -> T::Map {
    let seed = data.get_i64("seed").unwrap_or(0);
    v2550::vanilla_levels::<T>(seed, v2550::default_overworld::<T>(seed), false)
}
