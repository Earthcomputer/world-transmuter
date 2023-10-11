use crate::{static_string_map, types};
use world_transmuter_engine::{map_data_converter_func, DataVersion, JCompound, JList, JValue};

const VERSION: u32 = 2523;

static_string_map! {
    RENAMES, renames, {
        "generic.maxHealth" => "generic.max_health",
        "Max Health" => "generic.max_health",
        "zombie.spawnReinforcements" => "zombie.spawn_reinforcements",
        "Spawn Reinforcements Chance" => "zombie.spawn_reinforcements",
        "horse.jumpStrength" => "horse.jump_strength",
        "Jump Strength" => "horse.jump_strength",
        "generic.followRange" => "generic.follow_range",
        "Follow Range" => "generic.follow_range",
        "generic.knockbackResistance" => "generic.knockback_resistance",
        "Knockback Resistance" => "generic.knockback_resistance",
        "generic.movementSpeed" => "generic.movement_speed",
        "Movement Speed" => "generic.movement_speed",
        "generic.flyingSpeed" => "generic.flying_speed",
        "Flying Speed" => "generic.flying_speed",
        "generic.attackDamage" => "generic.attack_damage",
        "generic.attackKnockback" => "generic.attack_knockback",
        "generic.attackSpeed" => "generic.attack_speed",
        "generic.armorToughness" => "generic.armor_toughness",
    }
}

pub(crate) fn register() {
    types::entity_mut().add_structure_converter(VERSION, map_data_converter_func(entity_converter));
    types::player_mut().add_structure_converter(VERSION, map_data_converter_func(entity_converter));

    types::item_stack_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            if let Some(JValue::List(JList::Compound(attributes))) =
                data.get_mut("AttributeModifiers")
            {
                for attribute in attributes {
                    update_name(attribute, "AttributeName");
                }
            }
        }),
    );
}

fn entity_converter(data: &mut JCompound, _from_version: DataVersion, _to_version: DataVersion) {
    if let Some(JValue::List(JList::Compound(attributes))) = data.get_mut("Attributes") {
        for attribute in attributes {
            update_name(attribute, "Name");
        }
    }
}

fn update_name(data: &mut JCompound, path: &str) {
    let Some(JValue::String(name)) = data.get_mut(path) else {
        return;
    };
    if let Some(new_name) = renames().get(&name[..]).copied() {
        *name = new_name.to_owned();
    }
}
