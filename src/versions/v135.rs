use crate::MinecraftTypes;
use rust_dataconverter_engine::{
    convert_map_in_map, data_walker, map_data_converter_func, DataWalkerMapListPaths,
};
use valence_nbt::{List, Value};

const VERSION: u32 = 135;

pub(crate) fn register<'a>(types: &'a MinecraftTypes<'a>) {
    // In this update they changed the "Riding" value to be "Passengers", which is now a list. So it added
    // support for multiple entities riding. Of course, Riding and Passenger are opposites - so it also will
    // switch the data layout to be from highest rider to lowest rider, in terms of depth.
    types.entity.borrow_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            while let Some(Value::Compound(riding)) = data.remove("Riding") {
                let mut passenger = riding;
                std::mem::swap(&mut passenger, data);
                let passengers = List::from(vec![passenger]);
                data.insert("Passengers", passengers);
            }
        }),
    );

    types.player.borrow_mut().add_structure_walker(
        VERSION,
        DataWalkerMapListPaths::new_multi(
            &types.item_stack,
            vec!["Inventory".to_owned(), "EnderItems".to_owned()],
        ),
    );
    let entity_type = &types.entity;
    types.player.borrow_mut().add_structure_walker(
        VERSION,
        data_walker(move |data, from_version, to_version| {
            if let Some(Value::Compound(root_vehicle)) = data.get_mut("RootVehicle") {
                convert_map_in_map(
                    entity_type,
                    root_vehicle,
                    "Entity",
                    from_version,
                    to_version,
                );
            }
        }),
    );

    types.entity.borrow_mut().add_structure_walker(
        VERSION,
        DataWalkerMapListPaths::new(&types.entity, "Passengers"),
    );
}
