use crate::helpers::rename::rename_attribute;
use crate::static_string_map;

const VERSION: u32 = 2523;

static_string_map! {
    RENAMES, renames, {
        "generic.maxHealth" => "minecraft:generic.max_health",
        "Max Health" => "minecraft:generic.max_health",
        "zombie.spawnReinforcements" => "minecraft:zombie.spawn_reinforcements",
        "Spawn Reinforcements Chance" => "minecraft:zombie.spawn_reinforcements",
        "horse.jumpStrength" => "minecraft:horse.jump_strength",
        "Jump Strength" => "minecraft:horse.jump_strength",
        "generic.followRange" => "minecraft:generic.follow_range",
        "Follow Range" => "minecraft:generic.follow_range",
        "generic.knockbackResistance" => "minecraft:generic.knockback_resistance",
        "Knockback Resistance" => "minecraft:generic.knockback_resistance",
        "generic.movementSpeed" => "minecraft:generic.movement_speed",
        "Movement Speed" => "minecraft:generic.movement_speed",
        "generic.flyingSpeed" => "minecraft:generic.flying_speed",
        "Flying Speed" => "minecraft:generic.flying_speed",
        "generic.attackDamage" => "minecraft:generic.attack_damage",
        "generic.attackKnockback" => "minecraft:generic.attack_knockback",
        "generic.attackSpeed" => "minecraft:generic.attack_speed",
        "generic.armorToughness" => "minecraft:generic.armor_toughness",
    }
}

pub(crate) fn register() {
    rename_attribute(VERSION, |name| {
        renames().get(name).copied().map(|o| o.to_owned())
    })
}
