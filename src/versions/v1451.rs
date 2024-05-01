use crate::helpers::flatten_chunk_v1451::ConverterFlattenChunk;
use crate::helpers::flatten_item_stack_v1451::ConverterFlattenItemStack;
use crate::helpers::flatten_stats_v1451::{ObjectiveConverter, StatsConverter};
use crate::helpers::mc_namespace_map::McNamespaceMap;
use crate::helpers::rename::rename_keys_in_map;
use crate::helpers::resource_location::ResourceLocation;
use crate::helpers::{block_flattening_v1450, flatten_item_stack_v1451, item_name_v102};
use crate::versions::v100;
use crate::{static_string_mc_map, types};
use java_string::{format_java, JavaStr, JavaString};
use std::sync::OnceLock;
use valence_nbt::{compound, jcompound};
use world_transmuter_engine::{
    convert_map_in_map, convert_map_list_in_map, convert_object_in_map, map_data_converter_func,
    map_data_walker, AbstractMapDataType, DataVersion, DataWalkerMapListPaths,
    DataWalkerMapTypePaths, JCompound, JList, JValue, MapDataConverterFunc, MapDataHook,
};

const VERSION: u32 = 1451;

static BLOCK_NAME_TO_ID: OnceLock<McNamespaceMap<u8>> = OnceLock::new();

fn block_name_to_id() -> &'static McNamespaceMap<'static, u8> {
    BLOCK_NAME_TO_ID.get_or_init(|| {
        let mut map = McNamespaceMap::new();
        map.insert_mc("air", 0);
        map.insert_mc("stone", 1);
        map.insert_mc("grass", 2);
        map.insert_mc("dirt", 3);
        map.insert_mc("cobblestone", 4);
        map.insert_mc("planks", 5);
        map.insert_mc("sapling", 6);
        map.insert_mc("bedrock", 7);
        map.insert_mc("flowing_water", 8);
        map.insert_mc("water", 9);
        map.insert_mc("flowing_lava", 10);
        map.insert_mc("lava", 11);
        map.insert_mc("sand", 12);
        map.insert_mc("gravel", 13);
        map.insert_mc("gold_ore", 14);
        map.insert_mc("iron_ore", 15);
        map.insert_mc("coal_ore", 16);
        map.insert_mc("log", 17);
        map.insert_mc("leaves", 18);
        map.insert_mc("sponge", 19);
        map.insert_mc("glass", 20);
        map.insert_mc("lapis_ore", 21);
        map.insert_mc("lapis_block", 22);
        map.insert_mc("dispenser", 23);
        map.insert_mc("sandstone", 24);
        map.insert_mc("noteblock", 25);
        map.insert_mc("bed", 26);
        map.insert_mc("golden_rail", 27);
        map.insert_mc("detector_rail", 28);
        map.insert_mc("sticky_piston", 29);
        map.insert_mc("web", 30);
        map.insert_mc("tallgrass", 31);
        map.insert_mc("deadbush", 32);
        map.insert_mc("piston", 33);
        map.insert_mc("piston_head", 34);
        map.insert_mc("wool", 35);
        map.insert_mc("piston_extension", 36);
        map.insert_mc("yellow_flower", 37);
        map.insert_mc("red_flower", 38);
        map.insert_mc("brown_mushroom", 39);
        map.insert_mc("red_mushroom", 40);
        map.insert_mc("gold_block", 41);
        map.insert_mc("iron_block", 42);
        map.insert_mc("double_stone_slab", 43);
        map.insert_mc("stone_slab", 44);
        map.insert_mc("brick_block", 45);
        map.insert_mc("tnt", 46);
        map.insert_mc("bookshelf", 47);
        map.insert_mc("mossy_cobblestone", 48);
        map.insert_mc("obsidian", 49);
        map.insert_mc("torch", 50);
        map.insert_mc("fire", 51);
        map.insert_mc("mob_spawner", 52);
        map.insert_mc("oak_stairs", 53);
        map.insert_mc("chest", 54);
        map.insert_mc("redstone_wire", 55);
        map.insert_mc("diamond_ore", 56);
        map.insert_mc("diamond_block", 57);
        map.insert_mc("crafting_table", 58);
        map.insert_mc("wheat", 59);
        map.insert_mc("farmland", 60);
        map.insert_mc("furnace", 61);
        map.insert_mc("lit_furnace", 62);
        map.insert_mc("standing_sign", 63);
        map.insert_mc("wooden_door", 64);
        map.insert_mc("ladder", 65);
        map.insert_mc("rail", 66);
        map.insert_mc("stone_stairs", 67);
        map.insert_mc("wall_sign", 68);
        map.insert_mc("lever", 69);
        map.insert_mc("stone_pressure_plate", 70);
        map.insert_mc("iron_door", 71);
        map.insert_mc("wooden_pressure_plate", 72);
        map.insert_mc("redstone_ore", 73);
        map.insert_mc("lit_redstone_ore", 74);
        map.insert_mc("unlit_redstone_torch", 75);
        map.insert_mc("redstone_torch", 76);
        map.insert_mc("stone_button", 77);
        map.insert_mc("snow_layer", 78);
        map.insert_mc("ice", 79);
        map.insert_mc("snow", 80);
        map.insert_mc("cactus", 81);
        map.insert_mc("clay", 82);
        map.insert_mc("reeds", 83);
        map.insert_mc("jukebox", 84);
        map.insert_mc("fence", 85);
        map.insert_mc("pumpkin", 86);
        map.insert_mc("netherrack", 87);
        map.insert_mc("soul_sand", 88);
        map.insert_mc("glowstone", 89);
        map.insert_mc("portal", 90);
        map.insert_mc("lit_pumpkin", 91);
        map.insert_mc("cake", 92);
        map.insert_mc("unpowered_repeater", 93);
        map.insert_mc("powered_repeater", 94);
        map.insert_mc("stained_glass", 95);
        map.insert_mc("trapdoor", 96);
        map.insert_mc("monster_egg", 97);
        map.insert_mc("stonebrick", 98);
        map.insert_mc("brown_mushroom_block", 99);
        map.insert_mc("red_mushroom_block", 100);
        map.insert_mc("iron_bars", 101);
        map.insert_mc("glass_pane", 102);
        map.insert_mc("melon_block", 103);
        map.insert_mc("pumpkin_stem", 104);
        map.insert_mc("melon_stem", 105);
        map.insert_mc("vine", 106);
        map.insert_mc("fence_gate", 107);
        map.insert_mc("brick_stairs", 108);
        map.insert_mc("stone_brick_stairs", 109);
        map.insert_mc("mycelium", 110);
        map.insert_mc("waterlily", 111);
        map.insert_mc("nether_brick", 112);
        map.insert_mc("nether_brick_fence", 113);
        map.insert_mc("nether_brick_stairs", 114);
        map.insert_mc("nether_wart", 115);
        map.insert_mc("enchanting_table", 116);
        map.insert_mc("brewing_stand", 117);
        map.insert_mc("cauldron", 118);
        map.insert_mc("end_portal", 119);
        map.insert_mc("end_portal_frame", 120);
        map.insert_mc("end_stone", 121);
        map.insert_mc("dragon_egg", 122);
        map.insert_mc("redstone_lamp", 123);
        map.insert_mc("lit_redstone_lamp", 124);
        map.insert_mc("double_wooden_slab", 125);
        map.insert_mc("wooden_slab", 126);
        map.insert_mc("cocoa", 127);
        map.insert_mc("sandstone_stairs", 128);
        map.insert_mc("emerald_ore", 129);
        map.insert_mc("ender_chest", 130);
        map.insert_mc("tripwire_hook", 131);
        map.insert_mc("tripwire", 132);
        map.insert_mc("emerald_block", 133);
        map.insert_mc("spruce_stairs", 134);
        map.insert_mc("birch_stairs", 135);
        map.insert_mc("jungle_stairs", 136);
        map.insert_mc("command_block", 137);
        map.insert_mc("beacon", 138);
        map.insert_mc("cobblestone_wall", 139);
        map.insert_mc("flower_pot", 140);
        map.insert_mc("carrots", 141);
        map.insert_mc("potatoes", 142);
        map.insert_mc("wooden_button", 143);
        map.insert_mc("skull", 144);
        map.insert_mc("anvil", 145);
        map.insert_mc("trapped_chest", 146);
        map.insert_mc("light_weighted_pressure_plate", 147);
        map.insert_mc("heavy_weighted_pressure_plate", 148);
        map.insert_mc("unpowered_comparator", 149);
        map.insert_mc("powered_comparator", 150);
        map.insert_mc("daylight_detector", 151);
        map.insert_mc("redstone_block", 152);
        map.insert_mc("quartz_ore", 153);
        map.insert_mc("hopper", 154);
        map.insert_mc("quartz_block", 155);
        map.insert_mc("quartz_stairs", 156);
        map.insert_mc("activator_rail", 157);
        map.insert_mc("dropper", 158);
        map.insert_mc("stained_hardened_clay", 159);
        map.insert_mc("stained_glass_pane", 160);
        map.insert_mc("leaves2", 161);
        map.insert_mc("log2", 162);
        map.insert_mc("acacia_stairs", 163);
        map.insert_mc("dark_oak_stairs", 164);
        map.insert_mc("slime", 165);
        map.insert_mc("barrier", 166);
        map.insert_mc("iron_trapdoor", 167);
        map.insert_mc("prismarine", 168);
        map.insert_mc("sea_lantern", 169);
        map.insert_mc("hay_block", 170);
        map.insert_mc("carpet", 171);
        map.insert_mc("hardened_clay", 172);
        map.insert_mc("coal_block", 173);
        map.insert_mc("packed_ice", 174);
        map.insert_mc("double_plant", 175);
        map.insert_mc("standing_banner", 176);
        map.insert_mc("wall_banner", 177);
        map.insert_mc("daylight_detector_inverted", 178);
        map.insert_mc("red_sandstone", 179);
        map.insert_mc("red_sandstone_stairs", 180);
        map.insert_mc("double_stone_slab2", 181);
        map.insert_mc("stone_slab2", 182);
        map.insert_mc("spruce_fence_gate", 183);
        map.insert_mc("birch_fence_gate", 184);
        map.insert_mc("jungle_fence_gate", 185);
        map.insert_mc("dark_oak_fence_gate", 186);
        map.insert_mc("acacia_fence_gate", 187);
        map.insert_mc("spruce_fence", 188);
        map.insert_mc("birch_fence", 189);
        map.insert_mc("jungle_fence", 190);
        map.insert_mc("dark_oak_fence", 191);
        map.insert_mc("acacia_fence", 192);
        map.insert_mc("spruce_door", 193);
        map.insert_mc("birch_door", 194);
        map.insert_mc("jungle_door", 195);
        map.insert_mc("acacia_door", 196);
        map.insert_mc("dark_oak_door", 197);
        map.insert_mc("end_rod", 198);
        map.insert_mc("chorus_plant", 199);
        map.insert_mc("chorus_flower", 200);
        map.insert_mc("purpur_block", 201);
        map.insert_mc("purpur_pillar", 202);
        map.insert_mc("purpur_stairs", 203);
        map.insert_mc("purpur_double_slab", 204);
        map.insert_mc("purpur_slab", 205);
        map.insert_mc("end_bricks", 206);
        map.insert_mc("beetroots", 207);
        map.insert_mc("grass_path", 208);
        map.insert_mc("end_gateway", 209);
        map.insert_mc("repeating_command_block", 210);
        map.insert_mc("chain_command_block", 211);
        map.insert_mc("frosted_ice", 212);
        map.insert_mc("magma", 213);
        map.insert_mc("nether_wart_block", 214);
        map.insert_mc("red_nether_brick", 215);
        map.insert_mc("bone_block", 216);
        map.insert_mc("structure_void", 217);
        map.insert_mc("observer", 218);
        map.insert_mc("white_shulker_box", 219);
        map.insert_mc("orange_shulker_box", 220);
        map.insert_mc("magenta_shulker_box", 221);
        map.insert_mc("light_blue_shulker_box", 222);
        map.insert_mc("yellow_shulker_box", 223);
        map.insert_mc("lime_shulker_box", 224);
        map.insert_mc("pink_shulker_box", 225);
        map.insert_mc("gray_shulker_box", 226);
        map.insert_mc("silver_shulker_box", 227);
        map.insert_mc("cyan_shulker_box", 228);
        map.insert_mc("purple_shulker_box", 229);
        map.insert_mc("blue_shulker_box", 230);
        map.insert_mc("brown_shulker_box", 231);
        map.insert_mc("green_shulker_box", 232);
        map.insert_mc("red_shulker_box", 233);
        map.insert_mc("black_shulker_box", 234);
        map.insert_mc("white_glazed_terracotta", 235);
        map.insert_mc("orange_glazed_terracotta", 236);
        map.insert_mc("magenta_glazed_terracotta", 237);
        map.insert_mc("light_blue_glazed_terracotta", 238);
        map.insert_mc("yellow_glazed_terracotta", 239);
        map.insert_mc("lime_glazed_terracotta", 240);
        map.insert_mc("pink_glazed_terracotta", 241);
        map.insert_mc("gray_glazed_terracotta", 242);
        map.insert_mc("silver_glazed_terracotta", 243);
        map.insert_mc("cyan_glazed_terracotta", 244);
        map.insert_mc("purple_glazed_terracotta", 245);
        map.insert_mc("blue_glazed_terracotta", 246);
        map.insert_mc("brown_glazed_terracotta", 247);
        map.insert_mc("green_glazed_terracotta", 248);
        map.insert_mc("red_glazed_terracotta", 249);
        map.insert_mc("black_glazed_terracotta", 250);
        map.insert_mc("concrete", 251);
        map.insert_mc("concrete_powder", 252);
        map.insert_mc("structure_block", 255);
        map
    })
}

static_string_mc_map! {
    ENTITY_ID_TO_NEW_EGG_ID, entity_id_to_new_egg_id, {
        "bat" => "minecraft:bat_spawn_egg",
        "blaze" => "minecraft:blaze_spawn_egg",
        "cave_spider" => "minecraft:cave_spider_spawn_egg",
        "chicken" => "minecraft:chicken_spawn_egg",
        "cow" => "minecraft:cow_spawn_egg",
        "creeper" => "minecraft:creeper_spawn_egg",
        "donkey" => "minecraft:donkey_spawn_egg",
        "elder_guardian" => "minecraft:elder_guardian_spawn_egg",
        "enderman" => "minecraft:enderman_spawn_egg",
        "endermite" => "minecraft:endermite_spawn_egg",
        "evocation_illager" => "minecraft:evocation_illager_spawn_egg",
        "ghast" => "minecraft:ghast_spawn_egg",
        "guardian" => "minecraft:guardian_spawn_egg",
        "horse" => "minecraft:horse_spawn_egg",
        "husk" => "minecraft:husk_spawn_egg",
        "llama" => "minecraft:llama_spawn_egg",
        "magma_cube" => "minecraft:magma_cube_spawn_egg",
        "mooshroom" => "minecraft:mooshroom_spawn_egg",
        "mule" => "minecraft:mule_spawn_egg",
        "ocelot" => "minecraft:ocelot_spawn_egg",
        "pufferfish" => "minecraft:pufferfish_spawn_egg",
        "parrot" => "minecraft:parrot_spawn_egg",
        "pig" => "minecraft:pig_spawn_egg",
        "polar_bear" => "minecraft:polar_bear_spawn_egg",
        "rabbit" => "minecraft:rabbit_spawn_egg",
        "sheep" => "minecraft:sheep_spawn_egg",
        "shulker" => "minecraft:shulker_spawn_egg",
        "silverfish" => "minecraft:silverfish_spawn_egg",
        "skeleton" => "minecraft:skeleton_spawn_egg",
        "skeleton_horse" => "minecraft:skeleton_horse_spawn_egg",
        "slime" => "minecraft:slime_spawn_egg",
        "spider" => "minecraft:spider_spawn_egg",
        "squid" => "minecraft:squid_spawn_egg",
        "stray" => "minecraft:stray_spawn_egg",
        "turtle" => "minecraft:turtle_spawn_egg",
        "vex" => "minecraft:vex_spawn_egg",
        "villager" => "minecraft:villager_spawn_egg",
        "vindication_illager" => "minecraft:vindication_illager_spawn_egg",
        "witch" => "minecraft:witch_spawn_egg",
        "wither_skeleton" => "minecraft:wither_skeleton_spawn_egg",
        "wolf" => "minecraft:wolf_spawn_egg",
        "zombie" => "minecraft:zombie_spawn_egg",
        "zombie_horse" => "minecraft:zombie_horse_spawn_egg",
        "zombie_pigman" => "minecraft:zombie_pigman_spawn_egg",
        "zombie_villager" => "minecraft:zombie_villager_spawn_egg",
    }
}

pub(crate) fn register() {
    // V0
    types::tile_entity_mut().add_walker_for_id(
        DataVersion::new(VERSION, 0),
        "minecraft:trapped_chest",
        DataWalkerMapListPaths::new(types::item_stack_ref(), "Items"),
    );

    // V1
    types::chunk_mut().add_structure_converter(DataVersion::new(VERSION, 1), ConverterFlattenChunk);

    types::chunk_mut().add_structure_walker(
        DataVersion::new(VERSION, 1),
        map_data_walker(move |data, from_version, to_version| {
            let Some(JValue::Compound(level)) = data.get_mut("Level") else {
                return;
            };
            convert_map_list_in_map(
                types::entity_ref(),
                level,
                "Entities",
                from_version,
                to_version,
            );
            convert_map_list_in_map(
                types::tile_entity_ref(),
                level,
                "TileEntities",
                from_version,
                to_version,
            );

            if let Some(JValue::List(JList::Compound(tile_ticks))) = level.get_mut("TileTicks") {
                for tile_tick in tile_ticks {
                    convert_object_in_map(
                        types::block_name_ref(),
                        tile_tick,
                        "i",
                        from_version,
                        to_version,
                    );
                }
            }

            if let Some(JValue::List(JList::Compound(sections))) = level.get_mut("Sections") {
                for section in sections {
                    convert_map_list_in_map(
                        types::block_state_ref(),
                        section,
                        "Palette",
                        from_version,
                        to_version,
                    );
                }
            }
        }),
    );

    // V2
    types::tile_entity_mut().add_converter_for_id(
        "minecraft:piston",
        DataVersion::new(VERSION, 2),
        map_data_converter_func(|data, _from_version, _to_version| {
            let block_id = data.remove("blockId").and_then(|o| o.as_i16()).unwrap_or(0) as u16;
            let block_data = data
                .remove("blockData")
                .and_then(|o| o.as_i8())
                .unwrap_or(0) as u8
                & 15;
            data.insert(
                "blockState",
                block_flattening_v1450::get_nbt_for_id((block_id << 4) | block_data as u16),
            );
        }),
    );

    types::tile_entity_mut().add_walker_for_id(
        DataVersion::new(VERSION, 2),
        "minecraft:piston",
        DataWalkerMapTypePaths::new(types::block_state_ref(), "blockState"),
    );

    // V3
    register_entity_flatteners();
    types::item_stack_mut().add_converter_for_id(
        "minecraft:filled_map",
        DataVersion::new(VERSION, 3),
        map_data_converter_func(|data, _from_version, _to_version| {
            let Some(damage) = data.get("Damage").cloned() else {
                return;
            };
            if !matches!(data.get("tag"), Some(JValue::Compound(_))) {
                data.insert("tag", JCompound::new());
            }
            let Some(JValue::Compound(tag)) = data.get_mut("tag") else {
                unreachable!()
            };

            if tag.get("map").map(|v| v.is_number()) != Some(true) {
                // This if is from CB. as usual, no documentation from CB. I'm guessing it just wants to avoid possibly overwriting it. seems fine.
                tag.insert("map", damage);
            }
        }),
    );

    types::entity_mut().add_walker_for_id(
        DataVersion::new(VERSION, 3),
        "minecraft:potion",
        DataWalkerMapTypePaths::new(types::item_stack_ref(), "Potion"),
    );
    types::entity_mut().add_walker_for_id(
        DataVersion::new(VERSION, 3),
        "minecraft:arrow",
        DataWalkerMapTypePaths::new(types::block_state_ref(), "inBlockState"),
    );
    v100::register_equipment(DataVersion::new(VERSION, 3), "minecraft:enderman");
    types::entity_mut().add_walker_for_id(
        DataVersion::new(VERSION, 3),
        "minecraft:enderman",
        DataWalkerMapTypePaths::new(types::block_state_ref(), "carriedBlockState"),
    );
    types::entity_mut().add_walker_for_id(
        DataVersion::new(VERSION, 3),
        "minecraft:falling_block",
        DataWalkerMapTypePaths::new(types::block_state_ref(), "BlockState"),
    );
    types::entity_mut().add_walker_for_id(
        DataVersion::new(VERSION, 3),
        "minecraft:falling_block",
        DataWalkerMapTypePaths::new(types::tile_entity_ref(), "TileEntityData"),
    );
    types::entity_mut().add_walker_for_id(
        DataVersion::new(VERSION, 3),
        "minecraft:falling_block",
        DataWalkerMapTypePaths::new(types::tile_entity_ref(), "TileEntityData"),
    );
    types::entity_mut().add_walker_for_id(
        DataVersion::new(VERSION, 3),
        "minecraft:spectral_arrow",
        DataWalkerMapTypePaths::new(types::block_state_ref(), "inBlockState"),
    );
    types::entity_mut().add_walker_for_id(
        DataVersion::new(VERSION, 3),
        "minecraft:chest_minecart",
        DataWalkerMapTypePaths::new(types::block_state_ref(), "DisplayState"),
    );
    types::entity_mut().add_walker_for_id(
        DataVersion::new(VERSION, 3),
        "minecraft:chest_minecart",
        DataWalkerMapListPaths::new(types::item_stack_ref(), "Items"),
    );
    types::entity_mut().add_walker_for_id(
        DataVersion::new(VERSION, 3),
        "minecraft:commandblock_minecart",
        DataWalkerMapTypePaths::new(types::block_state_ref(), "DisplayState"),
    );
    types::entity_mut().add_walker_for_id(
        DataVersion::new(VERSION, 3),
        "minecraft:furnace_minecart",
        DataWalkerMapTypePaths::new(types::block_state_ref(), "DisplayState"),
    );
    types::entity_mut().add_walker_for_id(
        DataVersion::new(VERSION, 3),
        "minecraft:hopper_minecart",
        DataWalkerMapTypePaths::new(types::block_state_ref(), "DisplayState"),
    );
    types::entity_mut().add_walker_for_id(
        DataVersion::new(VERSION, 3),
        "minecraft:hopper_minecart",
        DataWalkerMapListPaths::new(types::item_stack_ref(), "Items"),
    );
    types::entity_mut().add_walker_for_id(
        DataVersion::new(VERSION, 3),
        "minecraft:minecart",
        DataWalkerMapTypePaths::new(types::block_state_ref(), "DisplayState"),
    );
    types::entity_mut().add_walker_for_id(
        DataVersion::new(VERSION, 3),
        "minecraft:spawner_minecart",
        DataWalkerMapTypePaths::new(types::block_state_ref(), "DisplayState"),
    );
    types::entity_mut().add_walker_for_id(
        DataVersion::new(VERSION, 3),
        "minecraft:spawner_minecart",
        map_data_walker(move |data, from_version, to_version| {
            types::untagged_spawner().convert(data, from_version, to_version);
        }),
    );
    types::entity_mut().add_walker_for_id(
        DataVersion::new(VERSION, 3),
        "minecraft:tnt_minecart",
        DataWalkerMapTypePaths::new(types::block_state_ref(), "DisplayState"),
    );

    // V4
    // We cannot use a structure converter for block_name to change types as we don't support that.
    // Instead, we add structure converters to all types containing block_name that we aren't already converting elsewhere.
    types::item_stack_mut().add_structure_converter(
        DataVersion::new(VERSION, 4),
        map_data_converter_func(|data, _from_version, _to_version| {
            let Some(JValue::Compound(tag)) = data.get_mut("tag") else {
                return;
            };
            replace_ids(tag, "CanDestroy");
            replace_ids(tag, "CanPlaceOn");
            fn replace_ids(tag: &mut JCompound, key: &str) {
                let Some(JValue::List(value)) = tag.get_mut(key) else {
                    return;
                };
                if let JList::String(strings) = value {
                    for string in strings {
                        *string = block_flattening_v1450::get_new_block_name(string).to_owned();
                    }
                } else {
                    let new_list: Vec<_> = value
                        .iter()
                        .filter_map(|value| {
                            value.as_i16().map(|id| {
                                block_flattening_v1450::get_name_for_id(id as u16).to_owned()
                            })
                        })
                        .collect();
                    *value = JList::String(new_list);
                }
            }
        }),
    );
    types::chunk_mut().add_structure_converter(
        DataVersion::new(VERSION, 4),
        map_data_converter_func(|data, _from_version, _to_version| {
            let Some(JValue::Compound(level)) = data.get_mut("Level") else {
                return;
            };
            if let Some(JValue::List(JList::Compound(tile_ticks))) = level.get_mut("TileTicks") {
                for tile_tick in tile_ticks {
                    let Some(id_val) = tile_tick.get_mut("i") else {
                        continue;
                    };
                    if let Some(id) = id_val.as_i16() {
                        *id_val = JValue::String(
                            block_flattening_v1450::get_name_for_id(id as u16).to_owned(),
                        );
                    } else if let JValue::String(id) = id_val {
                        *id = block_flattening_v1450::get_new_block_name(id).to_owned();
                    }
                }
            }
        }),
    );

    types::item_stack_mut()
        .add_structure_converter(DataVersion::new(VERSION, 4), ConverterFlattenItemStack);

    // V5
    types::item_stack_mut().add_converter_for_id(
        "minecraft:spawn_egg",
        DataVersion::new(VERSION, 5),
        ConverterFlattenSpawnEgg,
    );
    // Skip the wolf collar color converter.
    // See: https://github.com/PaperMC/DataConverter/blob/b8c345c76f7bd6554666ef856ebd2043775ee47a/src/main/java/ca/spottedleaf/dataconverter/minecraft/versions/V1451.java#L146-L160
    types::tile_entity_mut().add_converter_for_id(
        "minecraft:banner",
        DataVersion::new(VERSION, 5),
        map_data_converter_func(|data, _from_version, _to_version| {
            if let Some(base) = data.get("Base").and_then(|v| v.as_i32()) {
                data.insert("Base", 15i32.wrapping_sub(base));
            }

            if let Some(JValue::List(JList::Compound(patterns))) = data.get_mut("Patterns") {
                for pattern in patterns {
                    if let Some(color) = pattern.get("Color").and_then(|v| v.as_i32()) {
                        pattern.insert("Color", 15i32.wrapping_sub(color));
                    }
                }
            }
        }),
    );
    types::level_mut().add_structure_converter(
        DataVersion::new(VERSION, 5),
        map_data_converter_func(|data, _from_version, _to_version| {
            if !matches!(data.get("generatorName"), Some(JValue::String(str)) if str == "flat") {
                return;
            }

            let Some(JValue::String(generator_options)) = data.get_mut("generatorOptions") else {
                return;
            };

            let new_options = if generator_options.is_empty() {
                JavaString::from(
                    "minecraft:bedrock,2*minecraft:dirt,minecraft:grass_block;1;village",
                )
            } else {
                let mut parts = generator_options.splitn(5, ';');
                let first = parts.next().unwrap();
                let (version, layers) = if let Some(second) = parts.next() {
                    (first.parse::<i32>().unwrap_or(0), second)
                } else {
                    (0, first)
                };
                if (0..=3).contains(&version) {
                    let result = layers
                        .split(',')
                        .map(|layer| {
                            let mut amount_parts =
                                layer.splitn(2, if version < 3 { 'x' } else { '*' });
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
                                block_name_to_id()
                                    .get(&format_java!("minecraft:{}", block_name))
                                    .copied()
                                    .unwrap_or(0) as u16
                            } else {
                                block_name.parse::<u16>().unwrap_or(0)
                            };

                            let new_block_name = block_flattening_v1450::get_state_for_id_raw(
                                (block_id << 4) | meta as u16,
                            )
                            .map_or_else(|| JavaStr::from_str("minecraft:air"), |state| state.name);
                            if count == 1 {
                                new_block_name.to_string()
                            } else {
                                format!("{}*{}", count, new_block_name)
                            }
                        })
                        .collect::<Vec<_>>()
                        .join(",");
                    let mut result = JavaString::from(result);

                    for part in parts {
                        result.push(';');
                        result.push_java_str(part);
                    }

                    result
                } else {
                    JavaString::from(
                        "minecraft:bedrock,2*minecraft:dirt,minecraft:grass_block;1;village",
                    )
                }
            };

            *generator_options = new_options;
        }),
    );

    // V6
    types::stats_mut().add_structure_converter(DataVersion::new(VERSION, 6), StatsConverter);
    types::objective_mut()
        .add_structure_converter(DataVersion::new(VERSION, 6), ObjectiveConverter);
    types::tile_entity_mut().add_converter_for_id(
        "minecraft:jukebox",
        DataVersion::new(VERSION, 6),
        map_data_converter_func(|data, _from_version, _to_version| {
            let record = data.get("Record").and_then(|v| v.as_i32()).unwrap_or(0);
            if record <= 0 {
                return;
            }
            data.remove("Record");

            if let Some(new_item_id) = item_name_v102::get_name_from_id(record)
                .and_then(|str| flatten_item_stack_v1451::flatten_item(str, 0))
            {
                let record_item = jcompound! {
                    "id" => new_item_id,
                    "Count" => 1i8,
                };
                data.insert("RecordItem", record_item);
            }
        }),
    );

    types::stats_mut().add_structure_walker(
        DataVersion::new(VERSION, 6),
        map_data_walker(move |data, from_version, to_version| {
            if let Some(JValue::Compound(stats)) = data.get_mut("stats") {
                rename_keys_in_map(
                    types::block_name_ref(),
                    stats,
                    "minecraft:mined",
                    from_version,
                    to_version,
                );

                rename_keys_in_map(
                    types::item_name_ref(),
                    stats,
                    "minecraft:crafted",
                    from_version,
                    to_version,
                );
                rename_keys_in_map(
                    types::item_name_ref(),
                    stats,
                    "minecraft:used",
                    from_version,
                    to_version,
                );
                rename_keys_in_map(
                    types::item_name_ref(),
                    stats,
                    "minecraft:broken",
                    from_version,
                    to_version,
                );
                rename_keys_in_map(
                    types::item_name_ref(),
                    stats,
                    "minecraft:picked_up",
                    from_version,
                    to_version,
                );
                rename_keys_in_map(
                    types::item_name_ref(),
                    stats,
                    "minecraft:dropped",
                    from_version,
                    to_version,
                );

                rename_keys_in_map(
                    types::entity_name_ref(),
                    stats,
                    "minecraft:killed",
                    from_version,
                    to_version,
                );
                rename_keys_in_map(
                    types::entity_name_ref(),
                    stats,
                    "minecraft:killed_by",
                    from_version,
                    to_version,
                );
            }
        }),
    );

    struct ObjectiveHook;
    impl MapDataHook for ObjectiveHook {
        fn pre_hook(
            &self,
            data: &mut JCompound,
            _from_version: DataVersion,
            _to_version: DataVersion,
        ) {
            // unpack
            if let Some(JValue::String(criteria_name)) = data.get("CriteriaName") {
                fn try_get_type_and_id(
                    criteria_name: &JavaStr,
                ) -> Option<(JavaString, JavaString)> {
                    let (typ, id) = criteria_name.split_at(criteria_name.find(':')?);
                    let id = id.strip_prefix(':').unwrap();
                    let typ = ResourceLocation::parse_with_separator(typ, '.')
                        .ok()?
                        .to_java_string();
                    let id = ResourceLocation::parse_with_separator(id, '.')
                        .ok()?
                        .to_java_string();
                    Some((typ, id))
                }
                let (typ, id) = try_get_type_and_id(criteria_name)
                    .unwrap_or_else(|| (JavaString::from("_special"), criteria_name.to_owned()));

                let criteria_type = jcompound! {
                    "type" => typ,
                    "id" => id,
                };
                data.insert("CriteriaType", criteria_type);
            }
        }

        fn post_hook(
            &self,
            data: &mut JCompound,
            _from_version: DataVersion,
            _to_version: DataVersion,
        ) {
            // repack
            if let Some(JValue::Compound(criteria_type)) = data.get("CriteriaType") {
                if let (Some(JValue::String(typ)), Some(JValue::String(id))) =
                    (criteria_type.get("type"), criteria_type.get("id"))
                {
                    let new_name = if typ == "_special" {
                        id.to_owned()
                    } else {
                        format_java!("{}:{}", pack_with_dot(typ), pack_with_dot(id))
                    };

                    data.remove("CriteriaType");
                    data.insert("CriteriaName", new_name);
                }
            }
        }
    }
    types::objective_mut().add_structure_hook(DataVersion::new(VERSION, 6), ObjectiveHook);

    types::objective_mut().add_structure_walker(
        DataVersion::new(VERSION, 6),
        map_data_walker(move |data, from_version, to_version| {
            let Some(JValue::Compound(criteria_type)) = data.get_mut("CriteriaType") else {
                return;
            };
            let Some(JValue::String(typ)) = criteria_type.get("type") else {
                return;
            };
            match typ.as_bytes() {
                b"minecraft:mined" => {
                    convert_object_in_map(
                        types::block_name_ref(),
                        criteria_type,
                        "id",
                        from_version,
                        to_version,
                    );
                }
                b"minecraft:crafted"
                | b"minecraft:used"
                | b"minecraft:broken"
                | b"minecraft:picked_up"
                | b"minecraft:dropped" => {
                    convert_object_in_map(
                        types::item_name_ref(),
                        criteria_type,
                        "id",
                        from_version,
                        to_version,
                    );
                }
                b"minecraft:killed" | b"minecraft:killed_by" => {
                    convert_object_in_map(
                        types::entity_name_ref(),
                        criteria_type,
                        "id",
                        from_version,
                        to_version,
                    );
                }
                _ => {}
            }
        }),
    );

    // V7
    types::structure_feature_mut().add_structure_converter(
        DataVersion::new(VERSION, 7),
        map_data_converter_func(|data, _from_version, _to_version| {
            fn convert_to_block_state(data: &mut JCompound, path: &str) {
                if let Some(id) = data.get(path).and_then(|v| v.as_i16()) {
                    data.insert(
                        path,
                        block_flattening_v1450::get_nbt_for_id((id as u16) << 4),
                    );
                }
            }

            if let Some(JValue::List(JList::Compound(children))) = data.get_mut("Children") {
                for child in children.iter_mut() {
                    let Some(JValue::String(id)) = child.get("id") else {
                        continue;
                    };
                    match id.as_bytes() {
                        b"ViF" => {
                            convert_to_block_state(child, "CA");
                            convert_to_block_state(child, "CB");
                        }
                        b"ViDF" => {
                            convert_to_block_state(child, "CA");
                            convert_to_block_state(child, "CB");
                            convert_to_block_state(child, "CC");
                            convert_to_block_state(child, "CD");
                        }
                        _ => {}
                    }
                }
            }
        }),
    );

    // convert villagers to trade with pumpkins and not the carved pumpkin
    types::entity_mut().add_converter_for_id("minecraft:villager", DataVersion::new(VERSION, 7), map_data_converter_func(|data, _from_version, _to_version| {
        fn convert_pumpkin(data: &mut JCompound, path: &str) {
            if let Some(JValue::Compound(item)) = data.get_mut(path) {
                if matches!(item.get("id"), Some(JValue::String(str)) if str == "minecraft:carved_pumpkin") {
                    item.insert("id", "minecraft:pumpkin");
                }
            }
        }

        if let Some(JValue::Compound(offers)) = data.get_mut("Offers") {
            if let Some(JValue::List(JList::Compound(recipes))) = offers.get_mut("Recipes") {
                for recipe in recipes {
                    convert_pumpkin(recipe, "buy");
                    convert_pumpkin(recipe, "buyB");
                    convert_pumpkin(recipe, "sell");
                }
            }
        }
    }));

    types::structure_feature_mut().add_structure_walker(
        DataVersion::new(VERSION, 7),
        map_data_walker(move |data, from_version, to_version| {
            if let Some(JValue::List(JList::Compound(children))) = data.get_mut("Children") {
                for child in children {
                    convert_map_in_map(
                        types::block_state_ref(),
                        child,
                        "CA",
                        from_version,
                        to_version,
                    );
                    convert_map_in_map(
                        types::block_state_ref(),
                        child,
                        "CB",
                        from_version,
                        to_version,
                    );
                    convert_map_in_map(
                        types::block_state_ref(),
                        child,
                        "CC",
                        from_version,
                        to_version,
                    );
                    convert_map_in_map(
                        types::block_state_ref(),
                        child,
                        "CD",
                        from_version,
                        to_version,
                    );
                }
            }
        }),
    );
}

fn register_entity_flatteners() {
    types::entity_mut().add_converter_for_id(
        "minecraft:falling_block",
        DataVersion::new(VERSION, 3),
        map_data_converter_func(|data, _from_version, _to_version| {
            let block_id = if data.contains_key("Block") {
                if let Some(id) = data.get("Block").and_then(|v| v.as_i16()) {
                    id as u16
                } else if let Some(JValue::String(id)) = data.get("Block") {
                    block_name_to_id().get(id).copied().unwrap_or(0) as u16
                } else {
                    0
                }
            } else {
                if let Some(id) = data.get("TileID").and_then(|v| v.as_i16()) {
                    id as u16
                } else if let Some(id) = data.get("Tile").and_then(|v| v.as_i8()) {
                    id as u8 as u16
                } else {
                    0
                }
            };

            let block_data = data.get("Data").and_then(|v| v.as_i8()).unwrap_or(0) as u8 & 15;

            data.remove("Block"); // from type update
            data.remove("Data");
            data.remove("TileID");
            data.remove("Tile");

            // key is from type update
            data.insert(
                "BlockState",
                block_flattening_v1450::get_nbt_for_id((block_id << 4) | block_data as u16),
            );
        }),
    );
    convert_entity_state(
        "minecraft:enderman",
        "carried",
        "carriedData",
        "carriedBlockState",
    );
    convert_entity_state("minecraft:arrow", "inTile", "inData", "inBlockState");
    convert_entity_state(
        "minecraft:spectral_arrow",
        "inTile",
        "inData",
        "inBlockState",
    );
    remove_in_tile("minecraft:egg");
    remove_in_tile("minecraft:ender_pearl");
    remove_in_tile("minecraft:fireball");
    remove_in_tile("minecraft:potion");
    remove_in_tile("minecraft:small_fireball");
    remove_in_tile("minecraft:snowball");
    remove_in_tile("minecraft:wither_skull");
    remove_in_tile("minecraft:xp_bottle");
    convert_entity_state(
        "minecraft:commandblock_minecart",
        "DisplayTile",
        "DisplayData",
        "DisplayState",
    );
    convert_entity_state(
        "minecraft:minecart",
        "DisplayTile",
        "DisplayData",
        "DisplayState",
    );
    convert_entity_state(
        "minecraft:chest_minecart",
        "DisplayTile",
        "DisplayData",
        "DisplayState",
    );
    convert_entity_state(
        "minecraft:furnace_minecart",
        "DisplayTile",
        "DisplayData",
        "DisplayState",
    );
    convert_entity_state(
        "minecraft:tnt_minecart",
        "DisplayTile",
        "DisplayData",
        "DisplayState",
    );
    convert_entity_state(
        "minecraft:hopper_minecart",
        "DisplayTile",
        "DisplayData",
        "DisplayState",
    );
    convert_entity_state(
        "minecraft:spawner_minecart",
        "DisplayTile",
        "DisplayData",
        "DisplayState",
    );
}

fn remove_in_tile(entity_id: impl Into<JavaString>) {
    types::entity_mut().add_converter_for_id(
        entity_id,
        DataVersion::new(VERSION, 3),
        map_data_converter_func(|data, _from_version, _to_version| {
            data.remove("inTile");
        }),
    );
}

fn convert_entity_state(
    entity_id: impl Into<JavaString>,
    id_path: &'static str,
    data_path: &'static str,
    output_path: impl Into<JavaString> + Clone + 'static,
) {
    types::entity_mut().add_converter_for_id(
        entity_id,
        DataVersion::new(VERSION, 3),
        map_data_converter_func(move |data, _from_version, _to_version| {
            let block_id = if let Some(id) = data.get(id_path).and_then(|v| v.as_i16()) {
                id as u16
            } else if let Some(JValue::String(id)) = data.get(id_path) {
                block_name_to_id().get(id).copied().unwrap_or(0) as u16
            } else {
                0
            };

            let block_data = data.get(data_path).and_then(|v| v.as_i8()).unwrap_or(0) as u8 & 15;

            data.remove(id_path);
            data.remove(data_path);
            data.insert(
                output_path.clone(),
                block_flattening_v1450::get_nbt_for_id((block_id << 4) | block_data as u16),
            );
        }),
    );
}

pub(crate) fn pack_with_dot(id: &JavaStr) -> JavaString {
    ResourceLocation::parse(id).map_or_else(
        |_| id.to_owned(),
        |loc| format_java!("{}.{}", loc.namespace, loc.path),
    )
}

pub(crate) struct ConverterFlattenSpawnEgg;

impl MapDataConverterFunc for ConverterFlattenSpawnEgg {
    fn convert(&self, data: &mut JCompound, _from_version: DataVersion, _to_version: DataVersion) {
        if let Some(JValue::Compound(tag)) = data.get("tag") {
            if let Some(JValue::Compound(entity_tag)) = tag.get("EntityTag") {
                if let Some(JValue::String(id)) = entity_tag.get("id") {
                    let new_id = entity_id_to_new_egg_id()
                        .get(id)
                        .copied()
                        .unwrap_or(JavaStr::from_str("minecraft:pig_spawn_egg"));
                    data.insert("id", new_id);
                }
            }
        }
    }
}
