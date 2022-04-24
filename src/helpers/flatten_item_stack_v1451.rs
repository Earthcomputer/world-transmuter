use std::lazy::SyncOnceCell;
use std::marker::PhantomData;
use rust_dataconverter_engine::{DataConverterFunc, DataVersion, MapType, ObjectType, Types};

#[derive(Default)]
struct ItemStackFlattenData {
    // Map of ("id", damage) -> "flattened id"
    flatten_map: rust_dataconverter_engine::Map<(String, u8), String>,
    // maps out ids requiring flattening
    ids_requiring_flattening: rust_dataconverter_engine::Map<String, ()>,
    // Damage tag is moved from the ItemStack base tag to the ItemStack tag, and we only want to migrate that
    // for items that actually require it for damage purposes (Remember, old damage was used to differentiate item types)
    // It should be noted that this ID set should not be included in the flattening map, because damage for these items
    // is actual damage and not a subtype specifier
    items_with_damage: rust_dataconverter_engine::Map<String, ()>,
}

static FLATTEN_DATA: SyncOnceCell<ItemStackFlattenData> = SyncOnceCell::new();

fn flatten_data() -> &'static ItemStackFlattenData {
    FLATTEN_DATA.get_or_init(|| {
        let mut flatten_data = ItemStackFlattenData::default();

        let mut flatten = |id: &str, data: u8, new_id: &str| {
            debug_assert!(data == 0 || flatten_data.flatten_map.contains_key(&(id.to_owned(), 0)));
            flatten_data.flatten_map.insert((id.to_owned(), data), new_id.to_owned());
            flatten_data.ids_requiring_flattening.insert(id.to_owned(), ());
        };

        flatten("minecraft:stone", 0, "minecraft:stone");
        flatten("minecraft:stone", 1, "minecraft:granite");
        flatten("minecraft:stone", 2, "minecraft:polished_granite");
        flatten("minecraft:stone", 3, "minecraft:diorite");
        flatten("minecraft:stone", 4, "minecraft:polished_diorite");
        flatten("minecraft:stone", 5, "minecraft:andesite");
        flatten("minecraft:stone", 6, "minecraft:polished_andesite");
        flatten("minecraft:dirt", 0, "minecraft:dirt");
        flatten("minecraft:dirt", 1, "minecraft:coarse_dirt");
        flatten("minecraft:dirt", 2, "minecraft:podzol");
        flatten("minecraft:leaves", 0, "minecraft:oak_leaves");
        flatten("minecraft:leaves", 1, "minecraft:spruce_leaves");
        flatten("minecraft:leaves", 2, "minecraft:birch_leaves");
        flatten("minecraft:leaves", 3, "minecraft:jungle_leaves");
        flatten("minecraft:leaves2", 0, "minecraft:acacia_leaves");
        flatten("minecraft:leaves2", 1, "minecraft:dark_oak_leaves");
        flatten("minecraft:log", 0, "minecraft:oak_log");
        flatten("minecraft:log", 1, "minecraft:spruce_log");
        flatten("minecraft:log", 2, "minecraft:birch_log");
        flatten("minecraft:log", 3, "minecraft:jungle_log");
        flatten("minecraft:log2", 0, "minecraft:acacia_log");
        flatten("minecraft:log2", 1, "minecraft:dark_oak_log");
        flatten("minecraft:sapling", 0, "minecraft:oak_sapling");
        flatten("minecraft:sapling", 1, "minecraft:spruce_sapling");
        flatten("minecraft:sapling", 2, "minecraft:birch_sapling");
        flatten("minecraft:sapling", 3, "minecraft:jungle_sapling");
        flatten("minecraft:sapling", 4, "minecraft:acacia_sapling");
        flatten("minecraft:sapling", 5, "minecraft:dark_oak_sapling");
        flatten("minecraft:planks", 0, "minecraft:oak_planks");
        flatten("minecraft:planks", 1, "minecraft:spruce_planks");
        flatten("minecraft:planks", 2, "minecraft:birch_planks");
        flatten("minecraft:planks", 3, "minecraft:jungle_planks");
        flatten("minecraft:planks", 4, "minecraft:acacia_planks");
        flatten("minecraft:planks", 5, "minecraft:dark_oak_planks");
        flatten("minecraft:sand", 0, "minecraft:sand");
        flatten("minecraft:sand", 1, "minecraft:red_sand");
        flatten("minecraft:quartz_block", 0, "minecraft:quartz_block");
        flatten("minecraft:quartz_block", 1, "minecraft:chiseled_quartz_block");
        flatten("minecraft:quartz_block", 2, "minecraft:quartz_pillar");
        flatten("minecraft:anvil", 0, "minecraft:anvil");
        flatten("minecraft:anvil", 1, "minecraft:chipped_anvil");
        flatten("minecraft:anvil", 2, "minecraft:damaged_anvil");
        flatten("minecraft:wool", 0, "minecraft:white_wool");
        flatten("minecraft:wool", 1, "minecraft:orange_wool");
        flatten("minecraft:wool", 2, "minecraft:magenta_wool");
        flatten("minecraft:wool", 3, "minecraft:light_blue_wool");
        flatten("minecraft:wool", 4, "minecraft:yellow_wool");
        flatten("minecraft:wool", 5, "minecraft:lime_wool");
        flatten("minecraft:wool", 6, "minecraft:pink_wool");
        flatten("minecraft:wool", 7, "minecraft:gray_wool");
        flatten("minecraft:wool", 8, "minecraft:light_gray_wool");
        flatten("minecraft:wool", 9, "minecraft:cyan_wool");
        flatten("minecraft:wool", 10, "minecraft:purple_wool");
        flatten("minecraft:wool", 11, "minecraft:blue_wool");
        flatten("minecraft:wool", 12, "minecraft:brown_wool");
        flatten("minecraft:wool", 13, "minecraft:green_wool");
        flatten("minecraft:wool", 14, "minecraft:red_wool");
        flatten("minecraft:wool", 15, "minecraft:black_wool");
        flatten("minecraft:carpet", 0, "minecraft:white_carpet");
        flatten("minecraft:carpet", 1, "minecraft:orange_carpet");
        flatten("minecraft:carpet", 2, "minecraft:magenta_carpet");
        flatten("minecraft:carpet", 3, "minecraft:light_blue_carpet");
        flatten("minecraft:carpet", 4, "minecraft:yellow_carpet");
        flatten("minecraft:carpet", 5, "minecraft:lime_carpet");
        flatten("minecraft:carpet", 6, "minecraft:pink_carpet");
        flatten("minecraft:carpet", 7, "minecraft:gray_carpet");
        flatten("minecraft:carpet", 8, "minecraft:light_gray_carpet");
        flatten("minecraft:carpet", 9, "minecraft:cyan_carpet");
        flatten("minecraft:carpet", 10, "minecraft:purple_carpet");
        flatten("minecraft:carpet", 11, "minecraft:blue_carpet");
        flatten("minecraft:carpet", 12, "minecraft:brown_carpet");
        flatten("minecraft:carpet", 13, "minecraft:green_carpet");
        flatten("minecraft:carpet", 14, "minecraft:red_carpet");
        flatten("minecraft:carpet", 15, "minecraft:black_carpet");
        flatten("minecraft:hardened_clay", 0, "minecraft:terracotta");
        flatten("minecraft:stained_hardened_clay", 0, "minecraft:white_terracotta");
        flatten("minecraft:stained_hardened_clay", 1, "minecraft:orange_terracotta");
        flatten("minecraft:stained_hardened_clay", 2, "minecraft:magenta_terracotta");
        flatten("minecraft:stained_hardened_clay", 3, "minecraft:light_blue_terracotta");
        flatten("minecraft:stained_hardened_clay", 4, "minecraft:yellow_terracotta");
        flatten("minecraft:stained_hardened_clay", 5, "minecraft:lime_terracotta");
        flatten("minecraft:stained_hardened_clay", 6, "minecraft:pink_terracotta");
        flatten("minecraft:stained_hardened_clay", 7, "minecraft:gray_terracotta");
        flatten("minecraft:stained_hardened_clay", 8, "minecraft:light_gray_terracotta");
        flatten("minecraft:stained_hardened_clay", 9, "minecraft:cyan_terracotta");
        flatten("minecraft:stained_hardened_clay", 10, "minecraft:purple_terracotta");
        flatten("minecraft:stained_hardened_clay", 11, "minecraft:blue_terracotta");
        flatten("minecraft:stained_hardened_clay", 12, "minecraft:brown_terracotta");
        flatten("minecraft:stained_hardened_clay", 13, "minecraft:green_terracotta");
        flatten("minecraft:stained_hardened_clay", 14, "minecraft:red_terracotta");
        flatten("minecraft:stained_hardened_clay", 15, "minecraft:black_terracotta");
        flatten("minecraft:silver_glazed_terracotta", 0, "minecraft:light_gray_glazed_terracotta");
        flatten("minecraft:stained_glass", 0, "minecraft:white_stained_glass");
        flatten("minecraft:stained_glass", 1, "minecraft:orange_stained_glass");
        flatten("minecraft:stained_glass", 2, "minecraft:magenta_stained_glass");
        flatten("minecraft:stained_glass", 3, "minecraft:light_blue_stained_glass");
        flatten("minecraft:stained_glass", 4, "minecraft:yellow_stained_glass");
        flatten("minecraft:stained_glass", 5, "minecraft:lime_stained_glass");
        flatten("minecraft:stained_glass", 6, "minecraft:pink_stained_glass");
        flatten("minecraft:stained_glass", 7, "minecraft:gray_stained_glass");
        flatten("minecraft:stained_glass", 8, "minecraft:light_gray_stained_glass");
        flatten("minecraft:stained_glass", 9, "minecraft:cyan_stained_glass");
        flatten("minecraft:stained_glass", 10, "minecraft:purple_stained_glass");
        flatten("minecraft:stained_glass", 11, "minecraft:blue_stained_glass");
        flatten("minecraft:stained_glass", 12, "minecraft:brown_stained_glass");
        flatten("minecraft:stained_glass", 13, "minecraft:green_stained_glass");
        flatten("minecraft:stained_glass", 14, "minecraft:red_stained_glass");
        flatten("minecraft:stained_glass", 15, "minecraft:black_stained_glass");
        flatten("minecraft:stained_glass_pane", 0, "minecraft:white_stained_glass_pane");
        flatten("minecraft:stained_glass_pane", 1, "minecraft:orange_stained_glass_pane");
        flatten("minecraft:stained_glass_pane", 2, "minecraft:magenta_stained_glass_pane");
        flatten("minecraft:stained_glass_pane", 3, "minecraft:light_blue_stained_glass_pane");
        flatten("minecraft:stained_glass_pane", 4, "minecraft:yellow_stained_glass_pane");
        flatten("minecraft:stained_glass_pane", 5, "minecraft:lime_stained_glass_pane");
        flatten("minecraft:stained_glass_pane", 6, "minecraft:pink_stained_glass_pane");
        flatten("minecraft:stained_glass_pane", 7, "minecraft:gray_stained_glass_pane");
        flatten("minecraft:stained_glass_pane", 8, "minecraft:light_gray_stained_glass_pane");
        flatten("minecraft:stained_glass_pane", 9, "minecraft:cyan_stained_glass_pane");
        flatten("minecraft:stained_glass_pane", 10, "minecraft:purple_stained_glass_pane");
        flatten("minecraft:stained_glass_pane", 11, "minecraft:blue_stained_glass_pane");
        flatten("minecraft:stained_glass_pane", 12, "minecraft:brown_stained_glass_pane");
        flatten("minecraft:stained_glass_pane", 13, "minecraft:green_stained_glass_pane");
        flatten("minecraft:stained_glass_pane", 14, "minecraft:red_stained_glass_pane");
        flatten("minecraft:stained_glass_pane", 15, "minecraft:black_stained_glass_pane");
        flatten("minecraft:prismarine", 0, "minecraft:prismarine");
        flatten("minecraft:prismarine", 1, "minecraft:prismarine_bricks");
        flatten("minecraft:prismarine", 2, "minecraft:dark_prismarine");
        flatten("minecraft:concrete", 0, "minecraft:white_concrete");
        flatten("minecraft:concrete", 1, "minecraft:orange_concrete");
        flatten("minecraft:concrete", 2, "minecraft:magenta_concrete");
        flatten("minecraft:concrete", 3, "minecraft:light_blue_concrete");
        flatten("minecraft:concrete", 4, "minecraft:yellow_concrete");
        flatten("minecraft:concrete", 5, "minecraft:lime_concrete");
        flatten("minecraft:concrete", 6, "minecraft:pink_concrete");
        flatten("minecraft:concrete", 7, "minecraft:gray_concrete");
        flatten("minecraft:concrete", 8, "minecraft:light_gray_concrete");
        flatten("minecraft:concrete", 9, "minecraft:cyan_concrete");
        flatten("minecraft:concrete", 10, "minecraft:purple_concrete");
        flatten("minecraft:concrete", 11, "minecraft:blue_concrete");
        flatten("minecraft:concrete", 12, "minecraft:brown_concrete");
        flatten("minecraft:concrete", 13, "minecraft:green_concrete");
        flatten("minecraft:concrete", 14, "minecraft:red_concrete");
        flatten("minecraft:concrete", 15, "minecraft:black_concrete");
        flatten("minecraft:concrete_powder", 0, "minecraft:white_concrete_powder");
        flatten("minecraft:concrete_powder", 1, "minecraft:orange_concrete_powder");
        flatten("minecraft:concrete_powder", 2, "minecraft:magenta_concrete_powder");
        flatten("minecraft:concrete_powder", 3, "minecraft:light_blue_concrete_powder");
        flatten("minecraft:concrete_powder", 4, "minecraft:yellow_concrete_powder");
        flatten("minecraft:concrete_powder", 5, "minecraft:lime_concrete_powder");
        flatten("minecraft:concrete_powder", 6, "minecraft:pink_concrete_powder");
        flatten("minecraft:concrete_powder", 7, "minecraft:gray_concrete_powder");
        flatten("minecraft:concrete_powder", 8, "minecraft:light_gray_concrete_powder");
        flatten("minecraft:concrete_powder", 9, "minecraft:cyan_concrete_powder");
        flatten("minecraft:concrete_powder", 10, "minecraft:purple_concrete_powder");
        flatten("minecraft:concrete_powder", 11, "minecraft:blue_concrete_powder");
        flatten("minecraft:concrete_powder", 12, "minecraft:brown_concrete_powder");
        flatten("minecraft:concrete_powder", 13, "minecraft:green_concrete_powder");
        flatten("minecraft:concrete_powder", 14, "minecraft:red_concrete_powder");
        flatten("minecraft:concrete_powder", 15, "minecraft:black_concrete_powder");
        flatten("minecraft:cobblestone_wall", 0, "minecraft:cobblestone_wall");
        flatten("minecraft:cobblestone_wall", 1, "minecraft:mossy_cobblestone_wall");
        flatten("minecraft:sandstone", 0, "minecraft:sandstone");
        flatten("minecraft:sandstone", 1, "minecraft:chiseled_sandstone");
        flatten("minecraft:sandstone", 2, "minecraft:cut_sandstone");
        flatten("minecraft:red_sandstone", 0, "minecraft:red_sandstone");
        flatten("minecraft:red_sandstone", 1, "minecraft:chiseled_red_sandstone");
        flatten("minecraft:red_sandstone", 2, "minecraft:cut_red_sandstone");
        flatten("minecraft:stonebrick", 0, "minecraft:stone_bricks");
        flatten("minecraft:stonebrick", 1, "minecraft:mossy_stone_bricks");
        flatten("minecraft:stonebrick", 2, "minecraft:cracked_stone_bricks");
        flatten("minecraft:stonebrick", 3, "minecraft:chiseled_stone_bricks");
        flatten("minecraft:monster_egg", 0, "minecraft:infested_stone");
        flatten("minecraft:monster_egg", 1, "minecraft:infested_cobblestone");
        flatten("minecraft:monster_egg", 2, "minecraft:infested_stone_bricks");
        flatten("minecraft:monster_egg", 3, "minecraft:infested_mossy_stone_bricks");
        flatten("minecraft:monster_egg", 4, "minecraft:infested_cracked_stone_bricks");
        flatten("minecraft:monster_egg", 5, "minecraft:infested_chiseled_stone_bricks");
        flatten("minecraft:yellow_flower", 0, "minecraft:dandelion");
        flatten("minecraft:red_flower", 0, "minecraft:poppy");
        flatten("minecraft:red_flower", 1, "minecraft:blue_orchid");
        flatten("minecraft:red_flower", 2, "minecraft:allium");
        flatten("minecraft:red_flower", 3, "minecraft:azure_bluet");
        flatten("minecraft:red_flower", 4, "minecraft:red_tulip");
        flatten("minecraft:red_flower", 5, "minecraft:orange_tulip");
        flatten("minecraft:red_flower", 6, "minecraft:white_tulip");
        flatten("minecraft:red_flower", 7, "minecraft:pink_tulip");
        flatten("minecraft:red_flower", 8, "minecraft:oxeye_daisy");
        flatten("minecraft:double_plant", 0, "minecraft:sunflower");
        flatten("minecraft:double_plant", 1, "minecraft:lilac");
        flatten("minecraft:double_plant", 2, "minecraft:tall_grass");
        flatten("minecraft:double_plant", 3, "minecraft:large_fern");
        flatten("minecraft:double_plant", 4, "minecraft:rose_bush");
        flatten("minecraft:double_plant", 5, "minecraft:peony");
        flatten("minecraft:deadbush", 0, "minecraft:dead_bush");
        flatten("minecraft:tallgrass", 0, "minecraft:dead_bush");
        flatten("minecraft:tallgrass", 1, "minecraft:grass");
        flatten("minecraft:tallgrass", 2, "minecraft:fern");
        flatten("minecraft:sponge", 0, "minecraft:sponge");
        flatten("minecraft:sponge", 1, "minecraft:wet_sponge");
        flatten("minecraft:purpur_slab", 0, "minecraft:purpur_slab");
        flatten("minecraft:stone_slab", 0, "minecraft:stone_slab");
        flatten("minecraft:stone_slab", 1, "minecraft:sandstone_slab");
        flatten("minecraft:stone_slab", 2, "minecraft:petrified_oak_slab");
        flatten("minecraft:stone_slab", 3, "minecraft:cobblestone_slab");
        flatten("minecraft:stone_slab", 4, "minecraft:brick_slab");
        flatten("minecraft:stone_slab", 5, "minecraft:stone_brick_slab");
        flatten("minecraft:stone_slab", 6, "minecraft:nether_brick_slab");
        flatten("minecraft:stone_slab", 7, "minecraft:quartz_slab");
        flatten("minecraft:stone_slab2", 0, "minecraft:red_sandstone_slab");
        flatten("minecraft:wooden_slab", 0, "minecraft:oak_slab");
        flatten("minecraft:wooden_slab", 1, "minecraft:spruce_slab");
        flatten("minecraft:wooden_slab", 2, "minecraft:birch_slab");
        flatten("minecraft:wooden_slab", 3, "minecraft:jungle_slab");
        flatten("minecraft:wooden_slab", 4, "minecraft:acacia_slab");
        flatten("minecraft:wooden_slab", 5, "minecraft:dark_oak_slab");
        flatten("minecraft:coal", 0, "minecraft:coal");
        flatten("minecraft:coal", 1, "minecraft:charcoal");
        flatten("minecraft:fish", 0, "minecraft:cod");
        flatten("minecraft:fish", 1, "minecraft:salmon");
        flatten("minecraft:fish", 2, "minecraft:clownfish");
        flatten("minecraft:fish", 3, "minecraft:pufferfish");
        flatten("minecraft:cooked_fish", 0, "minecraft:cooked_cod");
        flatten("minecraft:cooked_fish", 1, "minecraft:cooked_salmon");
        flatten("minecraft:skull", 0, "minecraft:skeleton_skull");
        flatten("minecraft:skull", 1, "minecraft:wither_skeleton_skull");
        flatten("minecraft:skull", 2, "minecraft:zombie_head");
        flatten("minecraft:skull", 3, "minecraft:player_head");
        flatten("minecraft:skull", 4, "minecraft:creeper_head");
        flatten("minecraft:skull", 5, "minecraft:dragon_head");
        flatten("minecraft:golden_apple", 0, "minecraft:golden_apple");
        flatten("minecraft:golden_apple", 1, "minecraft:enchanted_golden_apple");
        flatten("minecraft:fireworks", 0, "minecraft:firework_rocket");
        flatten("minecraft:firework_charge", 0, "minecraft:firework_star");
        flatten("minecraft:dye", 0, "minecraft:ink_sac");
        flatten("minecraft:dye", 1, "minecraft:rose_red");
        flatten("minecraft:dye", 2, "minecraft:cactus_green");
        flatten("minecraft:dye", 3, "minecraft:cocoa_beans");
        flatten("minecraft:dye", 4, "minecraft:lapis_lazuli");
        flatten("minecraft:dye", 5, "minecraft:purple_dye");
        flatten("minecraft:dye", 6, "minecraft:cyan_dye");
        flatten("minecraft:dye", 7, "minecraft:light_gray_dye");
        flatten("minecraft:dye", 8, "minecraft:gray_dye");
        flatten("minecraft:dye", 9, "minecraft:pink_dye");
        flatten("minecraft:dye", 10, "minecraft:lime_dye");
        flatten("minecraft:dye", 11, "minecraft:dandelion_yellow");
        flatten("minecraft:dye", 12, "minecraft:light_blue_dye");
        flatten("minecraft:dye", 13, "minecraft:magenta_dye");
        flatten("minecraft:dye", 14, "minecraft:orange_dye");
        flatten("minecraft:dye", 15, "minecraft:bone_meal");
        flatten("minecraft:silver_shulker_box", 0, "minecraft:light_gray_shulker_box");
        flatten("minecraft:fence", 0, "minecraft:oak_fence");
        flatten("minecraft:fence_gate", 0, "minecraft:oak_fence_gate");
        flatten("minecraft:wooden_door", 0, "minecraft:oak_door");
        flatten("minecraft:boat", 0, "minecraft:oak_boat");
        flatten("minecraft:lit_pumpkin", 0, "minecraft:jack_o_lantern");
        flatten("minecraft:pumpkin", 0, "minecraft:carved_pumpkin");
        flatten("minecraft:trapdoor", 0, "minecraft:oak_trapdoor");
        flatten("minecraft:nether_brick", 0, "minecraft:nether_bricks");
        flatten("minecraft:red_nether_brick", 0, "minecraft:red_nether_bricks");
        flatten("minecraft:netherbrick", 0, "minecraft:nether_brick");
        flatten("minecraft:wooden_button", 0, "minecraft:oak_button");
        flatten("minecraft:wooden_pressure_plate", 0, "minecraft:oak_pressure_plate");
        flatten("minecraft:noteblock", 0, "minecraft:note_block");
        flatten("minecraft:bed", 0, "minecraft:white_bed");
        flatten("minecraft:bed", 1, "minecraft:orange_bed");
        flatten("minecraft:bed", 2, "minecraft:magenta_bed");
        flatten("minecraft:bed", 3, "minecraft:light_blue_bed");
        flatten("minecraft:bed", 4, "minecraft:yellow_bed");
        flatten("minecraft:bed", 5, "minecraft:lime_bed");
        flatten("minecraft:bed", 6, "minecraft:pink_bed");
        flatten("minecraft:bed", 7, "minecraft:gray_bed");
        flatten("minecraft:bed", 8, "minecraft:light_gray_bed");
        flatten("minecraft:bed", 9, "minecraft:cyan_bed");
        flatten("minecraft:bed", 10, "minecraft:purple_bed");
        flatten("minecraft:bed", 11, "minecraft:blue_bed");
        flatten("minecraft:bed", 12, "minecraft:brown_bed");
        flatten("minecraft:bed", 13, "minecraft:green_bed");
        flatten("minecraft:bed", 14, "minecraft:red_bed");
        flatten("minecraft:bed", 15, "minecraft:black_bed");
        flatten("minecraft:banner", 15, "minecraft:white_banner");
        flatten("minecraft:banner", 14, "minecraft:orange_banner");
        flatten("minecraft:banner", 13, "minecraft:magenta_banner");
        flatten("minecraft:banner", 12, "minecraft:light_blue_banner");
        flatten("minecraft:banner", 11, "minecraft:yellow_banner");
        flatten("minecraft:banner", 10, "minecraft:lime_banner");
        flatten("minecraft:banner", 9, "minecraft:pink_banner");
        flatten("minecraft:banner", 8, "minecraft:gray_banner");
        flatten("minecraft:banner", 7, "minecraft:light_gray_banner");
        flatten("minecraft:banner", 6, "minecraft:cyan_banner");
        flatten("minecraft:banner", 5, "minecraft:purple_banner");
        flatten("minecraft:banner", 4, "minecraft:blue_banner");
        flatten("minecraft:banner", 3, "minecraft:brown_banner");
        flatten("minecraft:banner", 2, "minecraft:green_banner");
        flatten("minecraft:banner", 1, "minecraft:red_banner");
        flatten("minecraft:banner", 0, "minecraft:black_banner");
        flatten("minecraft:grass", 0, "minecraft:grass_block");
        flatten("minecraft:brick_block", 0, "minecraft:bricks");
        flatten("minecraft:end_bricks", 0, "minecraft:end_stone_bricks");
        flatten("minecraft:golden_rail", 0, "minecraft:powered_rail");
        flatten("minecraft:magma", 0, "minecraft:magma_block");
        flatten("minecraft:quartz_ore", 0, "minecraft:nether_quartz_ore");
        flatten("minecraft:reeds", 0, "minecraft:sugar_cane");
        flatten("minecraft:slime", 0, "minecraft:slime_block");
        flatten("minecraft:stone_stairs", 0, "minecraft:cobblestone_stairs");
        flatten("minecraft:waterlily", 0, "minecraft:lily_pad");
        flatten("minecraft:web", 0, "minecraft:cobweb");
        flatten("minecraft:snow", 0, "minecraft:snow_block");
        flatten("minecraft:snow_layer", 0, "minecraft:snow");
        flatten("minecraft:record_11", 0, "minecraft:music_disc_11");
        flatten("minecraft:record_13", 0, "minecraft:music_disc_13");
        flatten("minecraft:record_blocks", 0, "minecraft:music_disc_blocks");
        flatten("minecraft:record_cat", 0, "minecraft:music_disc_cat");
        flatten("minecraft:record_chirp", 0, "minecraft:music_disc_chirp");
        flatten("minecraft:record_far", 0, "minecraft:music_disc_far");
        flatten("minecraft:record_mall", 0, "minecraft:music_disc_mall");
        flatten("minecraft:record_mellohi", 0, "minecraft:music_disc_mellohi");
        flatten("minecraft:record_stal", 0, "minecraft:music_disc_stal");
        flatten("minecraft:record_strad", 0, "minecraft:music_disc_strad");
        flatten("minecraft:record_wait", 0, "minecraft:music_disc_wait");
        flatten("minecraft:record_ward", 0, "minecraft:music_disc_ward");

        flatten_data.items_with_damage.insert("minecraft:bow".to_owned(), ());
        flatten_data.items_with_damage.insert("minecraft:carrot_on_a_stick".to_owned(), ());
        flatten_data.items_with_damage.insert("minecraft:chainmail_boots".to_owned(), ());
        flatten_data.items_with_damage.insert("minecraft:chainmail_chestplate".to_owned(), ());
        flatten_data.items_with_damage.insert("minecraft:chainmail_helmet".to_owned(), ());
        flatten_data.items_with_damage.insert("minecraft:chainmail_leggings".to_owned(), ());
        flatten_data.items_with_damage.insert("minecraft:diamond_axe".to_owned(), ());
        flatten_data.items_with_damage.insert("minecraft:diamond_boots".to_owned(), ());
        flatten_data.items_with_damage.insert("minecraft:diamond_chestplate".to_owned(), ());
        flatten_data.items_with_damage.insert("minecraft:diamond_helmet".to_owned(), ());
        flatten_data.items_with_damage.insert("minecraft:diamond_hoe".to_owned(), ());
        flatten_data.items_with_damage.insert("minecraft:diamond_leggings".to_owned(), ());
        flatten_data.items_with_damage.insert("minecraft:diamond_pickaxe".to_owned(), ());
        flatten_data.items_with_damage.insert("minecraft:diamond_shovel".to_owned(), ());
        flatten_data.items_with_damage.insert("minecraft:diamond_sword".to_owned(), ());
        flatten_data.items_with_damage.insert("minecraft:elytra".to_owned(), ());
        flatten_data.items_with_damage.insert("minecraft:fishing_rod".to_owned(), ());
        flatten_data.items_with_damage.insert("minecraft:flint_and_steel".to_owned(), ());
        flatten_data.items_with_damage.insert("minecraft:golden_axe".to_owned(), ());
        flatten_data.items_with_damage.insert("minecraft:golden_boots".to_owned(), ());
        flatten_data.items_with_damage.insert("minecraft:golden_chestplate".to_owned(), ());
        flatten_data.items_with_damage.insert("minecraft:golden_helmet".to_owned(), ());
        flatten_data.items_with_damage.insert("minecraft:golden_hoe".to_owned(), ());
        flatten_data.items_with_damage.insert("minecraft:golden_leggings".to_owned(), ());
        flatten_data.items_with_damage.insert("minecraft:golden_pickaxe".to_owned(), ());
        flatten_data.items_with_damage.insert("minecraft:golden_shovel".to_owned(), ());
        flatten_data.items_with_damage.insert("minecraft:golden_sword".to_owned(), ());
        flatten_data.items_with_damage.insert("minecraft:iron_axe".to_owned(), ());
        flatten_data.items_with_damage.insert("minecraft:iron_boots".to_owned(), ());
        flatten_data.items_with_damage.insert("minecraft:iron_chestplate".to_owned(), ());
        flatten_data.items_with_damage.insert("minecraft:iron_helmet".to_owned(), ());
        flatten_data.items_with_damage.insert("minecraft:iron_hoe".to_owned(), ());
        flatten_data.items_with_damage.insert("minecraft:iron_leggings".to_owned(), ());
        flatten_data.items_with_damage.insert("minecraft:iron_pickaxe".to_owned(), ());
        flatten_data.items_with_damage.insert("minecraft:iron_shovel".to_owned(), ());
        flatten_data.items_with_damage.insert("minecraft:iron_sword".to_owned(), ());
        flatten_data.items_with_damage.insert("minecraft:leather_boots".to_owned(), ());
        flatten_data.items_with_damage.insert("minecraft:leather_chestplate".to_owned(), ());
        flatten_data.items_with_damage.insert("minecraft:leather_helmet".to_owned(), ());
        flatten_data.items_with_damage.insert("minecraft:leather_leggings".to_owned(), ());
        flatten_data.items_with_damage.insert("minecraft:shears".to_owned(), ());
        flatten_data.items_with_damage.insert("minecraft:shield".to_owned(), ());
        flatten_data.items_with_damage.insert("minecraft:stone_axe".to_owned(), ());
        flatten_data.items_with_damage.insert("minecraft:stone_hoe".to_owned(), ());
        flatten_data.items_with_damage.insert("minecraft:stone_pickaxe".to_owned(), ());
        flatten_data.items_with_damage.insert("minecraft:stone_shovel".to_owned(), ());
        flatten_data.items_with_damage.insert("minecraft:stone_sword".to_owned(), ());
        flatten_data.items_with_damage.insert("minecraft:wooden_axe".to_owned(), ());
        flatten_data.items_with_damage.insert("minecraft:wooden_hoe".to_owned(), ());
        flatten_data.items_with_damage.insert("minecraft:wooden_pickaxe".to_owned(), ());
        flatten_data.items_with_damage.insert("minecraft:wooden_shovel".to_owned(), ());
        flatten_data.items_with_damage.insert("minecraft:wooden_sword".to_owned(), ());

        flatten_data
    })
}

pub(crate) fn flatten_item(old_name: &str, data: u8) -> Option<&String> {
    let flatten_data = flatten_data();
    if flatten_data.ids_requiring_flattening.contains_key(old_name) {
        flatten_data.flatten_map.get(&(old_name.to_string(), data))
            .or_else(|| flatten_data.flatten_map.get(&(old_name.to_string(), 0)))
    } else {
        None
    }
}

pub(crate) struct ConverterFlattenItemStack<T: Types + ?Sized> {
    _phantom: PhantomData<T>,
}

impl<T: Types + ?Sized> ConverterFlattenItemStack<T> {
    pub(crate) fn new() -> Self { Self { _phantom: PhantomData } }
}

impl<T: Types + ?Sized> DataConverterFunc<T::Map> for ConverterFlattenItemStack<T> {
    fn convert(&self, data: &mut T::Map, _from_version: DataVersion, _to_version: DataVersion) {
        if let Some(id) = data.get_string("id") {
            let flatten_data = flatten_data();

            let damage = data.get_i64("Damage").unwrap_or(0) as i16;

            if flatten_data.ids_requiring_flattening.contains_key(id) {
                let remap = damage.try_into().ok()
                    .and_then(|damage: u8| flatten_data.flatten_map.get(&(id.to_owned(), damage)))
                    .or_else(|| flatten_data.flatten_map.get(&(id.to_owned(), 0)))
                    .unwrap().clone();
                data.set("id", T::Object::create_string(remap));
            } else if damage != 0 && flatten_data.items_with_damage.contains_key(id) {
                // migrate damage
                if data.get_map("tag").is_none() {
                    data.set("tag", T::Object::create_map(T::Map::create_empty()));
                }
                let tag = data.get_map_mut("tag").unwrap();
                tag.set("Damage", T::Object::create_int(damage as i32));
            }

            data.remove("Damage");
        }
    }
}
