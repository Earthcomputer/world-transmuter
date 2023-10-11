use crate::helpers::rename::rename_item;
use crate::static_string_mc_map;

const VERSION: u32 = 3447;

static_string_mc_map! {
    RENAMES, renames, {
        "angler_pottery_shard" => "minecraft:angler_pottery_sherd",
        "archer_pottery_shard" => "minecraft:archer_pottery_sherd",
        "arms_up_pottery_shard" => "minecraft:arms_up_pottery_sherd",
        "blade_pottery_shard" => "minecraft:blade_pottery_sherd",
        "brewer_pottery_shard" => "minecraft:brewer_pottery_sherd",
        "burn_pottery_shard" => "minecraft:burn_pottery_sherd",
        "danger_pottery_shard" => "minecraft:danger_pottery_sherd",
        "explorer_pottery_shard" => "minecraft:explorer_pottery_sherd",
        "friend_pottery_shard" => "minecraft:friend_pottery_sherd",
        "heart_pottery_shard" => "minecraft:heart_pottery_sherd",
        "heartbreak_pottery_shard" => "minecraft:heartbreak_pottery_sherd",
        "howl_pottery_shard" => "minecraft:howl_pottery_sherd",
        "miner_pottery_shard" => "minecraft:miner_pottery_sherd",
        "mourner_pottery_shard" => "minecraft:mourner_pottery_sherd",
        "plenty_pottery_shard" => "minecraft:plenty_pottery_sherd",
        "prize_pottery_shard" => "minecraft:prize_pottery_sherd",
        "sheaf_pottery_shard" => "minecraft:sheaf_pottery_sherd",
        "shelter_pottery_shard" => "minecraft:shelter_pottery_sherd",
        "skull_pottery_shard" => "minecraft:skull_pottery_sherd",
        "snort_pottery_shard" => "minecraft:snort_pottery_sherd",
    }
}

pub(crate) fn register() {
    rename_item(VERSION, |name| {
        renames()
            .get(name)
            .copied()
            .map(|new_name| new_name.to_owned())
    });
}
