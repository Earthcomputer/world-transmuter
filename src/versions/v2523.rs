use std::lazy::SyncOnceCell;
use rust_dataconverter_engine::{data_converter_func, DataVersion, ListType, MapType, ObjectType, Types};
use crate::MinecraftTypesMut;

const VERSION: u32 = 2523;

static RENAMES: SyncOnceCell<rust_dataconverter_engine::Map<&'static str, &'static str>> = SyncOnceCell::new();

fn renames() -> &'static rust_dataconverter_engine::Map<&'static str, &'static str> {
    RENAMES.get_or_init(|| {
        let mut map = rust_dataconverter_engine::Map::new();
        map.insert("generic.maxHealth", "generic.max_health");
        map.insert("Max Health", "generic.max_health");
        map.insert("zombie.spawnReinforcements", "zombie.spawn_reinforcements");
        map.insert("Spawn Reinforcements Chance", "zombie.spawn_reinforcements");
        map.insert("horse.jumpStrength", "horse.jump_strength");
        map.insert("Jump Strength", "horse.jump_strength");
        map.insert("generic.followRange", "generic.follow_range");
        map.insert("Follow Range", "generic.follow_range");
        map.insert("generic.knockbackResistance", "generic.knockback_resistance");
        map.insert("Knockback Resistance", "generic.knockback_resistance");
        map.insert("generic.movementSpeed", "generic.movement_speed");
        map.insert("Movement Speed", "generic.movement_speed");
        map.insert("generic.flyingSpeed", "generic.flying_speed");
        map.insert("Flying Speed", "generic.flying_speed");
        map.insert("generic.attackDamage", "generic.attack_damage");
        map.insert("generic.attackKnockback", "generic.attack_knockback");
        map.insert("generic.attackSpeed", "generic.attack_speed");
        map.insert("generic.armorToughness", "generic.armor_toughness");
        map
    })
}

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    types.entity.borrow_mut().add_structure_converter(VERSION, data_converter_func::<T::Map, _>(entity_converter::<T>));
    types.player.borrow_mut().add_structure_converter(VERSION, data_converter_func::<T::Map, _>(entity_converter::<T>));

    types.item_stack.borrow_mut().add_structure_converter(VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        if let Some(attributes) = data.get_list_mut("AttributeModifiers") {
            for attribute in attributes.iter_mut() {
                if let Some(attribute) = attribute.as_map_mut() {
                    update_name::<T>(attribute, "AttributeName");
                }
            }
        }
    }));
}

fn entity_converter<T: Types + ?Sized>(data: &mut T::Map, _from_version: DataVersion, _to_version: DataVersion) {
    if let Some(attributes) = data.get_list_mut("Attributes") {
        for attribute in attributes.iter_mut() {
            if let Some(attribute) = attribute.as_map_mut() {
                update_name::<T>(attribute, "Name");
            }
        }
    }
}

fn update_name<T: Types + ?Sized>(data: &mut T::Map, path: &str) {
    if let Some(new_name) = data.get_string(path).and_then(|name| renames().get(name)).copied() {
        data.set(path, T::Object::create_string(new_name.to_owned()));
    }
}
