use crate::helpers::mc_namespace_map::McNamespaceMap;
use crate::helpers::rename::rename_item;
use crate::types::MinecraftTypesMut;
use std::sync::OnceLock;

const VERSION: u32 = 3447;

static RENAMES: OnceLock<McNamespaceMap<&str>> = OnceLock::new();

fn renames() -> &'static McNamespaceMap<'static, &'static str> {
    RENAMES.get_or_init(|| {
        let mut map = McNamespaceMap::new();
        map.insert_mc("angler_pottery_shard", "minecraft:angler_pottery_sherd");
        map.insert_mc("archer_pottery_shard", "minecraft:archer_pottery_sherd");
        map.insert_mc("arms_up_pottery_shard", "minecraft:arms_up_pottery_sherd");
        map.insert_mc("blade_pottery_shard", "minecraft:blade_pottery_sherd");
        map.insert_mc("brewer_pottery_shard", "minecraft:brewer_pottery_sherd");
        map.insert_mc("burn_pottery_shard", "minecraft:burn_pottery_sherd");
        map.insert_mc("danger_pottery_shard", "minecraft:danger_pottery_sherd");
        map.insert_mc("explorer_pottery_shard", "minecraft:explorer_pottery_sherd");
        map.insert_mc("friend_pottery_shard", "minecraft:friend_pottery_sherd");
        map.insert_mc("heart_pottery_shard", "minecraft:heart_pottery_sherd");
        map.insert_mc(
            "heartbreak_pottery_shard",
            "minecraft:heartbreak_pottery_sherd",
        );
        map.insert_mc("howl_pottery_shard", "minecraft:howl_pottery_sherd");
        map.insert_mc("miner_pottery_shard", "minecraft:miner_pottery_sherd");
        map.insert_mc("mourner_pottery_shard", "minecraft:mourner_pottery_sherd");
        map.insert_mc("plenty_pottery_shard", "minecraft:plenty_pottery_sherd");
        map.insert_mc("prize_pottery_shard", "minecraft:prize_pottery_sherd");
        map.insert_mc("sheaf_pottery_shard", "minecraft:sheaf_pottery_sherd");
        map.insert_mc("shelter_pottery_shard", "minecraft:shelter_pottery_sherd");
        map.insert_mc("skull_pottery_shard", "minecraft:skull_pottery_sherd");
        map.insert_mc("snort_pottery_shard", "minecraft:snort_pottery_sherd");
        map
    })
}

pub(crate) fn register(types: &MinecraftTypesMut) {
    rename_item(types, VERSION, |name| {
        renames()
            .get(name)
            .copied()
            .map(|new_name| new_name.to_owned())
    });
}
