use rust_dataconverter_engine::{convert_map_in_map, data_converter_func, data_walker, DataWalkerMapListPaths, ListType, MapType, ObjectType, Types};
use crate::MinecraftTypesMut;

const VERSION: u32 = 135;

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    // In this update they changed the "Riding" value to be "Passengers", which is now a list. So it added
    // support for multiple entities riding. Of course, Riding and Passenger are opposites - so it also will
    // switch the data layout to be from highest rider to lowest rider, in terms of depth.
    types.entity.borrow_mut().add_structure_converter(VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        while let Some(riding) = data.remove("Riding").and_then(|o| o.into_map()) {
            let mut passenger = riding;
            std::mem::swap(&mut passenger, data);
            let mut passengers = T::List::create_empty();
            passengers.add(T::Object::create_map(passenger));
            data.set("Passengers", T::Object::create_list(passengers));
        }
    }));

    types.player.borrow_mut().add_structure_walker(VERSION, DataWalkerMapListPaths::new_multi(types.item_stack, vec!["Inventory".to_owned(), "EnderItems".to_owned()]));
    let entity_type = types.entity;
    types.player.borrow_mut().add_structure_walker(VERSION, data_walker::<T, _>(move |data: &mut T::Map, from_version, to_version| {
        if let Some(root_vehicle) = data.get_map_mut("RootVehicle") {
            convert_map_in_map::<_, T>(entity_type, root_vehicle, "Entity", from_version, to_version);
        }
    }));

    types.entity.borrow_mut().add_structure_walker(VERSION, DataWalkerMapListPaths::new(types.entity, "Passengers"));
}
