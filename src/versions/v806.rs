use std::marker::PhantomData;
use rust_dataconverter_engine::{DataConverterFunc, DataVersion, MapEntry, MapType, ObjectType, Types};
use crate::MinecraftTypesMut;

const VERSION: u32 = 806;

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    struct PotionWaterUpdater<T: Types + ?Sized> {
        phantom: PhantomData<T>,
    }
    impl<T: Types + ?Sized> PotionWaterUpdater<T> {
        fn new() -> Self { Self { phantom: PhantomData } }
    }
    impl<T: Types + ?Sized> DataConverterFunc<T::Map> for PotionWaterUpdater<T> {
        fn convert(&self, data: &mut T::Map, _from_version: DataVersion, _to_version: DataVersion) {
            let tag = data.entry("tag").or_insert_with(|| T::Object::create_map(T::Map::create_empty())).as_map_mut();
            let tag = match tag {
                Some(tag) => tag,
                None => return
            };

            if tag.get_string("Potion").is_none() {
                tag.set("Potion", T::Object::create_string("minecraft:water".to_owned()));
            }
        }
    }

    types.item_stack.borrow_mut().add_converter_for_id("minecraft:potion", VERSION, PotionWaterUpdater::<T>::new());
    types.item_stack.borrow_mut().add_converter_for_id("minecraft:splash_potion", VERSION, PotionWaterUpdater::<T>::new());
    types.item_stack.borrow_mut().add_converter_for_id("minecraft:lingering_potion", VERSION, PotionWaterUpdater::<T>::new());
    types.item_stack.borrow_mut().add_converter_for_id("minecraft:tipped_arrow", VERSION, PotionWaterUpdater::<T>::new());
}
