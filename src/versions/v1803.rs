use rust_dataconverter_engine::{data_converter_func, ListType, MapType, ObjectType, Types};
use crate::MinecraftTypesMut;

const VERSION: u32 = 1803;

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    types.item_stack.borrow_mut().add_structure_converter(VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        let _: Option<_> = try {
            let lore = data.get_map_mut("tag")?.get_map_mut("display")?.get_list_mut("Lore")?;
            for i in 0..lore.size() {
                if let Some(prev_lore) = lore.get(i).as_string() {
                    let new_lore = format!("{{\"text\":\"{}\"}}", prev_lore.replace('\\', "\\\\").replace('"', "\\\""));
                    lore.set(i, T::Object::create_string(new_lore));
                }
            }
        };
    }));
}
