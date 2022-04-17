use std::lazy::SyncOnceCell;

static ITEM_NAMES: SyncOnceCell<rust_dataconverter_engine::Map<i32, String>> = SyncOnceCell::new();

fn item_names() -> &'static rust_dataconverter_engine::Map<i32, String> {
    ITEM_NAMES.get_or_init(|| {
        let mut map = rust_dataconverter_engine::Map::new();
        map.insert(0, "minecraft:air".to_owned());
        map.insert(1, "minecraft:stone".to_owned());
        map.insert(2, "minecraft:grass".to_owned());
        map.insert(3, "minecraft:dirt".to_owned());
        map.insert(4, "minecraft:cobblestone".to_owned());
        map.insert(5, "minecraft:planks".to_owned());
        map.insert(6, "minecraft:sapling".to_owned());
        map.insert(7, "minecraft:bedrock".to_owned());
        map.insert(8, "minecraft:flowing_water".to_owned());
        map.insert(9, "minecraft:water".to_owned());
        map.insert(10, "minecraft:flowing_lava".to_owned());
        map.insert(11, "minecraft:lava".to_owned());
        map.insert(12, "minecraft:sand".to_owned());
        map.insert(13, "minecraft:gravel".to_owned());
        map.insert(14, "minecraft:gold_ore".to_owned());
        map.insert(15, "minecraft:iron_ore".to_owned());
        map.insert(16, "minecraft:coal_ore".to_owned());
        map.insert(17, "minecraft:log".to_owned());
        map.insert(18, "minecraft:leaves".to_owned());
        map.insert(19, "minecraft:sponge".to_owned());
        map.insert(20, "minecraft:glass".to_owned());
        map.insert(21, "minecraft:lapis_ore".to_owned());
        map.insert(22, "minecraft:lapis_block".to_owned());
        map.insert(23, "minecraft:dispenser".to_owned());
        map.insert(24, "minecraft:sandstone".to_owned());
        map.insert(25, "minecraft:noteblock".to_owned());
        map.insert(27, "minecraft:golden_rail".to_owned());
        map.insert(28, "minecraft:detector_rail".to_owned());
        map.insert(29, "minecraft:sticky_piston".to_owned());
        map.insert(30, "minecraft:web".to_owned());
        map.insert(31, "minecraft:tallgrass".to_owned());
        map.insert(32, "minecraft:deadbush".to_owned());
        map.insert(33, "minecraft:piston".to_owned());
        map.insert(35, "minecraft:wool".to_owned());
        map.insert(37, "minecraft:yellow_flower".to_owned());
        map.insert(38, "minecraft:red_flower".to_owned());
        map.insert(39, "minecraft:brown_mushroom".to_owned());
        map.insert(40, "minecraft:red_mushroom".to_owned());
        map.insert(41, "minecraft:gold_block".to_owned());
        map.insert(42, "minecraft:iron_block".to_owned());
        map.insert(43, "minecraft:double_stone_slab".to_owned());
        map.insert(44, "minecraft:stone_slab".to_owned());
        map.insert(45, "minecraft:brick_block".to_owned());
        map.insert(46, "minecraft:tnt".to_owned());
        map.insert(47, "minecraft:bookshelf".to_owned());
        map.insert(48, "minecraft:mossy_cobblestone".to_owned());
        map.insert(49, "minecraft:obsidian".to_owned());
        map.insert(50, "minecraft:torch".to_owned());
        map.insert(51, "minecraft:fire".to_owned());
        map.insert(52, "minecraft:mob_spawner".to_owned());
        map.insert(53, "minecraft:oak_stairs".to_owned());
        map.insert(54, "minecraft:chest".to_owned());
        map.insert(56, "minecraft:diamond_ore".to_owned());
        map.insert(57, "minecraft:diamond_block".to_owned());
        map.insert(58, "minecraft:crafting_table".to_owned());
        map.insert(60, "minecraft:farmland".to_owned());
        map.insert(61, "minecraft:furnace".to_owned());
        map.insert(62, "minecraft:lit_furnace".to_owned());
        map.insert(65, "minecraft:ladder".to_owned());
        map.insert(66, "minecraft:rail".to_owned());
        map.insert(67, "minecraft:stone_stairs".to_owned());
        map.insert(69, "minecraft:lever".to_owned());
        map.insert(70, "minecraft:stone_pressure_plate".to_owned());
        map.insert(72, "minecraft:wooden_pressure_plate".to_owned());
        map.insert(73, "minecraft:redstone_ore".to_owned());
        map.insert(76, "minecraft:redstone_torch".to_owned());
        map.insert(77, "minecraft:stone_button".to_owned());
        map.insert(78, "minecraft:snow_layer".to_owned());
        map.insert(79, "minecraft:ice".to_owned());
        map.insert(80, "minecraft:snow".to_owned());
        map.insert(81, "minecraft:cactus".to_owned());
        map.insert(82, "minecraft:clay".to_owned());
        map.insert(84, "minecraft:jukebox".to_owned());
        map.insert(85, "minecraft:fence".to_owned());
        map.insert(86, "minecraft:pumpkin".to_owned());
        map.insert(87, "minecraft:netherrack".to_owned());
        map.insert(88, "minecraft:soul_sand".to_owned());
        map.insert(89, "minecraft:glowstone".to_owned());
        map.insert(90, "minecraft:portal".to_owned());
        map.insert(91, "minecraft:lit_pumpkin".to_owned());
        map.insert(95, "minecraft:stained_glass".to_owned());
        map.insert(96, "minecraft:trapdoor".to_owned());
        map.insert(97, "minecraft:monster_egg".to_owned());
        map.insert(98, "minecraft:stonebrick".to_owned());
        map.insert(99, "minecraft:brown_mushroom_block".to_owned());
        map.insert(100, "minecraft:red_mushroom_block".to_owned());
        map.insert(101, "minecraft:iron_bars".to_owned());
        map.insert(102, "minecraft:glass_pane".to_owned());
        map.insert(103, "minecraft:melon_block".to_owned());
        map.insert(106, "minecraft:vine".to_owned());
        map.insert(107, "minecraft:fence_gate".to_owned());
        map.insert(108, "minecraft:brick_stairs".to_owned());
        map.insert(109, "minecraft:stone_brick_stairs".to_owned());
        map.insert(110, "minecraft:mycelium".to_owned());
        map.insert(111, "minecraft:waterlily".to_owned());
        map.insert(112, "minecraft:nether_brick".to_owned());
        map.insert(113, "minecraft:nether_brick_fence".to_owned());
        map.insert(114, "minecraft:nether_brick_stairs".to_owned());
        map.insert(116, "minecraft:enchanting_table".to_owned());
        map.insert(119, "minecraft:end_portal".to_owned());
        map.insert(120, "minecraft:end_portal_frame".to_owned());
        map.insert(121, "minecraft:end_stone".to_owned());
        map.insert(122, "minecraft:dragon_egg".to_owned());
        map.insert(123, "minecraft:redstone_lamp".to_owned());
        map.insert(125, "minecraft:double_wooden_slab".to_owned());
        map.insert(126, "minecraft:wooden_slab".to_owned());
        map.insert(127, "minecraft:cocoa".to_owned());
        map.insert(128, "minecraft:sandstone_stairs".to_owned());
        map.insert(129, "minecraft:emerald_ore".to_owned());
        map.insert(130, "minecraft:ender_chest".to_owned());
        map.insert(131, "minecraft:tripwire_hook".to_owned());
        map.insert(133, "minecraft:emerald_block".to_owned());
        map.insert(134, "minecraft:spruce_stairs".to_owned());
        map.insert(135, "minecraft:birch_stairs".to_owned());
        map.insert(136, "minecraft:jungle_stairs".to_owned());
        map.insert(137, "minecraft:command_block".to_owned());
        map.insert(138, "minecraft:beacon".to_owned());
        map.insert(139, "minecraft:cobblestone_wall".to_owned());
        map.insert(141, "minecraft:carrots".to_owned());
        map.insert(142, "minecraft:potatoes".to_owned());
        map.insert(143, "minecraft:wooden_button".to_owned());
        map.insert(145, "minecraft:anvil".to_owned());
        map.insert(146, "minecraft:trapped_chest".to_owned());
        map.insert(147, "minecraft:light_weighted_pressure_plate".to_owned());
        map.insert(148, "minecraft:heavy_weighted_pressure_plate".to_owned());
        map.insert(151, "minecraft:daylight_detector".to_owned());
        map.insert(152, "minecraft:redstone_block".to_owned());
        map.insert(153, "minecraft:quartz_ore".to_owned());
        map.insert(154, "minecraft:hopper".to_owned());
        map.insert(155, "minecraft:quartz_block".to_owned());
        map.insert(156, "minecraft:quartz_stairs".to_owned());
        map.insert(157, "minecraft:activator_rail".to_owned());
        map.insert(158, "minecraft:dropper".to_owned());
        map.insert(159, "minecraft:stained_hardened_clay".to_owned());
        map.insert(160, "minecraft:stained_glass_pane".to_owned());
        map.insert(161, "minecraft:leaves2".to_owned());
        map.insert(162, "minecraft:log2".to_owned());
        map.insert(163, "minecraft:acacia_stairs".to_owned());
        map.insert(164, "minecraft:dark_oak_stairs".to_owned());
        map.insert(170, "minecraft:hay_block".to_owned());
        map.insert(171, "minecraft:carpet".to_owned());
        map.insert(172, "minecraft:hardened_clay".to_owned());
        map.insert(173, "minecraft:coal_block".to_owned());
        map.insert(174, "minecraft:packed_ice".to_owned());
        map.insert(175, "minecraft:double_plant".to_owned());
        map.insert(256, "minecraft:iron_shovel".to_owned());
        map.insert(257, "minecraft:iron_pickaxe".to_owned());
        map.insert(258, "minecraft:iron_axe".to_owned());
        map.insert(259, "minecraft:flint_and_steel".to_owned());
        map.insert(260, "minecraft:apple".to_owned());
        map.insert(261, "minecraft:bow".to_owned());
        map.insert(262, "minecraft:arrow".to_owned());
        map.insert(263, "minecraft:coal".to_owned());
        map.insert(264, "minecraft:diamond".to_owned());
        map.insert(265, "minecraft:iron_ingot".to_owned());
        map.insert(266, "minecraft:gold_ingot".to_owned());
        map.insert(267, "minecraft:iron_sword".to_owned());
        map.insert(268, "minecraft:wooden_sword".to_owned());
        map.insert(269, "minecraft:wooden_shovel".to_owned());
        map.insert(270, "minecraft:wooden_pickaxe".to_owned());
        map.insert(271, "minecraft:wooden_axe".to_owned());
        map.insert(272, "minecraft:stone_sword".to_owned());
        map.insert(273, "minecraft:stone_shovel".to_owned());
        map.insert(274, "minecraft:stone_pickaxe".to_owned());
        map.insert(275, "minecraft:stone_axe".to_owned());
        map.insert(276, "minecraft:diamond_sword".to_owned());
        map.insert(277, "minecraft:diamond_shovel".to_owned());
        map.insert(278, "minecraft:diamond_pickaxe".to_owned());
        map.insert(279, "minecraft:diamond_axe".to_owned());
        map.insert(280, "minecraft:stick".to_owned());
        map.insert(281, "minecraft:bowl".to_owned());
        map.insert(282, "minecraft:mushroom_stew".to_owned());
        map.insert(283, "minecraft:golden_sword".to_owned());
        map.insert(284, "minecraft:golden_shovel".to_owned());
        map.insert(285, "minecraft:golden_pickaxe".to_owned());
        map.insert(286, "minecraft:golden_axe".to_owned());
        map.insert(287, "minecraft:string".to_owned());
        map.insert(288, "minecraft:feather".to_owned());
        map.insert(289, "minecraft:gunpowder".to_owned());
        map.insert(290, "minecraft:wooden_hoe".to_owned());
        map.insert(291, "minecraft:stone_hoe".to_owned());
        map.insert(292, "minecraft:iron_hoe".to_owned());
        map.insert(293, "minecraft:diamond_hoe".to_owned());
        map.insert(294, "minecraft:golden_hoe".to_owned());
        map.insert(295, "minecraft:wheat_seeds".to_owned());
        map.insert(296, "minecraft:wheat".to_owned());
        map.insert(297, "minecraft:bread".to_owned());
        map.insert(298, "minecraft:leather_helmet".to_owned());
        map.insert(299, "minecraft:leather_chestplate".to_owned());
        map.insert(300, "minecraft:leather_leggings".to_owned());
        map.insert(301, "minecraft:leather_boots".to_owned());
        map.insert(302, "minecraft:chainmail_helmet".to_owned());
        map.insert(303, "minecraft:chainmail_chestplate".to_owned());
        map.insert(304, "minecraft:chainmail_leggings".to_owned());
        map.insert(305, "minecraft:chainmail_boots".to_owned());
        map.insert(306, "minecraft:iron_helmet".to_owned());
        map.insert(307, "minecraft:iron_chestplate".to_owned());
        map.insert(308, "minecraft:iron_leggings".to_owned());
        map.insert(309, "minecraft:iron_boots".to_owned());
        map.insert(310, "minecraft:diamond_helmet".to_owned());
        map.insert(311, "minecraft:diamond_chestplate".to_owned());
        map.insert(312, "minecraft:diamond_leggings".to_owned());
        map.insert(313, "minecraft:diamond_boots".to_owned());
        map.insert(314, "minecraft:golden_helmet".to_owned());
        map.insert(315, "minecraft:golden_chestplate".to_owned());
        map.insert(316, "minecraft:golden_leggings".to_owned());
        map.insert(317, "minecraft:golden_boots".to_owned());
        map.insert(318, "minecraft:flint".to_owned());
        map.insert(319, "minecraft:porkchop".to_owned());
        map.insert(320, "minecraft:cooked_porkchop".to_owned());
        map.insert(321, "minecraft:painting".to_owned());
        map.insert(322, "minecraft:golden_apple".to_owned());
        map.insert(323, "minecraft:sign".to_owned());
        map.insert(324, "minecraft:wooden_door".to_owned());
        map.insert(325, "minecraft:bucket".to_owned());
        map.insert(326, "minecraft:water_bucket".to_owned());
        map.insert(327, "minecraft:lava_bucket".to_owned());
        map.insert(328, "minecraft:minecart".to_owned());
        map.insert(329, "minecraft:saddle".to_owned());
        map.insert(330, "minecraft:iron_door".to_owned());
        map.insert(331, "minecraft:redstone".to_owned());
        map.insert(332, "minecraft:snowball".to_owned());
        map.insert(333, "minecraft:boat".to_owned());
        map.insert(334, "minecraft:leather".to_owned());
        map.insert(335, "minecraft:milk_bucket".to_owned());
        map.insert(336, "minecraft:brick".to_owned());
        map.insert(337, "minecraft:clay_ball".to_owned());
        map.insert(338, "minecraft:reeds".to_owned());
        map.insert(339, "minecraft:paper".to_owned());
        map.insert(340, "minecraft:book".to_owned());
        map.insert(341, "minecraft:slime_ball".to_owned());
        map.insert(342, "minecraft:chest_minecart".to_owned());
        map.insert(343, "minecraft:furnace_minecart".to_owned());
        map.insert(344, "minecraft:egg".to_owned());
        map.insert(345, "minecraft:compass".to_owned());
        map.insert(346, "minecraft:fishing_rod".to_owned());
        map.insert(347, "minecraft:clock".to_owned());
        map.insert(348, "minecraft:glowstone_dust".to_owned());
        map.insert(349, "minecraft:fish".to_owned());
        map.insert(350, "minecraft:cooked_fish".to_owned()); // Fix typo, the game never recognized cooked_fished
        map.insert(351, "minecraft:dye".to_owned());
        map.insert(352, "minecraft:bone".to_owned());
        map.insert(353, "minecraft:sugar".to_owned());
        map.insert(354, "minecraft:cake".to_owned());
        map.insert(355, "minecraft:bed".to_owned());
        map.insert(356, "minecraft:repeater".to_owned());
        map.insert(357, "minecraft:cookie".to_owned());
        map.insert(358, "minecraft:filled_map".to_owned());
        map.insert(359, "minecraft:shears".to_owned());
        map.insert(360, "minecraft:melon".to_owned());
        map.insert(361, "minecraft:pumpkin_seeds".to_owned());
        map.insert(362, "minecraft:melon_seeds".to_owned());
        map.insert(363, "minecraft:beef".to_owned());
        map.insert(364, "minecraft:cooked_beef".to_owned());
        map.insert(365, "minecraft:chicken".to_owned());
        map.insert(366, "minecraft:cooked_chicken".to_owned());
        map.insert(367, "minecraft:rotten_flesh".to_owned());
        map.insert(368, "minecraft:ender_pearl".to_owned());
        map.insert(369, "minecraft:blaze_rod".to_owned());
        map.insert(370, "minecraft:ghast_tear".to_owned());
        map.insert(371, "minecraft:gold_nugget".to_owned());
        map.insert(372, "minecraft:nether_wart".to_owned());
        map.insert(373, "minecraft:potion".to_owned());
        map.insert(374, "minecraft:glass_bottle".to_owned());
        map.insert(375, "minecraft:spider_eye".to_owned());
        map.insert(376, "minecraft:fermented_spider_eye".to_owned());
        map.insert(377, "minecraft:blaze_powder".to_owned());
        map.insert(378, "minecraft:magma_cream".to_owned());
        map.insert(379, "minecraft:brewing_stand".to_owned());
        map.insert(380, "minecraft:cauldron".to_owned());
        map.insert(381, "minecraft:ender_eye".to_owned());
        map.insert(382, "minecraft:speckled_melon".to_owned());
        map.insert(383, "minecraft:spawn_egg".to_owned());
        map.insert(384, "minecraft:experience_bottle".to_owned());
        map.insert(385, "minecraft:fire_charge".to_owned());
        map.insert(386, "minecraft:writable_book".to_owned());
        map.insert(387, "minecraft:written_book".to_owned());
        map.insert(388, "minecraft:emerald".to_owned());
        map.insert(389, "minecraft:item_frame".to_owned());
        map.insert(390, "minecraft:flower_pot".to_owned());
        map.insert(391, "minecraft:carrot".to_owned());
        map.insert(392, "minecraft:potato".to_owned());
        map.insert(393, "minecraft:baked_potato".to_owned());
        map.insert(394, "minecraft:poisonous_potato".to_owned());
        map.insert(395, "minecraft:map".to_owned());
        map.insert(396, "minecraft:golden_carrot".to_owned());
        map.insert(397, "minecraft:skull".to_owned());
        map.insert(398, "minecraft:carrot_on_a_stick".to_owned());
        map.insert(399, "minecraft:nether_star".to_owned());
        map.insert(400, "minecraft:pumpkin_pie".to_owned());
        map.insert(401, "minecraft:fireworks".to_owned());
        map.insert(402, "minecraft:firework_charge".to_owned());
        map.insert(403, "minecraft:enchanted_book".to_owned());
        map.insert(404, "minecraft:comparator".to_owned());
        map.insert(405, "minecraft:netherbrick".to_owned());
        map.insert(406, "minecraft:quartz".to_owned());
        map.insert(407, "minecraft:tnt_minecart".to_owned());
        map.insert(408, "minecraft:hopper_minecart".to_owned());
        map.insert(417, "minecraft:iron_horse_armor".to_owned());
        map.insert(418, "minecraft:golden_horse_armor".to_owned());
        map.insert(419, "minecraft:diamond_horse_armor".to_owned());
        map.insert(420, "minecraft:lead".to_owned());
        map.insert(421, "minecraft:name_tag".to_owned());
        map.insert(422, "minecraft:command_block_minecart".to_owned());
        map.insert(2256, "minecraft:record_13".to_owned());
        map.insert(2257, "minecraft:record_cat".to_owned());
        map.insert(2258, "minecraft:record_blocks".to_owned());
        map.insert(2259, "minecraft:record_chirp".to_owned());
        map.insert(2260, "minecraft:record_far".to_owned());
        map.insert(2261, "minecraft:record_mall".to_owned());
        map.insert(2262, "minecraft:record_mellohi".to_owned());
        map.insert(2263, "minecraft:record_stal".to_owned());
        map.insert(2264, "minecraft:record_strad".to_owned());
        map.insert(2265, "minecraft:record_ward".to_owned());
        map.insert(2266, "minecraft:record_11".to_owned());
        map.insert(2267, "minecraft:record_wait".to_owned());

        // https://github.com/starlis/empirecraft/commit/2da59d1901407fc0c135ef0458c0fe9b016570b3
        // It's likely that this is a result of old CB/Spigot behavior still writing ids into items as ints.
        // These ids do not appear to be used by regular MC anyways, so I do not see the harm of porting it here.
        // Extras can be added if needed

        // EMC start
        map.insert(409, "minecraft:prismarine_shard".to_owned());
        map.insert(410, "minecraft:prismarine_crystals".to_owned());
        map.insert(411, "minecraft:rabbit".to_owned());
        map.insert(412, "minecraft:cooked_rabbit".to_owned());
        map.insert(413, "minecraft:rabbit_stew".to_owned());
        map.insert(414, "minecraft:rabbit_foot".to_owned());
        map.insert(415, "minecraft:rabbit_hide".to_owned());
        map.insert(416, "minecraft:armor_stand".to_owned());
        map.insert(423, "minecraft:mutton".to_owned());
        map.insert(424, "minecraft:cooked_mutton".to_owned());
        map.insert(425, "minecraft:banner".to_owned());
        map.insert(426, "minecraft:end_crystal".to_owned());
        map.insert(427, "minecraft:spruce_door".to_owned());
        map.insert(428, "minecraft:birch_door".to_owned());
        map.insert(429, "minecraft:jungle_door".to_owned());
        map.insert(430, "minecraft:acacia_door".to_owned());
        map.insert(431, "minecraft:dark_oak_door".to_owned());
        map.insert(432, "minecraft:chorus_fruit".to_owned());
        map.insert(433, "minecraft:chorus_fruit_popped".to_owned());
        map.insert(434, "minecraft:beetroot".to_owned());
        map.insert(435, "minecraft:beetroot_seeds".to_owned());
        map.insert(436, "minecraft:beetroot_soup".to_owned());
        map.insert(437, "minecraft:dragon_breath".to_owned());
        map.insert(438, "minecraft:splash_potion".to_owned());
        map.insert(439, "minecraft:spectral_arrow".to_owned());
        map.insert(440, "minecraft:tipped_arrow".to_owned());
        map.insert(441, "minecraft:lingering_potion".to_owned());
        map.insert(442, "minecraft:shield".to_owned());
        map.insert(443, "minecraft:elytra".to_owned());
        map.insert(444, "minecraft:spruce_boat".to_owned());
        map.insert(445, "minecraft:birch_boat".to_owned());
        map.insert(446, "minecraft:jungle_boat".to_owned());
        map.insert(447, "minecraft:acacia_boat".to_owned());
        map.insert(448, "minecraft:dark_oak_boat".to_owned());
        map.insert(449, "minecraft:totem_of_undying".to_owned());
        map.insert(450, "minecraft:shulker_shell".to_owned());
        map.insert(452, "minecraft:iron_nugget".to_owned());
        map.insert(453, "minecraft:knowledge_book".to_owned());
        // EMC end

        // Add block ids into conversion as well
        // Very old versions of the game handled them, but it seems 1.8.8 did not parse them at all, so no conversion
        // was written.
        // block ids are only skipped (set to AIR) if there is no 1-1 replacement item.
        map.insert(26, "minecraft:bed".to_owned()); // bed block
        map.insert(34, map.get(&0).unwrap().to_owned()); // skip (piston head block)
        map.insert(55, "minecraft:redstone".to_owned()); // redstone wire block
        map.insert(59, map.get(&0).unwrap().to_owned()); // skip (wheat crop block)
        map.insert(63, "minecraft:sign".to_owned()); // standing sign
        map.insert(64, "minecraft:wooden_door".to_owned()); // wooden door block
        map.insert(68, "minecraft:sign".to_owned()); // wall sign
        map.insert(71, "minecraft:iron_door".to_owned()); // iron door block
        map.insert(74, "minecraft:redstone_ore".to_owned()); // lit redstone ore block
        map.insert(75, "minecraft:redstone_torch".to_owned()); // unlit redstone torch
        map.insert(83, "minecraft:reeds".to_owned()); // sugar cane block
        map.insert(92, "minecraft:cake".to_owned()); // cake block
        map.insert(93, "minecraft:repeater".to_owned()); // unpowered repeater block
        map.insert(94, "minecraft:repeater".to_owned()); // powered repeater block
        map.insert(104, map.get(&0).unwrap().to_owned()); // skip (pumpkin stem)
        map.insert(105, map.get(&0).unwrap().to_owned()); // skip (melon stem)
        map.insert(115, "minecraft:nether_wart".to_owned()); // nether wart block
        map.insert(117, "minecraft:brewing_stand".to_owned()); // brewing stand block
        map.insert(118, "minecraft:cauldron".to_owned()); // cauldron block
        map.insert(124, "minecraft:redstone_lamp".to_owned()); // lit redstone lamp block
        map.insert(132, map.get(&0).unwrap().to_owned()); // skip (tripwire wire block)
        map.insert(140, "minecraft:flower_pot".to_owned()); // flower pot block
        map.insert(144, "minecraft:skull".to_owned()); // skull block
        map.insert(149, "minecraft:comparator".to_owned()); // unpowered comparator block
        map.insert(150, "minecraft:comparator".to_owned()); // powered comparator block
        // there are technically more, but at some point even older versions pre id -> name conversion didn't even load them.
        // (all I know is 1.7.10 does not load them)
        // and so given even the vanilla game wouldn't load them, there's no conversion path for them - they were never valid.

        map
    })
}

static POTION_NAMES: SyncOnceCell<[Option<String>; 128]> = SyncOnceCell::new();

fn potion_names() -> &'static [Option<String>; 128] {
    POTION_NAMES.get_or_init(|| {
        const INIT: Option<String> = None;
        let mut arr = [INIT; 128];
        arr[0] = Some("minecraft:water".to_owned());
        arr[1] = Some("minecraft:regeneration".to_owned());
        arr[2] = Some("minecraft:swiftness".to_owned());
        arr[3] = Some("minecraft:fire_resistance".to_owned());
        arr[4] = Some("minecraft:poison".to_owned());
        arr[5] = Some("minecraft:healing".to_owned());
        arr[6] = Some("minecraft:night_vision".to_owned());
        arr[8] = Some("minecraft:weakness".to_owned());
        arr[9] = Some("minecraft:strength".to_owned());
        arr[10] = Some("minecraft:slowness".to_owned());
        arr[11] = Some("minecraft:leaping".to_owned());
        arr[12] = Some("minecraft:harming".to_owned());
        arr[13] = Some("minecraft:water_breathing".to_owned());
        arr[14] = Some("minecraft:invisibility".to_owned());
        arr[16] = Some("minecraft:awkward".to_owned());
        arr[17] = Some("minecraft:regeneration".to_owned());
        arr[18] = Some("minecraft:swiftness".to_owned());
        arr[19] = Some("minecraft:fire_resistance".to_owned());
        arr[20] = Some("minecraft:poison".to_owned());
        arr[21] = Some("minecraft:healing".to_owned());
        arr[22] = Some("minecraft:night_vision".to_owned());
        arr[24] = Some("minecraft:weakness".to_owned());
        arr[25] = Some("minecraft:strength".to_owned());
        arr[26] = Some("minecraft:slowness".to_owned());
        arr[27] = Some("minecraft:leaping".to_owned());
        arr[28] = Some("minecraft:harming".to_owned());
        arr[29] = Some("minecraft:water_breathing".to_owned());
        arr[30] = Some("minecraft:invisibility".to_owned());
        arr[32] = Some("minecraft:thick".to_owned());
        arr[33] = Some("minecraft:strong_regeneration".to_owned());
        arr[34] = Some("minecraft:strong_swiftness".to_owned());
        arr[35] = Some("minecraft:fire_resistance".to_owned());
        arr[36] = Some("minecraft:strong_poison".to_owned());
        arr[37] = Some("minecraft:strong_healing".to_owned());
        arr[38] = Some("minecraft:night_vision".to_owned());
        arr[40] = Some("minecraft:weakness".to_owned());
        arr[41] = Some("minecraft:strong_strength".to_owned());
        arr[42] = Some("minecraft:slowness".to_owned());
        arr[43] = Some("minecraft:strong_leaping".to_owned());
        arr[44] = Some("minecraft:strong_harming".to_owned());
        arr[45] = Some("minecraft:water_breathing".to_owned());
        arr[46] = Some("minecraft:invisibility".to_owned());
        arr[49] = Some("minecraft:strong_regeneration".to_owned());
        arr[50] = Some("minecraft:strong_swiftness".to_owned());
        arr[51] = Some("minecraft:fire_resistance".to_owned());
        arr[52] = Some("minecraft:strong_poison".to_owned());
        arr[53] = Some("minecraft:strong_healing".to_owned());
        arr[54] = Some("minecraft:night_vision".to_owned());
        arr[56] = Some("minecraft:weakness".to_owned());
        arr[57] = Some("minecraft:strong_strength".to_owned());
        arr[58] = Some("minecraft:slowness".to_owned());
        arr[59] = Some("minecraft:strong_leaping".to_owned());
        arr[60] = Some("minecraft:strong_harming".to_owned());
        arr[61] = Some("minecraft:water_breathing".to_owned());
        arr[62] = Some("minecraft:invisibility".to_owned());
        arr[64] = Some("minecraft:mundane".to_owned());
        arr[65] = Some("minecraft:long_regeneration".to_owned());
        arr[66] = Some("minecraft:long_swiftness".to_owned());
        arr[67] = Some("minecraft:long_fire_resistance".to_owned());
        arr[68] = Some("minecraft:long_poison".to_owned());
        arr[69] = Some("minecraft:healing".to_owned());
        arr[70] = Some("minecraft:long_night_vision".to_owned());
        arr[72] = Some("minecraft:long_weakness".to_owned());
        arr[73] = Some("minecraft:long_strength".to_owned());
        arr[74] = Some("minecraft:long_slowness".to_owned());
        arr[75] = Some("minecraft:long_leaping".to_owned());
        arr[76] = Some("minecraft:harming".to_owned());
        arr[77] = Some("minecraft:long_water_breathing".to_owned());
        arr[78] = Some("minecraft:long_invisibility".to_owned());
        arr[80] = Some("minecraft:awkward".to_owned());
        arr[81] = Some("minecraft:long_regeneration".to_owned());
        arr[82] = Some("minecraft:long_swiftness".to_owned());
        arr[83] = Some("minecraft:long_fire_resistance".to_owned());
        arr[84] = Some("minecraft:long_poison".to_owned());
        arr[85] = Some("minecraft:healing".to_owned());
        arr[86] = Some("minecraft:long_night_vision".to_owned());
        arr[88] = Some("minecraft:long_weakness".to_owned());
        arr[89] = Some("minecraft:long_strength".to_owned());
        arr[90] = Some("minecraft:long_slowness".to_owned());
        arr[91] = Some("minecraft:long_leaping".to_owned());
        arr[92] = Some("minecraft:harming".to_owned());
        arr[93] = Some("minecraft:long_water_breathing".to_owned());
        arr[94] = Some("minecraft:long_invisibility".to_owned());
        arr[96] = Some("minecraft:thick".to_owned());
        arr[97] = Some("minecraft:regeneration".to_owned());
        arr[98] = Some("minecraft:swiftness".to_owned());
        arr[99] = Some("minecraft:long_fire_resistance".to_owned());
        arr[100] = Some("minecraft:poison".to_owned());
        arr[101] = Some("minecraft:strong_healing".to_owned());
        arr[102] = Some("minecraft:long_night_vision".to_owned());
        arr[104] = Some("minecraft:long_weakness".to_owned());
        arr[105] = Some("minecraft:strength".to_owned());
        arr[106] = Some("minecraft:long_slowness".to_owned());
        arr[107] = Some("minecraft:leaping".to_owned());
        arr[108] = Some("minecraft:strong_harming".to_owned());
        arr[109] = Some("minecraft:long_water_breathing".to_owned());
        arr[110] = Some("minecraft:long_invisibility".to_owned());
        arr[113] = Some("minecraft:regeneration".to_owned());
        arr[114] = Some("minecraft:swiftness".to_owned());
        arr[115] = Some("minecraft:long_fire_resistance".to_owned());
        arr[116] = Some("minecraft:poison".to_owned());
        arr[117] = Some("minecraft:strong_healing".to_owned());
        arr[118] = Some("minecraft:long_night_vision".to_owned());
        arr[120] = Some("minecraft:long_weakness".to_owned());
        arr[121] = Some("minecraft:strength".to_owned());
        arr[122] = Some("minecraft:long_slowness".to_owned());
        arr[123] = Some("minecraft:leaping".to_owned());
        arr[124] = Some("minecraft:strong_harming".to_owned());
        arr[125] = Some("minecraft:long_water_breathing".to_owned());
        arr[126] = Some("minecraft:long_invisibility".to_owned());
        arr
    })
}

pub fn get_name_from_id(id: i32) -> Option<&'static str> {
    item_names().get(&id).map(|s| s.as_str())
}

pub fn get_potion_name_from_id(id: i32) -> Option<&'static str> {
    potion_names()[(id & 127) as usize].as_ref().map(|s| s.as_str())
}
