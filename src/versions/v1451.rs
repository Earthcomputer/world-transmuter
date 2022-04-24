use std::lazy::SyncOnceCell;
use std::marker::PhantomData;
use rust_dataconverter_engine::{convert_map_in_map, convert_map_list_in_map, convert_object_in_map, data_converter_func, data_walker, DataHook, DataType, DataVersion, DataWalkerMapListPaths, DataWalkerMapTypePaths, ListType, MapType, ObjectType, Types};
use crate::helpers::{block_flattening_v1450, flatten_item_stack_v1451, item_name_v102};
use crate::helpers::flatten_chunk_v1451::ConverterFlattenChunk;
use crate::helpers::flatten_item_stack_v1451::ConverterFlattenItemStack;
use crate::helpers::rename::rename_keys_in_map;
use crate::helpers::resource_location::ResourceLocation;
use crate::MinecraftTypesMut;

const VERSION: u32 = 1451;

static BLOCK_NAME_TO_ID: SyncOnceCell<rust_dataconverter_engine::Map<String, u8>> = SyncOnceCell::new();

fn block_name_to_id() -> &'static rust_dataconverter_engine::Map<String, u8> {
    BLOCK_NAME_TO_ID.get_or_init(|| {
        let mut map = rust_dataconverter_engine::Map::new();
        map.insert("minecraft:air".to_owned(), 0);
        map.insert("minecraft:stone".to_owned(), 1);
        map.insert("minecraft:grass".to_owned(), 2);
        map.insert("minecraft:dirt".to_owned(), 3);
        map.insert("minecraft:cobblestone".to_owned(), 4);
        map.insert("minecraft:planks".to_owned(), 5);
        map.insert("minecraft:sapling".to_owned(), 6);
        map.insert("minecraft:bedrock".to_owned(), 7);
        map.insert("minecraft:flowing_water".to_owned(), 8);
        map.insert("minecraft:water".to_owned(), 9);
        map.insert("minecraft:flowing_lava".to_owned(), 10);
        map.insert("minecraft:lava".to_owned(), 11);
        map.insert("minecraft:sand".to_owned(), 12);
        map.insert("minecraft:gravel".to_owned(), 13);
        map.insert("minecraft:gold_ore".to_owned(), 14);
        map.insert("minecraft:iron_ore".to_owned(), 15);
        map.insert("minecraft:coal_ore".to_owned(), 16);
        map.insert("minecraft:log".to_owned(), 17);
        map.insert("minecraft:leaves".to_owned(), 18);
        map.insert("minecraft:sponge".to_owned(), 19);
        map.insert("minecraft:glass".to_owned(), 20);
        map.insert("minecraft:lapis_ore".to_owned(), 21);
        map.insert("minecraft:lapis_block".to_owned(), 22);
        map.insert("minecraft:dispenser".to_owned(), 23);
        map.insert("minecraft:sandstone".to_owned(), 24);
        map.insert("minecraft:noteblock".to_owned(), 25);
        map.insert("minecraft:bed".to_owned(), 26);
        map.insert("minecraft:golden_rail".to_owned(), 27);
        map.insert("minecraft:detector_rail".to_owned(), 28);
        map.insert("minecraft:sticky_piston".to_owned(), 29);
        map.insert("minecraft:web".to_owned(), 30);
        map.insert("minecraft:tallgrass".to_owned(), 31);
        map.insert("minecraft:deadbush".to_owned(), 32);
        map.insert("minecraft:piston".to_owned(), 33);
        map.insert("minecraft:piston_head".to_owned(), 34);
        map.insert("minecraft:wool".to_owned(), 35);
        map.insert("minecraft:piston_extension".to_owned(), 36);
        map.insert("minecraft:yellow_flower".to_owned(), 37);
        map.insert("minecraft:red_flower".to_owned(), 38);
        map.insert("minecraft:brown_mushroom".to_owned(), 39);
        map.insert("minecraft:red_mushroom".to_owned(), 40);
        map.insert("minecraft:gold_block".to_owned(), 41);
        map.insert("minecraft:iron_block".to_owned(), 42);
        map.insert("minecraft:double_stone_slab".to_owned(), 43);
        map.insert("minecraft:stone_slab".to_owned(), 44);
        map.insert("minecraft:brick_block".to_owned(), 45);
        map.insert("minecraft:tnt".to_owned(), 46);
        map.insert("minecraft:bookshelf".to_owned(), 47);
        map.insert("minecraft:mossy_cobblestone".to_owned(), 48);
        map.insert("minecraft:obsidian".to_owned(), 49);
        map.insert("minecraft:torch".to_owned(), 50);
        map.insert("minecraft:fire".to_owned(), 51);
        map.insert("minecraft:mob_spawner".to_owned(), 52);
        map.insert("minecraft:oak_stairs".to_owned(), 53);
        map.insert("minecraft:chest".to_owned(), 54);
        map.insert("minecraft:redstone_wire".to_owned(), 55);
        map.insert("minecraft:diamond_ore".to_owned(), 56);
        map.insert("minecraft:diamond_block".to_owned(), 57);
        map.insert("minecraft:crafting_table".to_owned(), 58);
        map.insert("minecraft:wheat".to_owned(), 59);
        map.insert("minecraft:farmland".to_owned(), 60);
        map.insert("minecraft:furnace".to_owned(), 61);
        map.insert("minecraft:lit_furnace".to_owned(), 62);
        map.insert("minecraft:standing_sign".to_owned(), 63);
        map.insert("minecraft:wooden_door".to_owned(), 64);
        map.insert("minecraft:ladder".to_owned(), 65);
        map.insert("minecraft:rail".to_owned(), 66);
        map.insert("minecraft:stone_stairs".to_owned(), 67);
        map.insert("minecraft:wall_sign".to_owned(), 68);
        map.insert("minecraft:lever".to_owned(), 69);
        map.insert("minecraft:stone_pressure_plate".to_owned(), 70);
        map.insert("minecraft:iron_door".to_owned(), 71);
        map.insert("minecraft:wooden_pressure_plate".to_owned(), 72);
        map.insert("minecraft:redstone_ore".to_owned(), 73);
        map.insert("minecraft:lit_redstone_ore".to_owned(), 74);
        map.insert("minecraft:unlit_redstone_torch".to_owned(), 75);
        map.insert("minecraft:redstone_torch".to_owned(), 76);
        map.insert("minecraft:stone_button".to_owned(), 77);
        map.insert("minecraft:snow_layer".to_owned(), 78);
        map.insert("minecraft:ice".to_owned(), 79);
        map.insert("minecraft:snow".to_owned(), 80);
        map.insert("minecraft:cactus".to_owned(), 81);
        map.insert("minecraft:clay".to_owned(), 82);
        map.insert("minecraft:reeds".to_owned(), 83);
        map.insert("minecraft:jukebox".to_owned(), 84);
        map.insert("minecraft:fence".to_owned(), 85);
        map.insert("minecraft:pumpkin".to_owned(), 86);
        map.insert("minecraft:netherrack".to_owned(), 87);
        map.insert("minecraft:soul_sand".to_owned(), 88);
        map.insert("minecraft:glowstone".to_owned(), 89);
        map.insert("minecraft:portal".to_owned(), 90);
        map.insert("minecraft:lit_pumpkin".to_owned(), 91);
        map.insert("minecraft:cake".to_owned(), 92);
        map.insert("minecraft:unpowered_repeater".to_owned(), 93);
        map.insert("minecraft:powered_repeater".to_owned(), 94);
        map.insert("minecraft:stained_glass".to_owned(), 95);
        map.insert("minecraft:trapdoor".to_owned(), 96);
        map.insert("minecraft:monster_egg".to_owned(), 97);
        map.insert("minecraft:stonebrick".to_owned(), 98);
        map.insert("minecraft:brown_mushroom_block".to_owned(), 99);
        map.insert("minecraft:red_mushroom_block".to_owned(), 100);
        map.insert("minecraft:iron_bars".to_owned(), 101);
        map.insert("minecraft:glass_pane".to_owned(), 102);
        map.insert("minecraft:melon_block".to_owned(), 103);
        map.insert("minecraft:pumpkin_stem".to_owned(), 104);
        map.insert("minecraft:melon_stem".to_owned(), 105);
        map.insert("minecraft:vine".to_owned(), 106);
        map.insert("minecraft:fence_gate".to_owned(), 107);
        map.insert("minecraft:brick_stairs".to_owned(), 108);
        map.insert("minecraft:stone_brick_stairs".to_owned(), 109);
        map.insert("minecraft:mycelium".to_owned(), 110);
        map.insert("minecraft:waterlily".to_owned(), 111);
        map.insert("minecraft:nether_brick".to_owned(), 112);
        map.insert("minecraft:nether_brick_fence".to_owned(), 113);
        map.insert("minecraft:nether_brick_stairs".to_owned(), 114);
        map.insert("minecraft:nether_wart".to_owned(), 115);
        map.insert("minecraft:enchanting_table".to_owned(), 116);
        map.insert("minecraft:brewing_stand".to_owned(), 117);
        map.insert("minecraft:cauldron".to_owned(), 118);
        map.insert("minecraft:end_portal".to_owned(), 119);
        map.insert("minecraft:end_portal_frame".to_owned(), 120);
        map.insert("minecraft:end_stone".to_owned(), 121);
        map.insert("minecraft:dragon_egg".to_owned(), 122);
        map.insert("minecraft:redstone_lamp".to_owned(), 123);
        map.insert("minecraft:lit_redstone_lamp".to_owned(), 124);
        map.insert("minecraft:double_wooden_slab".to_owned(), 125);
        map.insert("minecraft:wooden_slab".to_owned(), 126);
        map.insert("minecraft:cocoa".to_owned(), 127);
        map.insert("minecraft:sandstone_stairs".to_owned(), 128);
        map.insert("minecraft:emerald_ore".to_owned(), 129);
        map.insert("minecraft:ender_chest".to_owned(), 130);
        map.insert("minecraft:tripwire_hook".to_owned(), 131);
        map.insert("minecraft:tripwire".to_owned(), 132);
        map.insert("minecraft:emerald_block".to_owned(), 133);
        map.insert("minecraft:spruce_stairs".to_owned(), 134);
        map.insert("minecraft:birch_stairs".to_owned(), 135);
        map.insert("minecraft:jungle_stairs".to_owned(), 136);
        map.insert("minecraft:command_block".to_owned(), 137);
        map.insert("minecraft:beacon".to_owned(), 138);
        map.insert("minecraft:cobblestone_wall".to_owned(), 139);
        map.insert("minecraft:flower_pot".to_owned(), 140);
        map.insert("minecraft:carrots".to_owned(), 141);
        map.insert("minecraft:potatoes".to_owned(), 142);
        map.insert("minecraft:wooden_button".to_owned(), 143);
        map.insert("minecraft:skull".to_owned(), 144);
        map.insert("minecraft:anvil".to_owned(), 145);
        map.insert("minecraft:trapped_chest".to_owned(), 146);
        map.insert("minecraft:light_weighted_pressure_plate".to_owned(), 147);
        map.insert("minecraft:heavy_weighted_pressure_plate".to_owned(), 148);
        map.insert("minecraft:unpowered_comparator".to_owned(), 149);
        map.insert("minecraft:powered_comparator".to_owned(), 150);
        map.insert("minecraft:daylight_detector".to_owned(), 151);
        map.insert("minecraft:redstone_block".to_owned(), 152);
        map.insert("minecraft:quartz_ore".to_owned(), 153);
        map.insert("minecraft:hopper".to_owned(), 154);
        map.insert("minecraft:quartz_block".to_owned(), 155);
        map.insert("minecraft:quartz_stairs".to_owned(), 156);
        map.insert("minecraft:activator_rail".to_owned(), 157);
        map.insert("minecraft:dropper".to_owned(), 158);
        map.insert("minecraft:stained_hardened_clay".to_owned(), 159);
        map.insert("minecraft:stained_glass_pane".to_owned(), 160);
        map.insert("minecraft:leaves2".to_owned(), 161);
        map.insert("minecraft:log2".to_owned(), 162);
        map.insert("minecraft:acacia_stairs".to_owned(), 163);
        map.insert("minecraft:dark_oak_stairs".to_owned(), 164);
        map.insert("minecraft:slime".to_owned(), 165);
        map.insert("minecraft:barrier".to_owned(), 166);
        map.insert("minecraft:iron_trapdoor".to_owned(), 167);
        map.insert("minecraft:prismarine".to_owned(), 168);
        map.insert("minecraft:sea_lantern".to_owned(), 169);
        map.insert("minecraft:hay_block".to_owned(), 170);
        map.insert("minecraft:carpet".to_owned(), 171);
        map.insert("minecraft:hardened_clay".to_owned(), 172);
        map.insert("minecraft:coal_block".to_owned(), 173);
        map.insert("minecraft:packed_ice".to_owned(), 174);
        map.insert("minecraft:double_plant".to_owned(), 175);
        map.insert("minecraft:standing_banner".to_owned(), 176);
        map.insert("minecraft:wall_banner".to_owned(), 177);
        map.insert("minecraft:daylight_detector_inverted".to_owned(), 178);
        map.insert("minecraft:red_sandstone".to_owned(), 179);
        map.insert("minecraft:red_sandstone_stairs".to_owned(), 180);
        map.insert("minecraft:double_stone_slab2".to_owned(), 181);
        map.insert("minecraft:stone_slab2".to_owned(), 182);
        map.insert("minecraft:spruce_fence_gate".to_owned(), 183);
        map.insert("minecraft:birch_fence_gate".to_owned(), 184);
        map.insert("minecraft:jungle_fence_gate".to_owned(), 185);
        map.insert("minecraft:dark_oak_fence_gate".to_owned(), 186);
        map.insert("minecraft:acacia_fence_gate".to_owned(), 187);
        map.insert("minecraft:spruce_fence".to_owned(), 188);
        map.insert("minecraft:birch_fence".to_owned(), 189);
        map.insert("minecraft:jungle_fence".to_owned(), 190);
        map.insert("minecraft:dark_oak_fence".to_owned(), 191);
        map.insert("minecraft:acacia_fence".to_owned(), 192);
        map.insert("minecraft:spruce_door".to_owned(), 193);
        map.insert("minecraft:birch_door".to_owned(), 194);
        map.insert("minecraft:jungle_door".to_owned(), 195);
        map.insert("minecraft:acacia_door".to_owned(), 196);
        map.insert("minecraft:dark_oak_door".to_owned(), 197);
        map.insert("minecraft:end_rod".to_owned(), 198);
        map.insert("minecraft:chorus_plant".to_owned(), 199);
        map.insert("minecraft:chorus_flower".to_owned(), 200);
        map.insert("minecraft:purpur_block".to_owned(), 201);
        map.insert("minecraft:purpur_pillar".to_owned(), 202);
        map.insert("minecraft:purpur_stairs".to_owned(), 203);
        map.insert("minecraft:purpur_double_slab".to_owned(), 204);
        map.insert("minecraft:purpur_slab".to_owned(), 205);
        map.insert("minecraft:end_bricks".to_owned(), 206);
        map.insert("minecraft:beetroots".to_owned(), 207);
        map.insert("minecraft:grass_path".to_owned(), 208);
        map.insert("minecraft:end_gateway".to_owned(), 209);
        map.insert("minecraft:repeating_command_block".to_owned(), 210);
        map.insert("minecraft:chain_command_block".to_owned(), 211);
        map.insert("minecraft:frosted_ice".to_owned(), 212);
        map.insert("minecraft:magma".to_owned(), 213);
        map.insert("minecraft:nether_wart_block".to_owned(), 214);
        map.insert("minecraft:red_nether_brick".to_owned(), 215);
        map.insert("minecraft:bone_block".to_owned(), 216);
        map.insert("minecraft:structure_void".to_owned(), 217);
        map.insert("minecraft:observer".to_owned(), 218);
        map.insert("minecraft:white_shulker_box".to_owned(), 219);
        map.insert("minecraft:orange_shulker_box".to_owned(), 220);
        map.insert("minecraft:magenta_shulker_box".to_owned(), 221);
        map.insert("minecraft:light_blue_shulker_box".to_owned(), 222);
        map.insert("minecraft:yellow_shulker_box".to_owned(), 223);
        map.insert("minecraft:lime_shulker_box".to_owned(), 224);
        map.insert("minecraft:pink_shulker_box".to_owned(), 225);
        map.insert("minecraft:gray_shulker_box".to_owned(), 226);
        map.insert("minecraft:silver_shulker_box".to_owned(), 227);
        map.insert("minecraft:cyan_shulker_box".to_owned(), 228);
        map.insert("minecraft:purple_shulker_box".to_owned(), 229);
        map.insert("minecraft:blue_shulker_box".to_owned(), 230);
        map.insert("minecraft:brown_shulker_box".to_owned(), 231);
        map.insert("minecraft:green_shulker_box".to_owned(), 232);
        map.insert("minecraft:red_shulker_box".to_owned(), 233);
        map.insert("minecraft:black_shulker_box".to_owned(), 234);
        map.insert("minecraft:white_glazed_terracotta".to_owned(), 235);
        map.insert("minecraft:orange_glazed_terracotta".to_owned(), 236);
        map.insert("minecraft:magenta_glazed_terracotta".to_owned(), 237);
        map.insert("minecraft:light_blue_glazed_terracotta".to_owned(), 238);
        map.insert("minecraft:yellow_glazed_terracotta".to_owned(), 239);
        map.insert("minecraft:lime_glazed_terracotta".to_owned(), 240);
        map.insert("minecraft:pink_glazed_terracotta".to_owned(), 241);
        map.insert("minecraft:gray_glazed_terracotta".to_owned(), 242);
        map.insert("minecraft:silver_glazed_terracotta".to_owned(), 243);
        map.insert("minecraft:cyan_glazed_terracotta".to_owned(), 244);
        map.insert("minecraft:purple_glazed_terracotta".to_owned(), 245);
        map.insert("minecraft:blue_glazed_terracotta".to_owned(), 246);
        map.insert("minecraft:brown_glazed_terracotta".to_owned(), 247);
        map.insert("minecraft:green_glazed_terracotta".to_owned(), 248);
        map.insert("minecraft:red_glazed_terracotta".to_owned(), 249);
        map.insert("minecraft:black_glazed_terracotta".to_owned(), 250);
        map.insert("minecraft:concrete".to_owned(), 251);
        map.insert("minecraft:concrete_powder".to_owned(), 252);
        map.insert("minecraft:structure_block".to_owned(), 255);
        map
    })
}

static ENTITY_ID_TO_NEW_EGG_ID: SyncOnceCell<rust_dataconverter_engine::Map<String, String>> = SyncOnceCell::new();

fn entity_id_to_new_egg_id() -> &'static rust_dataconverter_engine::Map<String, String> {
    ENTITY_ID_TO_NEW_EGG_ID.get_or_init(|| {
        let mut map = rust_dataconverter_engine::Map::new();
        map.insert("minecraft:bat".to_owned(), "minecraft:bat_spawn_egg".to_owned());
        map.insert("minecraft:blaze".to_owned(), "minecraft:blaze_spawn_egg".to_owned());
        map.insert("minecraft:cave_spider".to_owned(), "minecraft:cave_spider_spawn_egg".to_owned());
        map.insert("minecraft:chicken".to_owned(), "minecraft:chicken_spawn_egg".to_owned());
        map.insert("minecraft:cow".to_owned(), "minecraft:cow_spawn_egg".to_owned());
        map.insert("minecraft:creeper".to_owned(), "minecraft:creeper_spawn_egg".to_owned());
        map.insert("minecraft:donkey".to_owned(), "minecraft:donkey_spawn_egg".to_owned());
        map.insert("minecraft:elder_guardian".to_owned(), "minecraft:elder_guardian_spawn_egg".to_owned());
        map.insert("minecraft:enderman".to_owned(), "minecraft:enderman_spawn_egg".to_owned());
        map.insert("minecraft:endermite".to_owned(), "minecraft:endermite_spawn_egg".to_owned());
        map.insert("minecraft:evocation_illager".to_owned(), "minecraft:evocation_illager_spawn_egg".to_owned());
        map.insert("minecraft:ghast".to_owned(), "minecraft:ghast_spawn_egg".to_owned());
        map.insert("minecraft:guardian".to_owned(), "minecraft:guardian_spawn_egg".to_owned());
        map.insert("minecraft:horse".to_owned(), "minecraft:horse_spawn_egg".to_owned());
        map.insert("minecraft:husk".to_owned(), "minecraft:husk_spawn_egg".to_owned());
        map.insert("minecraft:llama".to_owned(), "minecraft:llama_spawn_egg".to_owned());
        map.insert("minecraft:magma_cube".to_owned(), "minecraft:magma_cube_spawn_egg".to_owned());
        map.insert("minecraft:mooshroom".to_owned(), "minecraft:mooshroom_spawn_egg".to_owned());
        map.insert("minecraft:mule".to_owned(), "minecraft:mule_spawn_egg".to_owned());
        map.insert("minecraft:ocelot".to_owned(), "minecraft:ocelot_spawn_egg".to_owned());
        map.insert("minecraft:pufferfish".to_owned(), "minecraft:pufferfish_spawn_egg".to_owned());
        map.insert("minecraft:parrot".to_owned(), "minecraft:parrot_spawn_egg".to_owned());
        map.insert("minecraft:pig".to_owned(), "minecraft:pig_spawn_egg".to_owned());
        map.insert("minecraft:polar_bear".to_owned(), "minecraft:polar_bear_spawn_egg".to_owned());
        map.insert("minecraft:rabbit".to_owned(), "minecraft:rabbit_spawn_egg".to_owned());
        map.insert("minecraft:sheep".to_owned(), "minecraft:sheep_spawn_egg".to_owned());
        map.insert("minecraft:shulker".to_owned(), "minecraft:shulker_spawn_egg".to_owned());
        map.insert("minecraft:silverfish".to_owned(), "minecraft:silverfish_spawn_egg".to_owned());
        map.insert("minecraft:skeleton".to_owned(), "minecraft:skeleton_spawn_egg".to_owned());
        map.insert("minecraft:skeleton_horse".to_owned(), "minecraft:skeleton_horse_spawn_egg".to_owned());
        map.insert("minecraft:slime".to_owned(), "minecraft:slime_spawn_egg".to_owned());
        map.insert("minecraft:spider".to_owned(), "minecraft:spider_spawn_egg".to_owned());
        map.insert("minecraft:squid".to_owned(), "minecraft:squid_spawn_egg".to_owned());
        map.insert("minecraft:stray".to_owned(), "minecraft:stray_spawn_egg".to_owned());
        map.insert("minecraft:turtle".to_owned(), "minecraft:turtle_spawn_egg".to_owned());
        map.insert("minecraft:vex".to_owned(), "minecraft:vex_spawn_egg".to_owned());
        map.insert("minecraft:villager".to_owned(), "minecraft:villager_spawn_egg".to_owned());
        map.insert("minecraft:vindication_illager".to_owned(), "minecraft:vindication_illager_spawn_egg".to_owned());
        map.insert("minecraft:witch".to_owned(), "minecraft:witch_spawn_egg".to_owned());
        map.insert("minecraft:wither_skeleton".to_owned(), "minecraft:wither_skeleton_spawn_egg".to_owned());
        map.insert("minecraft:wolf".to_owned(), "minecraft:wolf_spawn_egg".to_owned());
        map.insert("minecraft:zombie".to_owned(), "minecraft:zombie_spawn_egg".to_owned());
        map.insert("minecraft:zombie_horse".to_owned(), "minecraft:zombie_horse_spawn_egg".to_owned());
        map.insert("minecraft:zombie_pigman".to_owned(), "minecraft:zombie_pigman_spawn_egg".to_owned());
        map.insert("minecraft:zombie_villager".to_owned(), "minecraft:zombie_villager_spawn_egg".to_owned());
        map
    })
}

static SKIP_STATS: SyncOnceCell<rust_dataconverter_engine::Map<String, ()>> = SyncOnceCell::new();

fn skip_stats() -> &'static rust_dataconverter_engine::Map<String, ()> {
    SKIP_STATS.get_or_init(|| {
        let mut map = rust_dataconverter_engine::Map::new();
        map.insert("stat.craftItem.minecraft.spawn_egg".to_owned(), ());
        map.insert("stat.useItem.minecraft.spawn_egg".to_owned(), ());
        map.insert("stat.breakItem.minecraft.spawn_egg".to_owned(), ());
        map.insert("stat.pickup.minecraft.spawn_egg".to_owned(), ());
        map.insert("stat.drop.minecraft.spawn_egg".to_owned(), ());
        map
    })
}

static CUSTOM_STATS: SyncOnceCell<rust_dataconverter_engine::Map<String, String>> = SyncOnceCell::new();

fn custom_stats() -> &'static rust_dataconverter_engine::Map<String, String> {
    CUSTOM_STATS.get_or_init(|| {
        let mut map = rust_dataconverter_engine::Map::new();
        map.insert("stat.leaveGame".to_owned(), "minecraft:leave_game".to_owned());
        map.insert("stat.playOneMinute".to_owned(), "minecraft:play_one_minute".to_owned());
        map.insert("stat.timeSinceDeath".to_owned(), "minecraft:time_since_death".to_owned());
        map.insert("stat.sneakTime".to_owned(), "minecraft:sneak_time".to_owned());
        map.insert("stat.walkOneCm".to_owned(), "minecraft:walk_one_cm".to_owned());
        map.insert("stat.crouchOneCm".to_owned(), "minecraft:crouch_one_cm".to_owned());
        map.insert("stat.sprintOneCm".to_owned(), "minecraft:sprint_one_cm".to_owned());
        map.insert("stat.swimOneCm".to_owned(), "minecraft:swim_one_cm".to_owned());
        map.insert("stat.fallOneCm".to_owned(), "minecraft:fall_one_cm".to_owned());
        map.insert("stat.climbOneCm".to_owned(), "minecraft:climb_one_cm".to_owned());
        map.insert("stat.flyOneCm".to_owned(), "minecraft:fly_one_cm".to_owned());
        map.insert("stat.diveOneCm".to_owned(), "minecraft:dive_one_cm".to_owned());
        map.insert("stat.minecartOneCm".to_owned(), "minecraft:minecart_one_cm".to_owned());
        map.insert("stat.boatOneCm".to_owned(), "minecraft:boat_one_cm".to_owned());
        map.insert("stat.pigOneCm".to_owned(), "minecraft:pig_one_cm".to_owned());
        map.insert("stat.horseOneCm".to_owned(), "minecraft:horse_one_cm".to_owned());
        map.insert("stat.aviateOneCm".to_owned(), "minecraft:aviate_one_cm".to_owned());
        map.insert("stat.jump".to_owned(), "minecraft:jump".to_owned());
        map.insert("stat.drop".to_owned(), "minecraft:drop".to_owned());
        map.insert("stat.damageDealt".to_owned(), "minecraft:damage_dealt".to_owned());
        map.insert("stat.damageTaken".to_owned(), "minecraft:damage_taken".to_owned());
        map.insert("stat.deaths".to_owned(), "minecraft:deaths".to_owned());
        map.insert("stat.mobKills".to_owned(), "minecraft:mob_kills".to_owned());
        map.insert("stat.animalsBred".to_owned(), "minecraft:animals_bred".to_owned());
        map.insert("stat.playerKills".to_owned(), "minecraft:player_kills".to_owned());
        map.insert("stat.fishCaught".to_owned(), "minecraft:fish_caught".to_owned());
        map.insert("stat.talkedToVillager".to_owned(), "minecraft:talked_to_villager".to_owned());
        map.insert("stat.tradedWithVillager".to_owned(), "minecraft:traded_with_villager".to_owned());
        map.insert("stat.cakeSlicesEaten".to_owned(), "minecraft:eat_cake_slice".to_owned());
        map.insert("stat.cauldronFilled".to_owned(), "minecraft:fill_cauldron".to_owned());
        map.insert("stat.cauldronUsed".to_owned(), "minecraft:use_cauldron".to_owned());
        map.insert("stat.armorCleaned".to_owned(), "minecraft:clean_armor".to_owned());
        map.insert("stat.bannerCleaned".to_owned(), "minecraft:clean_banner".to_owned());
        map.insert("stat.brewingstandInteraction".to_owned(), "minecraft:interact_with_brewingstand".to_owned());
        map.insert("stat.beaconInteraction".to_owned(), "minecraft:interact_with_beacon".to_owned());
        map.insert("stat.dropperInspected".to_owned(), "minecraft:inspect_dropper".to_owned());
        map.insert("stat.hopperInspected".to_owned(), "minecraft:inspect_hopper".to_owned());
        map.insert("stat.dispenserInspected".to_owned(), "minecraft:inspect_dispenser".to_owned());
        map.insert("stat.noteblockPlayed".to_owned(), "minecraft:play_noteblock".to_owned());
        map.insert("stat.noteblockTuned".to_owned(), "minecraft:tune_noteblock".to_owned());
        map.insert("stat.flowerPotted".to_owned(), "minecraft:pot_flower".to_owned());
        map.insert("stat.trappedChestTriggered".to_owned(), "minecraft:trigger_trapped_chest".to_owned());
        map.insert("stat.enderchestOpened".to_owned(), "minecraft:open_enderchest".to_owned());
        map.insert("stat.itemEnchanted".to_owned(), "minecraft:enchant_item".to_owned());
        map.insert("stat.recordPlayed".to_owned(), "minecraft:play_record".to_owned());
        map.insert("stat.furnaceInteraction".to_owned(), "minecraft:interact_with_furnace".to_owned());
        map.insert("stat.craftingTableInteraction".to_owned(), "minecraft:interact_with_crafting_table".to_owned());
        map.insert("stat.chestOpened".to_owned(), "minecraft:open_chest".to_owned());
        map.insert("stat.sleepInBed".to_owned(), "minecraft:sleep_in_bed".to_owned());
        map.insert("stat.shulkerBoxOpened".to_owned(), "minecraft:open_shulker_box".to_owned());
        map
    })
}

static ITEM_STATS: SyncOnceCell<rust_dataconverter_engine::Map<String, String>> = SyncOnceCell::new();

fn item_stats() -> &'static rust_dataconverter_engine::Map<String, String> {
    ITEM_STATS.get_or_init(|| {
        let mut map = rust_dataconverter_engine::Map::new();
        map.insert("stat.craftItem".to_owned(), "minecraft:crafted".to_owned());
        map.insert("stat.useItem".to_owned(), "minecraft:used".to_owned());
        map.insert("stat.breakItem".to_owned(), "minecraft:broken".to_owned());
        map.insert("stat.pickup".to_owned(), "minecraft:picked_up".to_owned());
        map.insert("stat.drop".to_owned(), "minecraft:dropped".to_owned());
        map
    })
}

static ENTITY_STATS: SyncOnceCell<rust_dataconverter_engine::Map<String, String>> = SyncOnceCell::new();

fn entity_stats() -> &'static rust_dataconverter_engine::Map<String, String> {
    ENTITY_STATS.get_or_init(|| {
        let mut map = rust_dataconverter_engine::Map::new();
        map.insert("stat.entityKilledBy".to_owned(), "minecraft:killed_by".to_owned());
        map.insert("stat.killEntity".to_owned(), "minecraft:killed".to_owned());
        map
    })
}

static ENTITY_MAP: SyncOnceCell<rust_dataconverter_engine::Map<String, String>> = SyncOnceCell::new();

fn entity_map() -> &'static rust_dataconverter_engine::Map<String, String> {
    ENTITY_MAP.get_or_init(|| {
        let mut map = rust_dataconverter_engine::Map::new();
        map.insert("Bat".to_owned(), "minecraft:bat".to_owned());
        map.insert("Blaze".to_owned(), "minecraft:blaze".to_owned());
        map.insert("CaveSpider".to_owned(), "minecraft:cave_spider".to_owned());
        map.insert("Chicken".to_owned(), "minecraft:chicken".to_owned());
        map.insert("Cow".to_owned(), "minecraft:cow".to_owned());
        map.insert("Creeper".to_owned(), "minecraft:creeper".to_owned());
        map.insert("Donkey".to_owned(), "minecraft:donkey".to_owned());
        map.insert("ElderGuardian".to_owned(), "minecraft:elder_guardian".to_owned());
        map.insert("Enderman".to_owned(), "minecraft:enderman".to_owned());
        map.insert("Endermite".to_owned(), "minecraft:endermite".to_owned());
        map.insert("EvocationIllager".to_owned(), "minecraft:evocation_illager".to_owned());
        map.insert("Ghast".to_owned(), "minecraft:ghast".to_owned());
        map.insert("Guardian".to_owned(), "minecraft:guardian".to_owned());
        map.insert("Horse".to_owned(), "minecraft:horse".to_owned());
        map.insert("Husk".to_owned(), "minecraft:husk".to_owned());
        map.insert("Llama".to_owned(), "minecraft:llama".to_owned());
        map.insert("LavaSlime".to_owned(), "minecraft:magma_cube".to_owned());
        map.insert("MushroomCow".to_owned(), "minecraft:mooshroom".to_owned());
        map.insert("Mule".to_owned(), "minecraft:mule".to_owned());
        map.insert("Ozelot".to_owned(), "minecraft:ocelot".to_owned());
        map.insert("Parrot".to_owned(), "minecraft:parrot".to_owned());
        map.insert("Pig".to_owned(), "minecraft:pig".to_owned());
        map.insert("PolarBear".to_owned(), "minecraft:polar_bear".to_owned());
        map.insert("Rabbit".to_owned(), "minecraft:rabbit".to_owned());
        map.insert("Sheep".to_owned(), "minecraft:sheep".to_owned());
        map.insert("Shulker".to_owned(), "minecraft:shulker".to_owned());
        map.insert("Silverfish".to_owned(), "minecraft:silverfish".to_owned());
        map.insert("SkeletonHorse".to_owned(), "minecraft:skeleton_horse".to_owned());
        map.insert("Skeleton".to_owned(), "minecraft:skeleton".to_owned());
        map.insert("Slime".to_owned(), "minecraft:slime".to_owned());
        map.insert("Spider".to_owned(), "minecraft:spider".to_owned());
        map.insert("Squid".to_owned(), "minecraft:squid".to_owned());
        map.insert("Stray".to_owned(), "minecraft:stray".to_owned());
        map.insert("Vex".to_owned(), "minecraft:vex".to_owned());
        map.insert("Villager".to_owned(), "minecraft:villager".to_owned());
        map.insert("VindicationIllager".to_owned(), "minecraft:vindication_illager".to_owned());
        map.insert("Witch".to_owned(), "minecraft:witch".to_owned());
        map.insert("WitherSkeleton".to_owned(), "minecraft:wither_skeleton".to_owned());
        map.insert("Wolf".to_owned(), "minecraft:wolf".to_owned());
        map.insert("ZombieHorse".to_owned(), "minecraft:zombie_horse".to_owned());
        map.insert("PigZombie".to_owned(), "minecraft:zombie_pigman".to_owned());
        map.insert("ZombieVillager".to_owned(), "minecraft:zombie_villager".to_owned());
        map.insert("Zombie".to_owned(), "minecraft:zombie".to_owned());
        map
    })
}

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    // V0
    types.tile_entity.borrow_mut().add_walker_for_id(DataVersion::new(VERSION, 0), "minecraft:trapped_chest", DataWalkerMapListPaths::new(types.item_stack, "Items"));

    // V1
    types.chunk.borrow_mut().add_structure_converter(DataVersion::new(VERSION, 1), ConverterFlattenChunk::<T>::new());

    let block_name_type = types.block_name;
    let block_state_type = types.block_state;
    let entity_type = types.entity;
    let tile_entity_type = types.tile_entity;
    types.chunk.borrow_mut().add_structure_walker(DataVersion::new(VERSION, 1), data_walker::<T, _>(move |data: &mut T::Map, from_version, to_version| {
        if let Some(level) = data.get_map_mut("Level") {
            convert_map_list_in_map::<_, T>(entity_type, level, "Entities", from_version, to_version);
            convert_map_list_in_map::<_, T>(tile_entity_type, level, "TileEntities", from_version, to_version);

            if let Some(tile_ticks) = level.get_list_mut("TileTicks") {
                for tile_tick in tile_ticks.iter_mut() {
                    if let Some(tile_tick) = tile_tick.as_map_mut() {
                        convert_object_in_map::<_, T>(block_name_type, tile_tick, "i", from_version, to_version);
                    }
                }
            }

            if let Some(sections) = level.get_list_mut("Sections") {
                for section in sections.iter_mut() {
                    if let Some(section) = section.as_map_mut() {
                        convert_map_list_in_map::<_, T>(block_state_type, section, "Palette", from_version, to_version);
                    }
                }
            }
        }
    }));

    // V2
    types.tile_entity.borrow_mut().add_converter_for_id("minecraft:piston", DataVersion::new(VERSION, 2), data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        let block_id = data.remove("blockId").and_then(|o| o.as_i64()).unwrap_or(0) as u16;
        let block_data = data.remove("blockData").and_then(|o| o.as_i64()).unwrap_or(0) as u8 & 15;
        data.set("blockState", T::Object::create_map(block_flattening_v1450::get_nbt_for_id::<T>((block_id << 4) | block_data as u16)));
    }));

    types.tile_entity.borrow_mut().add_walker_for_id(DataVersion::new(VERSION, 2), "minecraft:piston", DataWalkerMapTypePaths::new(types.block_state, "blockState"));

    // V3
    register_entity_flatteners::<T>(types);
    types.item_stack.borrow_mut().add_converter_for_id("minecraft:filled_map", DataVersion::new(VERSION, 3), data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        if data.get_map("tag").is_none() {
            data.set("tag", T::Object::create_map(T::Map::create_empty()));
        }
        let tag = data.get_map_mut("tag").unwrap();

        if tag.get_i64("map").is_none() { // This if is from CB. as usual, no documentation from CB. I'm guessing it just wants to avoid possibly overwriting it. seems fine.
            tag.rename_key("Damage", "map");
        }
    }));

    types.entity.borrow_mut().add_walker_for_id(DataVersion::new(VERSION, 3), "minecraft:potion", DataWalkerMapTypePaths::new(types.item_stack, "Potion"));
    types.entity.borrow_mut().add_walker_for_id(DataVersion::new(VERSION, 3), "minecraft:arrow", DataWalkerMapTypePaths::new(types.block_state, "inBlockState"));
    types.entity.borrow_mut().add_walker_for_id(DataVersion::new(VERSION, 3), "minecraft:enderman", DataWalkerMapListPaths::new_multi(types.item_stack, vec!["ArmorItems".to_owned(), "HandItems".to_owned()]));
    types.entity.borrow_mut().add_walker_for_id(DataVersion::new(VERSION, 3), "minecraft:enderman", DataWalkerMapTypePaths::new(types.block_state, "carriedBlockState"));
    types.entity.borrow_mut().add_walker_for_id(DataVersion::new(VERSION, 3), "minecraft:falling_block", DataWalkerMapTypePaths::new(types.block_state, "BlockState"));
    types.entity.borrow_mut().add_walker_for_id(DataVersion::new(VERSION, 3), "minecraft:falling_block", DataWalkerMapTypePaths::new(types.tile_entity, "TileEntityData"));
    types.entity.borrow_mut().add_walker_for_id(DataVersion::new(VERSION, 3), "minecraft:falling_block", DataWalkerMapTypePaths::new(types.tile_entity, "TileEntityData"));
    types.entity.borrow_mut().add_walker_for_id(DataVersion::new(VERSION, 3), "minecraft:spectral_arrow", DataWalkerMapTypePaths::new(types.block_state, "inBlockState"));
    types.entity.borrow_mut().add_walker_for_id(DataVersion::new(VERSION, 3), "minecraft:chest_minecart", DataWalkerMapTypePaths::new(types.block_state, "DisplayState"));
    types.entity.borrow_mut().add_walker_for_id(DataVersion::new(VERSION, 3), "minecraft:chest_minecart", DataWalkerMapListPaths::new(types.item_stack, "Items"));
    types.entity.borrow_mut().add_walker_for_id(DataVersion::new(VERSION, 3), "minecraft:commandblock_minecart", DataWalkerMapTypePaths::new(types.block_state, "DisplayState"));
    types.entity.borrow_mut().add_walker_for_id(DataVersion::new(VERSION, 3), "minecraft:furnace_minecart", DataWalkerMapTypePaths::new(types.block_state, "DisplayState"));
    types.entity.borrow_mut().add_walker_for_id(DataVersion::new(VERSION, 3), "minecraft:hopper_minecart", DataWalkerMapTypePaths::new(types.block_state, "DisplayState"));
    types.entity.borrow_mut().add_walker_for_id(DataVersion::new(VERSION, 3), "minecraft:hopper_minecart", DataWalkerMapListPaths::new(types.item_stack, "Items"));
    types.entity.borrow_mut().add_walker_for_id(DataVersion::new(VERSION, 3), "minecraft:minecart", DataWalkerMapTypePaths::new(types.block_state, "DisplayState"));
    types.entity.borrow_mut().add_walker_for_id(DataVersion::new(VERSION, 3), "minecraft:spawner_minecart", DataWalkerMapTypePaths::new(types.block_state, "DisplayState"));
    let untagged_spawner_type = types.untagged_spawner;
    types.entity.borrow_mut().add_walker_for_id(DataVersion::new(VERSION, 3), "minecraft:spawner_minecart", data_walker::<T, _>(move |data, from_version, to_version| {
        untagged_spawner_type.convert(data, from_version, to_version);
    }));
    types.entity.borrow_mut().add_walker_for_id(DataVersion::new(VERSION, 3), "minecraft:tnt_minecart", DataWalkerMapTypePaths::new(types.block_state, "DisplayState"));

    // V4
    types.block_name.borrow_mut().add_structure_converter(DataVersion::new(VERSION, 4), data_converter_func::<T::Object, _>(|data, _from_version, _to_version| {
        if let Some(id) = data.as_i64() {
            *data = T::Object::create_string(block_flattening_v1450::get_name_for_id(id as u16).to_owned());
        } else if let Some(name) = data.as_string() {
            *data = T::Object::create_string(block_flattening_v1450::get_new_block_name(name).to_owned());
        }
    }));
    types.item_stack.borrow_mut().add_structure_converter(DataVersion::new(VERSION, 4), ConverterFlattenItemStack::<T>::new());

    // V5
    types.item_stack.borrow_mut().add_converter_for_id("minecraft:spawn_egg", DataVersion::new(VERSION, 5), data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        if let Some(tag) = data.get_map("tag") {
            if let Some(entity_tag) = tag.get_map("EntityTag") {
                if let Some(id) = entity_tag.get_string("id") {
                    let new_id = entity_id_to_new_egg_id().get(id).cloned().unwrap_or_else(|| "minecraft:pig_spawn_egg".to_owned());
                    data.set("id", T::Object::create_string(new_id));
                }
            }
        }
    }));
    // Skip the wolf collar color converter.
    // See: https://github.com/PaperMC/DataConverter/blob/b8c345c76f7bd6554666ef856ebd2043775ee47a/src/main/java/ca/spottedleaf/dataconverter/minecraft/versions/V1451.java#L146-L160
    types.tile_entity.borrow_mut().add_converter_for_id("minecraft:banner", DataVersion::new(VERSION, 5), data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        if let Some(base) = data.get_i64("Base") {
            data.set("Base", T::Object::create_int(15i32.wrapping_sub(base as i32)));
        }

        if let Some(patterns) = data.get_list_mut("Patterns") {
            for pattern in patterns.iter_mut() {
                if let Some(pattern) = pattern.as_map_mut() {
                    if let Some(color) = pattern.get_i64("Color") {
                        pattern.set("Color", T::Object::create_int(15i32.wrapping_sub(color as i32)));
                    }
                }
            }
        }
    }));
    types.level.borrow_mut().add_structure_converter(DataVersion::new(VERSION, 5), data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        if data.get_string("generatorName").map(|str| str == "flat") != Some(true) {
            return;
        }

        let generator_options = match data.get_string("generatorOptions") {
            Some(str) => str,
            None => return
        };

        let new_options = if generator_options.is_empty() {
            "minecraft:bedrock,2*minecraft:dirt,minecraft:grass_block;1;village".to_owned()
        } else {
            let mut parts = generator_options.splitn(5, ';');
            let first = parts.next().unwrap();
            let (version, layers) = if let Some(second) = parts.next() {
                (first.parse::<i32>().unwrap_or(0), second)
            } else {
                (0, first)
            };
            if version >= 0 && version <= 3 {
                let mut result: String = layers.split(',').map(|layer| {
                    let mut amount_parts = layer.splitn(2, if version < 3 { 'x' } else { '*' });
                    let first = amount_parts.next().unwrap();
                    let (count, block) = if let Some(second) = amount_parts.next() {
                        (first.parse::<i32>().unwrap_or(0), second)
                    } else {
                        (1, first)
                    };

                    let mut block_parts = block.splitn(3, ':');
                    let first = block_parts.next().unwrap();
                    let block_name = if first == "minecraft" {
                        if let Some(block_name) = block_parts.next() {
                            block_name
                        } else {
                            first
                        }
                    } else {
                        first
                    };
                    let meta = if let Some(meta) = block_parts.next() {
                        meta.parse::<u8>().unwrap_or(0)
                    } else {
                        0
                    };

                    let block_id = if version == 3 {
                        block_name_to_id().get(&format!("minecraft:{}", block_name)).copied().unwrap_or(0) as u16
                    } else {
                        block_name.parse::<u16>().unwrap_or(0)
                    };

                    let new_block_name = block_flattening_v1450::get_state_for_id_raw((block_id << 4) | meta as u16)
                        .map_or_else(|| "minecraft:air", |state| state.name);
                    if count == 1 {
                        new_block_name.to_owned()
                    } else {
                        format!("{}*{}", count, new_block_name)
                    }
                }).intersperse(",".to_owned()).collect();

                while let Some(part) = parts.next() {
                    result.push(';');
                    result.push_str(part);
                }

                result
            } else {
                "minecraft:bedrock,2*minecraft:dirt,minecraft:grass_block;1;village".to_owned()
            }
        };

        data.set("generatorOptions", T::Object::create_string(new_options));
    }));

    // V6
    types.stats.borrow_mut().add_structure_converter(DataVersion::new(VERSION, 6), data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        let mut stats = T::Map::create_empty();

        for stat_key in data.keys() {
            let value = match data.get_i64(stat_key) {
                Some(value) => value as i32,
                None => continue
            };

            if skip_stats().contains_key(stat_key) {
                continue;
            }

            let (stat_type, new_stat_key) = if let Some(stat_key) = custom_stats().get(stat_key) {
                ("minecraft:custom".to_owned(), stat_key.clone())
            } else {
                let split_index = match stat_key.match_indices('.').nth(1) {
                    Some((index, _)) => index,
                    None => continue
                };

                let (key, item) = stat_key.split_at(split_index);
                let item = item.strip_prefix('.').unwrap().replace('.', ":");

                if key == "stat.mineBlock" {
                    ("minecraft:mined".to_owned(), block_flattening_v1450::get_new_block_name(&item).to_owned())
                } else if let Some(stat_key) = item_stats().get(stat_key) {
                    (stat_key.clone(), flatten_item_stack_v1451::flatten_item(&item, 0).cloned().unwrap_or(item))
                } else if let Some(stat_key) = entity_stats().get(stat_key) {
                    (stat_key.clone(), entity_map().get(&item).cloned().unwrap_or(item))
                } else {
                    continue
                }
            };

            if stats.get_map(&stat_type).is_none() {
                stats.set(stat_type.clone(), T::Object::create_map(T::Map::create_empty()));
            }
            let stat_type_map = stats.get_map_mut(&stat_type).unwrap();
            stat_type_map.set(new_stat_key, T::Object::create_int(value));
        }

        data.clear();
        data.set("stats", T::Object::create_map(stats));
    }));
    types.tile_entity.borrow_mut().add_converter_for_id("minecraft:jukebox", DataVersion::new(VERSION, 6), data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        let record = data.get_i64("Record").unwrap_or(0) as i32;
        if record <= 0 {
            return;
        }
        data.remove("Record");

        if let Some(new_item_id) = item_name_v102::get_name_from_id(record).and_then(|str| flatten_item_stack_v1451::flatten_item(str, 0)) {
            let mut record_item = T::Map::create_empty();
            record_item.set("id", T::Object::create_string(new_item_id.clone()));
            record_item.set("Count", T::Object::create_byte(1));
            data.set("RecordItem", T::Object::create_map(record_item));
        }
    }));

    let block_name_type = types.block_name;
    let entity_name_type = types.entity_name;
    let item_name_type = types.item_name;
    types.stats.borrow_mut().add_structure_walker(DataVersion::new(VERSION, 6), data_walker::<T, _>(move |data: &mut T::Map, from_version, to_version| {
        if let Some(stats) = data.get_map_mut("stats") {
            rename_keys_in_map::<T>(block_name_type, stats, "minecraft:mined", from_version, to_version);

            rename_keys_in_map::<T>(item_name_type, stats, "minecraft:crafted", from_version, to_version);
            rename_keys_in_map::<T>(item_name_type, stats, "minecraft:used", from_version, to_version);
            rename_keys_in_map::<T>(item_name_type, stats, "minecraft:broken", from_version, to_version);
            rename_keys_in_map::<T>(item_name_type, stats, "minecraft:picked_up", from_version, to_version);
            rename_keys_in_map::<T>(item_name_type, stats, "minecraft:dropped", from_version, to_version);

            rename_keys_in_map::<T>(entity_name_type, stats, "minecraft:killed", from_version, to_version);
            rename_keys_in_map::<T>(entity_name_type, stats, "minecraft:killed_by", from_version, to_version);
        }
    }));

    struct ObjectiveHook<T: Types + ?Sized> {
        _phantom: PhantomData<T>,
    }
    impl<T: Types + ?Sized> DataHook<T::Map> for ObjectiveHook<T> {
        fn pre_hook(&self, data: &mut T::Map, _from_version: DataVersion, _to_version: DataVersion) {
            // unpack
            if let Some(criteria_name) = data.get_string("CriteriaName") {
                let (typ, id) = Option::unwrap_or_else(try {
                    let (typ, id) = criteria_name.split_at(criteria_name.find(':')?);
                    let id = id.strip_prefix(':').unwrap();
                    let typ = ResourceLocation::parse_with_separator(typ, '.').ok()?.to_string();
                    let id = ResourceLocation::parse_with_separator(id, '.').ok()?.to_string();
                    (typ, id)
                }, || ("_special".to_owned(), criteria_name.to_owned()));

                let mut criteria_type = T::Map::create_empty();
                criteria_type.set("type", T::Object::create_string(typ));
                criteria_type.set("id", T::Object::create_string(id));
                data.set("CriteriaType", T::Object::create_map(criteria_type));
            }
        }

        fn post_hook(&self, data: &mut T::Map, _from_version: DataVersion, _to_version: DataVersion) {
            // repack
            if let Some(criteria_type) = data.get_map("CriteriaType") {
                if let (Some(typ), Some(id)) = (criteria_type.get_string("type"), criteria_type.get_string("id")) {
                    let new_name = if typ == "_special" {
                        id.to_owned()
                    } else {
                        let type_part = typ.parse::<ResourceLocation>().map_or_else(|_| typ.to_owned(), |loc| format!("{}.{}", loc.namespace, loc.path));
                        let id_part = id.parse::<ResourceLocation>().map_or_else(|_| id.to_owned(), |loc| format!("{}.{}", loc.namespace, loc.path));
                        format!("{}:{}", type_part, id_part)
                    };

                    data.remove("CriteriaType");
                    data.set("CriteriaName", T::Object::create_string(new_name));
                }
            }
        }
    }
    types.objective.borrow_mut().add_structure_hook(DataVersion::new(VERSION, 6), ObjectiveHook { _phantom: PhantomData::<T> });

    let block_name_type = types.block_name;
    let entity_name_type = types.entity_name;
    let item_name_type = types.item_name;
    types.objective.borrow_mut().add_structure_walker(DataVersion::new(VERSION, 6), data_walker::<T, _>(move |data: &mut T::Map, from_version, to_version| {
        if let Some(criteria_type) = data.get_map_mut("CriteriaType") {
            match criteria_type.get_string("type") {
                Some("minecraft:mined") => {
                    convert_object_in_map::<_, T>(block_name_type, criteria_type, "id", from_version, to_version);
                }
                Some("minecraft:crafted") |
                Some("minecraft:used") |
                Some("minecraft:broken") |
                Some("minecraft:picked_up") |
                Some("minecraft:dropped") => {
                    convert_object_in_map::<_, T>(item_name_type, criteria_type, "id", from_version, to_version);
                }
                Some("minecraft:killed") |
                Some("minecraft:killed_by") => {
                    convert_object_in_map::<_, T>(entity_name_type, criteria_type, "id", from_version, to_version);
                }
                _ => {}
            }
        }
    }));

    // V7
    types.structure_feature.borrow_mut().add_structure_converter(DataVersion::new(VERSION, 7), data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        fn convert_to_block_state<T: Types + ?Sized>(data: &mut T::Map, path: &str) {
            if let Some(id) = data.get_i64(path) {
                data.set(path, T::Object::create_map(block_flattening_v1450::get_nbt_for_id::<T>((id as u16) << 4)));
            }
        }

        if let Some(children) = data.get_list_mut("Children") {
            for child in children.iter_mut() {
                if let Some(child) = child.as_map_mut() {
                    match child.get_string("id") {
                        Some("ViF") => {
                            convert_to_block_state::<T>(child, "CA");
                            convert_to_block_state::<T>(child, "CB");
                        }
                        Some("ViDF") => {
                            convert_to_block_state::<T>(child, "CA");
                            convert_to_block_state::<T>(child, "CB");
                            convert_to_block_state::<T>(child, "CC");
                            convert_to_block_state::<T>(child, "CD");
                        }
                        _ => {}
                    }
                }
            }
        }
    }));

    // convert villagers to trade with pumpkins and not the carved pumpkin
    types.entity.borrow_mut().add_converter_for_id("minecraft:villager", DataVersion::new(VERSION, 7), data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        fn convert_pumpkin<T: Types + ?Sized>(data: &mut T::Map, path: &str) {
            if let Some(item) = data.get_map_mut(path) {
                if item.get_string("id").map(|str| str == "minecraft:carved_pumpkin") == Some(true) {
                    item.set("id", T::Object::create_string("minecraft:pumpkin".to_owned()));
                }
            }
        }

        if let Some(offers) = data.get_map_mut("Offers") {
            if let Some(recipes) = offers.get_list_mut("Recipes") {
                for recipe in recipes.iter_mut() {
                    if let Some(recipe) = recipe.as_map_mut() {
                        convert_pumpkin::<T>(recipe, "buy");
                        convert_pumpkin::<T>(recipe, "buyB");
                        convert_pumpkin::<T>(recipe, "sell");
                    }
                }
            }
        }
    }));

    let block_state_type = types.block_state;
    types.structure_feature.borrow_mut().add_structure_walker(DataVersion::new(VERSION, 7), data_walker::<T, _>(move |data: &mut T::Map, from_version, to_version| {
        if let Some(children) = data.get_list_mut("Children") {
            for child in children.iter_mut() {
                if let Some(child) = child.as_map_mut() {
                    convert_map_in_map::<_, T>(block_state_type, child, "CA", from_version, to_version);
                    convert_map_in_map::<_, T>(block_state_type, child, "CB", from_version, to_version);
                    convert_map_in_map::<_, T>(block_state_type, child, "CC", from_version, to_version);
                    convert_map_in_map::<_, T>(block_state_type, child, "CD", from_version, to_version);
                }
            }
        }
    }));
}

fn register_entity_flatteners<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    types.entity.borrow_mut().add_converter_for_id("minecraft:falling_block", DataVersion::new(VERSION, 3), data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        let block_id = if data.has_key("Block") {
            if let Some(id) = data.get_i64("Block") {
                id as u16
            } else if let Some(id) = data.get_string("Block") {
                block_name_to_id().get(id).copied().unwrap_or(0) as u16
            } else {
                0
            }
        } else {
            if let Some(id) = data.get_i64("TileID") {
                id as u16
            } else if let Some(id) = data.get_i64("Tile") {
                id as u8 as u16
            } else {
                0
            }
        };

        let block_data = data.get_i64("Data").unwrap_or(0) as u8 & 15;

        data.remove("Block"); // from type update
        data.remove("Data");
        data.remove("TileID");
        data.remove("Tile");

        // key is from type update
        data.set("BlockState", T::Object::create_map(block_flattening_v1450::get_nbt_for_id::<T>((block_id << 4) | block_data as u16)));
    }));
    convert_entity_state(types, "minecraft:enderman", "carried", "carriedData", "carriedBlockState");
    convert_entity_state(types, "minecraft:arrow", "inTile", "inData", "inBlockState");
    convert_entity_state(types, "minecraft:spectral_arrow", "inTile", "inData", "inBlockState");
    remove_in_tile(types, "minecraft:egg");
    remove_in_tile(types, "minecraft:ender_pearl");
    remove_in_tile(types, "minecraft:fireball");
    remove_in_tile(types, "minecraft:potion");
    remove_in_tile(types, "minecraft:small_fireball");
    remove_in_tile(types, "minecraft:snowball");
    remove_in_tile(types, "minecraft:wither_skull");
    remove_in_tile(types, "minecraft:xp_bottle");
    convert_entity_state(types, "minecraft:commandblock_minecart", "DisplayTile", "DisplayData", "DisplayState");
    convert_entity_state(types, "minecraft:minecart", "DisplayTile", "DisplayData", "DisplayState");
    convert_entity_state(types, "minecraft:chest_minecart", "DisplayTile", "DisplayData", "DisplayState");
    convert_entity_state(types, "minecraft:furnace_minecart", "DisplayTile", "DisplayData", "DisplayState");
    convert_entity_state(types, "minecraft:tnt_minecart", "DisplayTile", "DisplayData", "DisplayState");
    convert_entity_state(types, "minecraft:hopper_minecart", "DisplayTile", "DisplayData", "DisplayState");
    convert_entity_state(types, "minecraft:spawner_minecart", "DisplayTile", "DisplayData", "DisplayState");
}

fn remove_in_tile<T: Types + ?Sized>(types: &MinecraftTypesMut<T>, entity_id: impl Into<String>) {
    types.entity.borrow_mut().add_converter_for_id(entity_id, DataVersion::new(VERSION, 3), data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        data.remove("inTile");
    }));
}

fn convert_entity_state<'a, T: Types + ?Sized>(types: &MinecraftTypesMut<'a, T>, entity_id: impl Into<String>, id_path: &'a str, data_path: &'a str, output_path: impl Into<String> + Clone + 'a) {
    types.entity.borrow_mut().add_converter_for_id(entity_id, DataVersion::new(VERSION, 3), data_converter_func::<T::Map, _>(move |data, _from_version, _to_version| {
        let block_id = if let Some(id) = data.get_i64(id_path) {
            id as u16
        } else if let Some(id) = data.get_string(id_path) {
            block_name_to_id().get(id).cloned().unwrap_or(0) as u16
        } else {
            0
        };

        let block_data = data.get_i64(data_path).unwrap_or(0) as u8 & 15;

        data.remove(id_path);
        data.remove(data_path);
        data.set(output_path.clone(), T::Object::create_map(block_flattening_v1450::get_nbt_for_id::<T>((block_id << 4) | block_data as u16)));
    }));
}
