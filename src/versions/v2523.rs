use crate::MinecraftTypesMut;
use rust_dataconverter_engine::{map_data_converter_func, DataVersion};
use std::collections::BTreeMap;
use std::sync::OnceLock;
use valence_nbt::{Compound, List, Value};

const VERSION: u32 = 2523;

static RENAMES: OnceLock<BTreeMap<&'static str, &'static str>> = OnceLock::new();

fn renames() -> &'static BTreeMap<&'static str, &'static str> {
    RENAMES.get_or_init(|| {
        let mut map = BTreeMap::new();
        map.insert("generic.maxHealth", "generic.max_health");
        map.insert("Max Health", "generic.max_health");
        map.insert("zombie.spawnReinforcements", "zombie.spawn_reinforcements");
        map.insert("Spawn Reinforcements Chance", "zombie.spawn_reinforcements");
        map.insert("horse.jumpStrength", "horse.jump_strength");
        map.insert("Jump Strength", "horse.jump_strength");
        map.insert("generic.followRange", "generic.follow_range");
        map.insert("Follow Range", "generic.follow_range");
        map.insert(
            "generic.knockbackResistance",
            "generic.knockback_resistance",
        );
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

pub(crate) fn register(types: &MinecraftTypesMut) {
    types
        .entity
        .borrow_mut()
        .add_structure_converter(VERSION, map_data_converter_func(entity_converter));
    types
        .player
        .borrow_mut()
        .add_structure_converter(VERSION, map_data_converter_func(entity_converter));

    types.item_stack.borrow_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            if let Some(Value::List(List::Compound(attributes))) =
                data.get_mut("AttributeModifiers")
            {
                for attribute in attributes {
                    update_name(attribute, "AttributeName");
                }
            }
        }),
    );
}

fn entity_converter(data: &mut Compound, _from_version: DataVersion, _to_version: DataVersion) {
    if let Some(Value::List(List::Compound(attributes))) = data.get_mut("Attributes") {
        for attribute in attributes {
            update_name(attribute, "Name");
        }
    }
}

fn update_name(data: &mut Compound, path: &str) {
    let Some(Value::String(name)) = data.get_mut(path) else {
        return;
    };
    if let Some(new_name) = renames().get(&name[..]).copied() {
        *name = new_name.to_owned();
    }
}
