use rust_dataconverter_engine::{convert_map_in_map, data_converter_func, data_walker, ListType, MapType, ObjectType, Types};
use crate::MinecraftTypesMut;

const VERSION: u32 = 2831;

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    let entity_type = types.entity;
    types.untagged_spawner.borrow_mut().add_structure_walker(VERSION, data_walker::<T, _>(move |data: &mut T::Map, from_version, to_version| {
        if let Some(spawn_potentials) = data.get_list_mut("SpawnPotentials") {
            for spawn_potential in spawn_potentials.iter_mut() {
                if let Some(spawn_data) = spawn_potential.as_map_mut().and_then(|o| o.get_map_mut("data")) {
                    convert_map_in_map::<_, T>(entity_type, spawn_data, "entity", from_version, to_version);
                }
            }
        }

        if let Some(spawn_data) = data.get_map_mut("SpawnData") {
            convert_map_in_map::<_, T>(entity_type, spawn_data, "entity", from_version, to_version);
        }
    }));

    types.untagged_spawner.borrow_mut().add_structure_converter(VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        if data.get_map("SpawnData").is_some() {
            let spawn_data = data.remove("SpawnData").unwrap().into_map().unwrap();
            let mut wrapped = T::Map::create_empty();
            wrapped.set("entity", T::Object::create_map(spawn_data));
            data.set("SpawnData", T::Object::create_map(wrapped));
        }

        if let Some(spawn_potentials) = data.get_list_mut("SpawnPotentials") {
            for spawn_potential in spawn_potentials.iter_mut() {
                if let Some(spawn_potential) = spawn_potential.as_map_mut() {
                    // new format of weighted list (SpawnPotentials):
                    // root.data -> data
                    // root.weight -> weight

                    let entity = spawn_potential.remove("Entity").and_then(|o| o.into_map());
                    let weight = spawn_potential.remove("Weight").and_then(|o| o.as_i64()).unwrap_or(1) as i32;
                    spawn_potential.set("weight", T::Object::create_int(weight));
                    let mut data = T::Map::create_empty();
                    if let Some(entity) = entity {
                        data.set("entity", T::Object::create_map(entity));
                    }
                    spawn_potential.set("data", T::Object::create_map(data));
                }
            }
        }
    }));
}
