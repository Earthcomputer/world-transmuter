use ahash::AHashMap;
use std::sync::OnceLock;

static ITEM_NAMES: OnceLock<AHashMap<i32, &'static str>> = OnceLock::new();

fn item_names() -> &'static AHashMap<i32, &'static str> {
    ITEM_NAMES.get_or_init(|| {
        let mut map = AHashMap::new();
        map.insert(0, "minecraft:air");
        map.insert(1, "minecraft:stone");
        map.insert(2, "minecraft:grass");
        map.insert(3, "minecraft:dirt");
        map.insert(4, "minecraft:cobblestone");
        map.insert(5, "minecraft:planks");
        map.insert(6, "minecraft:sapling");
        map.insert(7, "minecraft:bedrock");
        map.insert(8, "minecraft:flowing_water");
        map.insert(9, "minecraft:water");
        map.insert(10, "minecraft:flowing_lava");
        map.insert(11, "minecraft:lava");
        map.insert(12, "minecraft:sand");
        map.insert(13, "minecraft:gravel");
        map.insert(14, "minecraft:gold_ore");
        map.insert(15, "minecraft:iron_ore");
        map.insert(16, "minecraft:coal_ore");
        map.insert(17, "minecraft:log");
        map.insert(18, "minecraft:leaves");
        map.insert(19, "minecraft:sponge");
        map.insert(20, "minecraft:glass");
        map.insert(21, "minecraft:lapis_ore");
        map.insert(22, "minecraft:lapis_block");
        map.insert(23, "minecraft:dispenser");
        map.insert(24, "minecraft:sandstone");
        map.insert(25, "minecraft:noteblock");
        map.insert(27, "minecraft:golden_rail");
        map.insert(28, "minecraft:detector_rail");
        map.insert(29, "minecraft:sticky_piston");
        map.insert(30, "minecraft:web");
        map.insert(31, "minecraft:tallgrass");
        map.insert(32, "minecraft:deadbush");
        map.insert(33, "minecraft:piston");
        map.insert(35, "minecraft:wool");
        map.insert(37, "minecraft:yellow_flower");
        map.insert(38, "minecraft:red_flower");
        map.insert(39, "minecraft:brown_mushroom");
        map.insert(40, "minecraft:red_mushroom");
        map.insert(41, "minecraft:gold_block");
        map.insert(42, "minecraft:iron_block");
        map.insert(43, "minecraft:double_stone_slab");
        map.insert(44, "minecraft:stone_slab");
        map.insert(45, "minecraft:brick_block");
        map.insert(46, "minecraft:tnt");
        map.insert(47, "minecraft:bookshelf");
        map.insert(48, "minecraft:mossy_cobblestone");
        map.insert(49, "minecraft:obsidian");
        map.insert(50, "minecraft:torch");
        map.insert(51, "minecraft:fire");
        map.insert(52, "minecraft:mob_spawner");
        map.insert(53, "minecraft:oak_stairs");
        map.insert(54, "minecraft:chest");
        map.insert(56, "minecraft:diamond_ore");
        map.insert(57, "minecraft:diamond_block");
        map.insert(58, "minecraft:crafting_table");
        map.insert(60, "minecraft:farmland");
        map.insert(61, "minecraft:furnace");
        map.insert(62, "minecraft:lit_furnace");
        map.insert(65, "minecraft:ladder");
        map.insert(66, "minecraft:rail");
        map.insert(67, "minecraft:stone_stairs");
        map.insert(69, "minecraft:lever");
        map.insert(70, "minecraft:stone_pressure_plate");
        map.insert(72, "minecraft:wooden_pressure_plate");
        map.insert(73, "minecraft:redstone_ore");
        map.insert(76, "minecraft:redstone_torch");
        map.insert(77, "minecraft:stone_button");
        map.insert(78, "minecraft:snow_layer");
        map.insert(79, "minecraft:ice");
        map.insert(80, "minecraft:snow");
        map.insert(81, "minecraft:cactus");
        map.insert(82, "minecraft:clay");
        map.insert(84, "minecraft:jukebox");
        map.insert(85, "minecraft:fence");
        map.insert(86, "minecraft:pumpkin");
        map.insert(87, "minecraft:netherrack");
        map.insert(88, "minecraft:soul_sand");
        map.insert(89, "minecraft:glowstone");
        map.insert(90, "minecraft:portal");
        map.insert(91, "minecraft:lit_pumpkin");
        map.insert(95, "minecraft:stained_glass");
        map.insert(96, "minecraft:trapdoor");
        map.insert(97, "minecraft:monster_egg");
        map.insert(98, "minecraft:stonebrick");
        map.insert(99, "minecraft:brown_mushroom_block");
        map.insert(100, "minecraft:red_mushroom_block");
        map.insert(101, "minecraft:iron_bars");
        map.insert(102, "minecraft:glass_pane");
        map.insert(103, "minecraft:melon_block");
        map.insert(106, "minecraft:vine");
        map.insert(107, "minecraft:fence_gate");
        map.insert(108, "minecraft:brick_stairs");
        map.insert(109, "minecraft:stone_brick_stairs");
        map.insert(110, "minecraft:mycelium");
        map.insert(111, "minecraft:waterlily");
        map.insert(112, "minecraft:nether_brick");
        map.insert(113, "minecraft:nether_brick_fence");
        map.insert(114, "minecraft:nether_brick_stairs");
        map.insert(116, "minecraft:enchanting_table");
        map.insert(119, "minecraft:end_portal");
        map.insert(120, "minecraft:end_portal_frame");
        map.insert(121, "minecraft:end_stone");
        map.insert(122, "minecraft:dragon_egg");
        map.insert(123, "minecraft:redstone_lamp");
        map.insert(125, "minecraft:double_wooden_slab");
        map.insert(126, "minecraft:wooden_slab");
        map.insert(127, "minecraft:cocoa");
        map.insert(128, "minecraft:sandstone_stairs");
        map.insert(129, "minecraft:emerald_ore");
        map.insert(130, "minecraft:ender_chest");
        map.insert(131, "minecraft:tripwire_hook");
        map.insert(133, "minecraft:emerald_block");
        map.insert(134, "minecraft:spruce_stairs");
        map.insert(135, "minecraft:birch_stairs");
        map.insert(136, "minecraft:jungle_stairs");
        map.insert(137, "minecraft:command_block");
        map.insert(138, "minecraft:beacon");
        map.insert(139, "minecraft:cobblestone_wall");
        map.insert(141, "minecraft:carrots");
        map.insert(142, "minecraft:potatoes");
        map.insert(143, "minecraft:wooden_button");
        map.insert(145, "minecraft:anvil");
        map.insert(146, "minecraft:trapped_chest");
        map.insert(147, "minecraft:light_weighted_pressure_plate");
        map.insert(148, "minecraft:heavy_weighted_pressure_plate");
        map.insert(151, "minecraft:daylight_detector");
        map.insert(152, "minecraft:redstone_block");
        map.insert(153, "minecraft:quartz_ore");
        map.insert(154, "minecraft:hopper");
        map.insert(155, "minecraft:quartz_block");
        map.insert(156, "minecraft:quartz_stairs");
        map.insert(157, "minecraft:activator_rail");
        map.insert(158, "minecraft:dropper");
        map.insert(159, "minecraft:stained_hardened_clay");
        map.insert(160, "minecraft:stained_glass_pane");
        map.insert(161, "minecraft:leaves2");
        map.insert(162, "minecraft:log2");
        map.insert(163, "minecraft:acacia_stairs");
        map.insert(164, "minecraft:dark_oak_stairs");
        map.insert(170, "minecraft:hay_block");
        map.insert(171, "minecraft:carpet");
        map.insert(172, "minecraft:hardened_clay");
        map.insert(173, "minecraft:coal_block");
        map.insert(174, "minecraft:packed_ice");
        map.insert(175, "minecraft:double_plant");
        map.insert(256, "minecraft:iron_shovel");
        map.insert(257, "minecraft:iron_pickaxe");
        map.insert(258, "minecraft:iron_axe");
        map.insert(259, "minecraft:flint_and_steel");
        map.insert(260, "minecraft:apple");
        map.insert(261, "minecraft:bow");
        map.insert(262, "minecraft:arrow");
        map.insert(263, "minecraft:coal");
        map.insert(264, "minecraft:diamond");
        map.insert(265, "minecraft:iron_ingot");
        map.insert(266, "minecraft:gold_ingot");
        map.insert(267, "minecraft:iron_sword");
        map.insert(268, "minecraft:wooden_sword");
        map.insert(269, "minecraft:wooden_shovel");
        map.insert(270, "minecraft:wooden_pickaxe");
        map.insert(271, "minecraft:wooden_axe");
        map.insert(272, "minecraft:stone_sword");
        map.insert(273, "minecraft:stone_shovel");
        map.insert(274, "minecraft:stone_pickaxe");
        map.insert(275, "minecraft:stone_axe");
        map.insert(276, "minecraft:diamond_sword");
        map.insert(277, "minecraft:diamond_shovel");
        map.insert(278, "minecraft:diamond_pickaxe");
        map.insert(279, "minecraft:diamond_axe");
        map.insert(280, "minecraft:stick");
        map.insert(281, "minecraft:bowl");
        map.insert(282, "minecraft:mushroom_stew");
        map.insert(283, "minecraft:golden_sword");
        map.insert(284, "minecraft:golden_shovel");
        map.insert(285, "minecraft:golden_pickaxe");
        map.insert(286, "minecraft:golden_axe");
        map.insert(287, "minecraft:string");
        map.insert(288, "minecraft:feather");
        map.insert(289, "minecraft:gunpowder");
        map.insert(290, "minecraft:wooden_hoe");
        map.insert(291, "minecraft:stone_hoe");
        map.insert(292, "minecraft:iron_hoe");
        map.insert(293, "minecraft:diamond_hoe");
        map.insert(294, "minecraft:golden_hoe");
        map.insert(295, "minecraft:wheat_seeds");
        map.insert(296, "minecraft:wheat");
        map.insert(297, "minecraft:bread");
        map.insert(298, "minecraft:leather_helmet");
        map.insert(299, "minecraft:leather_chestplate");
        map.insert(300, "minecraft:leather_leggings");
        map.insert(301, "minecraft:leather_boots");
        map.insert(302, "minecraft:chainmail_helmet");
        map.insert(303, "minecraft:chainmail_chestplate");
        map.insert(304, "minecraft:chainmail_leggings");
        map.insert(305, "minecraft:chainmail_boots");
        map.insert(306, "minecraft:iron_helmet");
        map.insert(307, "minecraft:iron_chestplate");
        map.insert(308, "minecraft:iron_leggings");
        map.insert(309, "minecraft:iron_boots");
        map.insert(310, "minecraft:diamond_helmet");
        map.insert(311, "minecraft:diamond_chestplate");
        map.insert(312, "minecraft:diamond_leggings");
        map.insert(313, "minecraft:diamond_boots");
        map.insert(314, "minecraft:golden_helmet");
        map.insert(315, "minecraft:golden_chestplate");
        map.insert(316, "minecraft:golden_leggings");
        map.insert(317, "minecraft:golden_boots");
        map.insert(318, "minecraft:flint");
        map.insert(319, "minecraft:porkchop");
        map.insert(320, "minecraft:cooked_porkchop");
        map.insert(321, "minecraft:painting");
        map.insert(322, "minecraft:golden_apple");
        map.insert(323, "minecraft:sign");
        map.insert(324, "minecraft:wooden_door");
        map.insert(325, "minecraft:bucket");
        map.insert(326, "minecraft:water_bucket");
        map.insert(327, "minecraft:lava_bucket");
        map.insert(328, "minecraft:minecart");
        map.insert(329, "minecraft:saddle");
        map.insert(330, "minecraft:iron_door");
        map.insert(331, "minecraft:redstone");
        map.insert(332, "minecraft:snowball");
        map.insert(333, "minecraft:boat");
        map.insert(334, "minecraft:leather");
        map.insert(335, "minecraft:milk_bucket");
        map.insert(336, "minecraft:brick");
        map.insert(337, "minecraft:clay_ball");
        map.insert(338, "minecraft:reeds");
        map.insert(339, "minecraft:paper");
        map.insert(340, "minecraft:book");
        map.insert(341, "minecraft:slime_ball");
        map.insert(342, "minecraft:chest_minecart");
        map.insert(343, "minecraft:furnace_minecart");
        map.insert(344, "minecraft:egg");
        map.insert(345, "minecraft:compass");
        map.insert(346, "minecraft:fishing_rod");
        map.insert(347, "minecraft:clock");
        map.insert(348, "minecraft:glowstone_dust");
        map.insert(349, "minecraft:fish");
        map.insert(350, "minecraft:cooked_fish"); // Fix typo, the game never recognized cooked_fished
        map.insert(351, "minecraft:dye");
        map.insert(352, "minecraft:bone");
        map.insert(353, "minecraft:sugar");
        map.insert(354, "minecraft:cake");
        map.insert(355, "minecraft:bed");
        map.insert(356, "minecraft:repeater");
        map.insert(357, "minecraft:cookie");
        map.insert(358, "minecraft:filled_map");
        map.insert(359, "minecraft:shears");
        map.insert(360, "minecraft:melon");
        map.insert(361, "minecraft:pumpkin_seeds");
        map.insert(362, "minecraft:melon_seeds");
        map.insert(363, "minecraft:beef");
        map.insert(364, "minecraft:cooked_beef");
        map.insert(365, "minecraft:chicken");
        map.insert(366, "minecraft:cooked_chicken");
        map.insert(367, "minecraft:rotten_flesh");
        map.insert(368, "minecraft:ender_pearl");
        map.insert(369, "minecraft:blaze_rod");
        map.insert(370, "minecraft:ghast_tear");
        map.insert(371, "minecraft:gold_nugget");
        map.insert(372, "minecraft:nether_wart");
        map.insert(373, "minecraft:potion");
        map.insert(374, "minecraft:glass_bottle");
        map.insert(375, "minecraft:spider_eye");
        map.insert(376, "minecraft:fermented_spider_eye");
        map.insert(377, "minecraft:blaze_powder");
        map.insert(378, "minecraft:magma_cream");
        map.insert(379, "minecraft:brewing_stand");
        map.insert(380, "minecraft:cauldron");
        map.insert(381, "minecraft:ender_eye");
        map.insert(382, "minecraft:speckled_melon");
        map.insert(383, "minecraft:spawn_egg");
        map.insert(384, "minecraft:experience_bottle");
        map.insert(385, "minecraft:fire_charge");
        map.insert(386, "minecraft:writable_book");
        map.insert(387, "minecraft:written_book");
        map.insert(388, "minecraft:emerald");
        map.insert(389, "minecraft:item_frame");
        map.insert(390, "minecraft:flower_pot");
        map.insert(391, "minecraft:carrot");
        map.insert(392, "minecraft:potato");
        map.insert(393, "minecraft:baked_potato");
        map.insert(394, "minecraft:poisonous_potato");
        map.insert(395, "minecraft:map");
        map.insert(396, "minecraft:golden_carrot");
        map.insert(397, "minecraft:skull");
        map.insert(398, "minecraft:carrot_on_a_stick");
        map.insert(399, "minecraft:nether_star");
        map.insert(400, "minecraft:pumpkin_pie");
        map.insert(401, "minecraft:fireworks");
        map.insert(402, "minecraft:firework_charge");
        map.insert(403, "minecraft:enchanted_book");
        map.insert(404, "minecraft:comparator");
        map.insert(405, "minecraft:netherbrick");
        map.insert(406, "minecraft:quartz");
        map.insert(407, "minecraft:tnt_minecart");
        map.insert(408, "minecraft:hopper_minecart");
        map.insert(417, "minecraft:iron_horse_armor");
        map.insert(418, "minecraft:golden_horse_armor");
        map.insert(419, "minecraft:diamond_horse_armor");
        map.insert(420, "minecraft:lead");
        map.insert(421, "minecraft:name_tag");
        map.insert(422, "minecraft:command_block_minecart");
        map.insert(2256, "minecraft:record_13");
        map.insert(2257, "minecraft:record_cat");
        map.insert(2258, "minecraft:record_blocks");
        map.insert(2259, "minecraft:record_chirp");
        map.insert(2260, "minecraft:record_far");
        map.insert(2261, "minecraft:record_mall");
        map.insert(2262, "minecraft:record_mellohi");
        map.insert(2263, "minecraft:record_stal");
        map.insert(2264, "minecraft:record_strad");
        map.insert(2265, "minecraft:record_ward");
        map.insert(2266, "minecraft:record_11");
        map.insert(2267, "minecraft:record_wait");

        // https://github.com/starlis/empirecraft/commit/2da59d1901407fc0c135ef0458c0fe9b016570b3
        // It's likely that this is a result of old CB/Spigot behavior still writing ids into items as ints.
        // These ids do not appear to be used by regular MC anyways, so I do not see the harm of porting it here.
        // Extras can be added if needed

        // EMC start
        map.insert(409, "minecraft:prismarine_shard");
        map.insert(410, "minecraft:prismarine_crystals");
        map.insert(411, "minecraft:rabbit");
        map.insert(412, "minecraft:cooked_rabbit");
        map.insert(413, "minecraft:rabbit_stew");
        map.insert(414, "minecraft:rabbit_foot");
        map.insert(415, "minecraft:rabbit_hide");
        map.insert(416, "minecraft:armor_stand");
        map.insert(423, "minecraft:mutton");
        map.insert(424, "minecraft:cooked_mutton");
        map.insert(425, "minecraft:banner");
        map.insert(426, "minecraft:end_crystal");
        map.insert(427, "minecraft:spruce_door");
        map.insert(428, "minecraft:birch_door");
        map.insert(429, "minecraft:jungle_door");
        map.insert(430, "minecraft:acacia_door");
        map.insert(431, "minecraft:dark_oak_door");
        map.insert(432, "minecraft:chorus_fruit");
        map.insert(433, "minecraft:chorus_fruit_popped");
        map.insert(434, "minecraft:beetroot");
        map.insert(435, "minecraft:beetroot_seeds");
        map.insert(436, "minecraft:beetroot_soup");
        map.insert(437, "minecraft:dragon_breath");
        map.insert(438, "minecraft:splash_potion");
        map.insert(439, "minecraft:spectral_arrow");
        map.insert(440, "minecraft:tipped_arrow");
        map.insert(441, "minecraft:lingering_potion");
        map.insert(442, "minecraft:shield");
        map.insert(443, "minecraft:elytra");
        map.insert(444, "minecraft:spruce_boat");
        map.insert(445, "minecraft:birch_boat");
        map.insert(446, "minecraft:jungle_boat");
        map.insert(447, "minecraft:acacia_boat");
        map.insert(448, "minecraft:dark_oak_boat");
        map.insert(449, "minecraft:totem_of_undying");
        map.insert(450, "minecraft:shulker_shell");
        map.insert(452, "minecraft:iron_nugget");
        map.insert(453, "minecraft:knowledge_book");
        // EMC end

        // Add block ids into conversion as well
        // Very old versions of the game handled them, but it seems 1.8.8 did not parse them at all, so no conversion
        // was written.
        // block ids are only skipped (set to AIR) if there is no 1-1 replacement item.
        map.insert(26, "minecraft:bed"); // bed block
        map.insert(34, map.get(&0).unwrap()); // skip (piston head block)
        map.insert(55, "minecraft:redstone"); // redstone wire block
        map.insert(59, map.get(&0).unwrap()); // skip (wheat crop block)
        map.insert(63, "minecraft:sign"); // standing sign
        map.insert(64, "minecraft:wooden_door"); // wooden door block
        map.insert(68, "minecraft:sign"); // wall sign
        map.insert(71, "minecraft:iron_door"); // iron door block
        map.insert(74, "minecraft:redstone_ore"); // lit redstone ore block
        map.insert(75, "minecraft:redstone_torch"); // unlit redstone torch
        map.insert(83, "minecraft:reeds"); // sugar cane block
        map.insert(92, "minecraft:cake"); // cake block
        map.insert(93, "minecraft:repeater"); // unpowered repeater block
        map.insert(94, "minecraft:repeater"); // powered repeater block
        map.insert(104, map.get(&0).unwrap()); // skip (pumpkin stem)
        map.insert(105, map.get(&0).unwrap()); // skip (melon stem)
        map.insert(115, "minecraft:nether_wart"); // nether wart block
        map.insert(117, "minecraft:brewing_stand"); // brewing stand block
        map.insert(118, "minecraft:cauldron"); // cauldron block
        map.insert(124, "minecraft:redstone_lamp"); // lit redstone lamp block
        map.insert(132, map.get(&0).unwrap()); // skip (tripwire wire block)
        map.insert(140, "minecraft:flower_pot"); // flower pot block
        map.insert(144, "minecraft:skull"); // skull block
        map.insert(149, "minecraft:comparator"); // unpowered comparator block
        map.insert(150, "minecraft:comparator"); // powered comparator block
                                                 // there are technically more, but at some point even older versions pre id -> name conversion didn't even load them.
                                                 // (all I know is 1.7.10 does not load them)
                                                 // and so given even the vanilla game wouldn't load them, there's no conversion path for them - they were never valid.

        map
    })
}

static POTION_NAMES: OnceLock<[Option<&'static str>; 128]> = OnceLock::new();

fn potion_names() -> &'static [Option<&'static str>; 128] {
    POTION_NAMES.get_or_init(|| {
        let mut arr = [None; 128];
        arr[0] = Some("minecraft:water");
        arr[1] = Some("minecraft:regeneration");
        arr[2] = Some("minecraft:swiftness");
        arr[3] = Some("minecraft:fire_resistance");
        arr[4] = Some("minecraft:poison");
        arr[5] = Some("minecraft:healing");
        arr[6] = Some("minecraft:night_vision");
        arr[8] = Some("minecraft:weakness");
        arr[9] = Some("minecraft:strength");
        arr[10] = Some("minecraft:slowness");
        arr[11] = Some("minecraft:leaping");
        arr[12] = Some("minecraft:harming");
        arr[13] = Some("minecraft:water_breathing");
        arr[14] = Some("minecraft:invisibility");
        arr[16] = Some("minecraft:awkward");
        arr[17] = Some("minecraft:regeneration");
        arr[18] = Some("minecraft:swiftness");
        arr[19] = Some("minecraft:fire_resistance");
        arr[20] = Some("minecraft:poison");
        arr[21] = Some("minecraft:healing");
        arr[22] = Some("minecraft:night_vision");
        arr[24] = Some("minecraft:weakness");
        arr[25] = Some("minecraft:strength");
        arr[26] = Some("minecraft:slowness");
        arr[27] = Some("minecraft:leaping");
        arr[28] = Some("minecraft:harming");
        arr[29] = Some("minecraft:water_breathing");
        arr[30] = Some("minecraft:invisibility");
        arr[32] = Some("minecraft:thick");
        arr[33] = Some("minecraft:strong_regeneration");
        arr[34] = Some("minecraft:strong_swiftness");
        arr[35] = Some("minecraft:fire_resistance");
        arr[36] = Some("minecraft:strong_poison");
        arr[37] = Some("minecraft:strong_healing");
        arr[38] = Some("minecraft:night_vision");
        arr[40] = Some("minecraft:weakness");
        arr[41] = Some("minecraft:strong_strength");
        arr[42] = Some("minecraft:slowness");
        arr[43] = Some("minecraft:strong_leaping");
        arr[44] = Some("minecraft:strong_harming");
        arr[45] = Some("minecraft:water_breathing");
        arr[46] = Some("minecraft:invisibility");
        arr[49] = Some("minecraft:strong_regeneration");
        arr[50] = Some("minecraft:strong_swiftness");
        arr[51] = Some("minecraft:fire_resistance");
        arr[52] = Some("minecraft:strong_poison");
        arr[53] = Some("minecraft:strong_healing");
        arr[54] = Some("minecraft:night_vision");
        arr[56] = Some("minecraft:weakness");
        arr[57] = Some("minecraft:strong_strength");
        arr[58] = Some("minecraft:slowness");
        arr[59] = Some("minecraft:strong_leaping");
        arr[60] = Some("minecraft:strong_harming");
        arr[61] = Some("minecraft:water_breathing");
        arr[62] = Some("minecraft:invisibility");
        arr[64] = Some("minecraft:mundane");
        arr[65] = Some("minecraft:long_regeneration");
        arr[66] = Some("minecraft:long_swiftness");
        arr[67] = Some("minecraft:long_fire_resistance");
        arr[68] = Some("minecraft:long_poison");
        arr[69] = Some("minecraft:healing");
        arr[70] = Some("minecraft:long_night_vision");
        arr[72] = Some("minecraft:long_weakness");
        arr[73] = Some("minecraft:long_strength");
        arr[74] = Some("minecraft:long_slowness");
        arr[75] = Some("minecraft:long_leaping");
        arr[76] = Some("minecraft:harming");
        arr[77] = Some("minecraft:long_water_breathing");
        arr[78] = Some("minecraft:long_invisibility");
        arr[80] = Some("minecraft:awkward");
        arr[81] = Some("minecraft:long_regeneration");
        arr[82] = Some("minecraft:long_swiftness");
        arr[83] = Some("minecraft:long_fire_resistance");
        arr[84] = Some("minecraft:long_poison");
        arr[85] = Some("minecraft:healing");
        arr[86] = Some("minecraft:long_night_vision");
        arr[88] = Some("minecraft:long_weakness");
        arr[89] = Some("minecraft:long_strength");
        arr[90] = Some("minecraft:long_slowness");
        arr[91] = Some("minecraft:long_leaping");
        arr[92] = Some("minecraft:harming");
        arr[93] = Some("minecraft:long_water_breathing");
        arr[94] = Some("minecraft:long_invisibility");
        arr[96] = Some("minecraft:thick");
        arr[97] = Some("minecraft:regeneration");
        arr[98] = Some("minecraft:swiftness");
        arr[99] = Some("minecraft:long_fire_resistance");
        arr[100] = Some("minecraft:poison");
        arr[101] = Some("minecraft:strong_healing");
        arr[102] = Some("minecraft:long_night_vision");
        arr[104] = Some("minecraft:long_weakness");
        arr[105] = Some("minecraft:strength");
        arr[106] = Some("minecraft:long_slowness");
        arr[107] = Some("minecraft:leaping");
        arr[108] = Some("minecraft:strong_harming");
        arr[109] = Some("minecraft:long_water_breathing");
        arr[110] = Some("minecraft:long_invisibility");
        arr[113] = Some("minecraft:regeneration");
        arr[114] = Some("minecraft:swiftness");
        arr[115] = Some("minecraft:long_fire_resistance");
        arr[116] = Some("minecraft:poison");
        arr[117] = Some("minecraft:strong_healing");
        arr[118] = Some("minecraft:long_night_vision");
        arr[120] = Some("minecraft:long_weakness");
        arr[121] = Some("minecraft:strength");
        arr[122] = Some("minecraft:long_slowness");
        arr[123] = Some("minecraft:leaping");
        arr[124] = Some("minecraft:strong_harming");
        arr[125] = Some("minecraft:long_water_breathing");
        arr[126] = Some("minecraft:long_invisibility");
        arr
    })
}

pub fn get_name_from_id(id: i32) -> Option<&'static str> {
    item_names().get(&id).copied()
}

pub fn get_potion_name_from_id(id: i32) -> Option<&'static str> {
    potion_names()[(id & 127) as usize].as_ref().copied()
}
