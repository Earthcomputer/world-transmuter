use rust_dataconverter_engine::{data_converter_func, MapType, ObjectType, Types};
use crate::MinecraftTypesMut;

const VERSION: u32 = 107;

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    types.entity.borrow_mut().add_converter_for_id("Minecart", VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        let typ = data.remove("Type").and_then(|o| o.as_i64()).unwrap_or(0);
        // Vanilla does not use all of the IDs here. The legacy (pre DFU) code does, so I'm going to use them.
        // No harm in catching more cases here.
        let id = match typ {
            1 => "MinecartChest",
            2 => "MinecartFurnace",
            3 => "MinecartTNT",
            4 => "MinecartSpawner",
            5 => "MinecartHopper",
            6 => "MinecartCommandBlock",
            _ => "MinecartRideable"
        };
        data.set("id", T::Object::create_string(id.to_owned()));
    }));
}
