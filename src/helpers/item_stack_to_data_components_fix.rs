use crate::helpers::components::make_translatable_component;
use crate::helpers::resource_location::ResourceLocation;
use crate::static_string_set;
use crate::versions::v3818;
use java_string::{JavaStr, JavaString};
use valence_nbt::{compound, jcompound};
use world_transmuter_engine::{rename_key, JCompound, JList, JValue};

const TOOLTIP_FLAG_HIDE_ENCHANTMENTS: i32 = 1 << 0;
const TOOLTIP_FLAG_HIDE_MODIFIERS: i32 = 1 << 1;
const TOOLTIP_FLAG_HIDE_UNBREAKABLE: i32 = 1 << 2;
const TOOLTIP_FLAG_HIDE_CAN_DESTROY: i32 = 1 << 3;
const TOOLTIP_FLAG_HIDE_CAN_PLACE: i32 = 1 << 4;
const TOOLTIP_FLAG_HIDE_ADDITIONAL: i32 = 1 << 5;
const TOOLTIP_FLAG_HIDE_DYE: i32 = 1 << 6;
const TOOLTIP_FLAG_HIDE_UPGRADES: i32 = 1 << 7;

const DEFAULT_LEATHER_COLOR: i32 = 0xA06540;

const BUCKETED_MOB_TAGS: [&JavaStr; 10] = [
    JavaStr::from_str("NoAI"),
    JavaStr::from_str("Silent"),
    JavaStr::from_str("NoGravity"),
    JavaStr::from_str("Glowing"),
    JavaStr::from_str("Invulnerable"),
    JavaStr::from_str("Health"),
    JavaStr::from_str("Age"),
    JavaStr::from_str("Variant"),
    JavaStr::from_str("HuntingCooldown"),
    JavaStr::from_str("BucketVariantTag"),
];

static_string_set! {
    boolean_block_state_properties = {
        "attached",
        "bottom",
        "conditional",
        "disarmed",
        "drag",
        "enabled",
        "extended",
        "eye",
        "falling",
        "hanging",
        "has_bottle_0",
        "has_bottle_1",
        "has_bottle_2",
        "has_record",
        "has_book",
        "inverted",
        "in_wall",
        "lit",
        "locked",
        "occupied",
        "open",
        "persistent",
        "powered",
        "short",
        "signal_fire",
        "snowy",
        "triggered",
        "unstable",
        "waterlogged",
        "berries",
        "bloom",
        "shrieking",
        "can_summon",
        "up",
        "down",
        "north",
        "east",
        "south",
        "west",
        "slot_0_occupied",
        "slot_1_occupied",
        "slot_2_occupied",
        "slot_3_occupied",
        "slot_4_occupied",
        "slot_5_occupied",
        "cracked",
        "crafting",
    }
}

const MAP_DECORATION_CONVERSION_TABLE: [&JavaStr; 34] = [
    JavaStr::from_str("player"),
    JavaStr::from_str("frame"),
    JavaStr::from_str("red_marker"),
    JavaStr::from_str("blue_marker"),
    JavaStr::from_str("target_x"),
    JavaStr::from_str("target_point"),
    JavaStr::from_str("player_off_map"),
    JavaStr::from_str("player_off_limits"),
    JavaStr::from_str("mansion"),
    JavaStr::from_str("monument"),
    JavaStr::from_str("banner_white"),
    JavaStr::from_str("banner_orange"),
    JavaStr::from_str("banner_magenta"),
    JavaStr::from_str("banner_light_blue"),
    JavaStr::from_str("banner_yellow"),
    JavaStr::from_str("banner_lime"),
    JavaStr::from_str("banner_pink"),
    JavaStr::from_str("banner_gray"),
    JavaStr::from_str("banner_light_gray"),
    JavaStr::from_str("banner_cyan"),
    JavaStr::from_str("banner_purple"),
    JavaStr::from_str("banner_blue"),
    JavaStr::from_str("banner_brown"),
    JavaStr::from_str("banner_green"),
    JavaStr::from_str("banner_red"),
    JavaStr::from_str("banner_black"),
    JavaStr::from_str("red_x"),
    JavaStr::from_str("village_desert"),
    JavaStr::from_str("village_plains"),
    JavaStr::from_str("village_savanna"),
    JavaStr::from_str("village_snowy"),
    JavaStr::from_str("village_taiga"),
    JavaStr::from_str("jungle_temple"),
    JavaStr::from_str("swamp_hut"),
];

fn convert_map_decoration_id(typ: i32) -> &'static JavaStr {
    MAP_DECORATION_CONVERSION_TABLE
        .get(typ as usize)
        .copied()
        .unwrap_or_else(|| MAP_DECORATION_CONVERSION_TABLE[0])
}

fn convert_block_state_properties(properties: &mut JCompound) {
    // convert values stored as boolean/integer to string
    for (key, value) in properties.iter_mut() {
        if value.is_number() {
            if boolean_block_state_properties().contains(&key[..]) {
                *value = JValue::String(JavaString::from(value.as_bool().unwrap().to_string()));
            } else if matches!(value, JValue::Float(_) | JValue::Double(_)) {
                *value = JValue::String(JavaString::from(value.as_f64().unwrap().to_string()));
            } else {
                *value = JValue::String(JavaString::from(value.as_i64().unwrap().to_string()));
            }
        }
    }
}

fn convert_tile_entity(tile_entity: &mut JCompound, transient_item: &mut TransientItemStack) {
    if let Some(lock) = tile_entity.remove("Lock") {
        transient_item.components.insert("minecraft:lock", lock);
    }

    if let Some(loot_table) = tile_entity.remove("LootTable") {
        let seed = tile_entity
            .remove("LootTableSeed")
            .and_then(|o| o.as_i64())
            .unwrap_or(0);

        let mut container_loot = jcompound! {
            "loot_table" => loot_table,
        };
        if seed != 0 {
            container_loot.insert("seed", seed);
        }

        transient_item
            .components
            .insert("minecraft:container_loot", container_loot);
    }

    let Some(JValue::String(id)) = tile_entity.get("id") else {
        return;
    };
    let id = ResourceLocation::make_correct(id);

    match id.as_bytes() {
        b"minecraft:skull" => {
            if let Some(note_block_sound) = tile_entity.remove("note_block_sound") {
                transient_item
                    .components
                    .insert("minecraft:note_block_sound", note_block_sound);
            }
        }
        b"minecraft:decorated_pot" => {
            if let Some(sherds) = tile_entity.remove("sherds") {
                transient_item
                    .components
                    .insert("minecraft:pot_decorations", sherds);
            }
            if let Some(item) = tile_entity.remove("item") {
                transient_item.components.insert(
                    "minecraft:container",
                    JList::Compound(vec![jcompound! {
                        "item" => item,
                        "slot" => 0,
                    }]),
                );
            }
        }
        b"minecraft:banner" => {
            if let Some(patterns) = tile_entity.remove("patterns") {
                transient_item
                    .components
                    .insert("minecraft:banner_patterns", patterns);
            }
            if let Some(base) = tile_entity.remove("Base").and_then(|o| o.as_i32()) {
                transient_item
                    .components
                    .insert("minecraft:base_color", v3818::get_banner_color(base));
            }
        }
        b"minecraft:shulker_box"
        | b"minecraft:chest"
        | b"minecraft:trapped_chest"
        | b"minecraft:furnace"
        | b"minecraft:ender_chest"
        | b"minecraft:dispenser"
        | b"minecraft:dropper"
        | b"minecraft:brewing_stand"
        | b"minecraft:hopper"
        | b"minecraft:barrel"
        | b"minecraft:smoker"
        | b"minecraft:blast_furnace"
        | b"minecraft:campfire"
        | b"minecraft:chiseled_bookshelf"
        | b"minecraft:crafter" => {
            if let Some(JValue::List(JList::Compound(mut items))) = tile_entity.remove("Items") {
                for item in &mut items {
                    let slot =
                        item.remove("Slot").and_then(|o| o.as_i8()).unwrap_or(0) as u8 as i32;
                    let wrapped_item = std::mem::take(item);
                    item.insert("slot", slot);
                    item.insert("item", wrapped_item);
                }
            }
        }
        b"minecraft:beehive" => {
            if let Some(bees) = tile_entity.remove("bees") {
                transient_item.components.insert("minecraft:bees", bees);
            }
        }
        _ => {}
    }
}

fn convert_enchantments(
    transient_item: &mut TransientItemStack,
    tag_key: &(impl AsRef<JavaStr> + ?Sized),
    component_key: impl Into<JavaString>,
    hide_tooltip: bool,
) {
    let enchantments = match transient_item.tag.remove(tag_key.as_ref()) {
        Some(JValue::List(JList::Compound(enchantments))) => Some(enchantments),
        _ => None,
    };

    if let Some(enchantments) = &enchantments {
        if enchantments.is_empty() {
            transient_item
                .components
                .insert("minecraft:enchantment_glint_override", true);
        }
    }

    match enchantments {
        Some(enchantments) if !enchantments.is_empty() => {
            let new_enchantments: JCompound = enchantments
                .into_iter()
                .filter_map(|mut enchantment| {
                    if let (Some(JValue::String(id)), Some(level)) = (
                        enchantment.remove("id"),
                        enchantment.get("lvl").and_then(|o| o.as_i32()),
                    ) {
                        Some((id, JValue::Int(level.clamp(0, 255))))
                    } else {
                        None
                    }
                })
                .collect();

            if !new_enchantments.is_empty() || hide_tooltip {
                let mut new_enchants = jcompound! {
                    "levels" => new_enchantments,
                };
                if hide_tooltip {
                    new_enchants.insert("show_in_tooltip", false);
                }
                transient_item
                    .components
                    .insert(component_key, new_enchants);
            }
        }
        _ => {
            if hide_tooltip {
                transient_item.components.insert(
                    component_key,
                    jcompound! {
                        "levels" => JCompound::new(),
                        "show_in_tooltip" => false,
                    },
                );
            }
        }
    }
}

fn convert_display(transient_item: &mut TransientItemStack, flags: i32) {
    let mut display = match transient_item.tag.get_mut("display") {
        Some(JValue::Compound(display)) => Some(display),
        _ => None,
    };

    if let Some(ref mut display) = display {
        if let Some(name) = display.remove("Name") {
            transient_item
                .components
                .insert("minecraft:custom_name", name);
        }
        if let Some(lore) = display.remove("Lore") {
            transient_item.components.insert("minecraft:lore", lore);
        }
    }

    let color = display
        .as_mut()
        .and_then(|display| display.remove("color"))
        .and_then(|o| o.as_i32());
    let hide_dye = (flags & TOOLTIP_FLAG_HIDE_DYE) != 0;

    if hide_dye || color.is_some() {
        let mut dyed_color = jcompound! {
            "rgb" => color.unwrap_or(DEFAULT_LEATHER_COLOR),
        };
        if hide_dye {
            dyed_color.insert("show_in_tooltip", false);
        }

        transient_item
            .components
            .insert("minecraft:dyed_color", dyed_color);
    }

    if let Some(JValue::String(loc_name)) = display
        .as_mut()
        .and_then(|display| display.remove("LocName"))
    {
        transient_item.components.insert(
            "minecraft:item_name",
            make_translatable_component(&loc_name),
        );
    }

    if transient_item.id == "minecraft:filled_map" {
        if let Some(map_color) = display
            .as_mut()
            .and_then(|display| display.remove("MapColor"))
        {
            transient_item
                .components
                .insert("minecraft:map_color", map_color);
        }
    }

    if display.is_some_and(|display| display.is_empty()) {
        transient_item.tag.remove("display");
    }
}

fn convert_block_state_predicate(value: &JavaStr) -> JCompound {
    let property_start = value.find('[');
    let nbt_start = value.find('{');
    let block_name_end = property_start
        .unwrap_or(value.len())
        .min(nbt_start.unwrap_or(value.len()));

    let mut ret = jcompound! {
        "blocks" => &value[..block_name_end],
    };

    if let Some(property_start) = property_start {
        let properties = &value[property_start + 1..];
        if let Some(property_end) = properties.find(']') {
            let properties = &properties[..property_end];
            let state: JCompound = properties
                .split(',')
                .filter_map(|property| {
                    property.split_once('=').map(|(key, value)| {
                        (key.trim().to_owned(), JValue::String(value.to_owned()))
                    })
                })
                .collect();
            ret.insert("state", state);
        }
    }

    if let Some(nbt_start) = nbt_start {
        // note: we want to include { and }
        let nbt = &value[nbt_start..];
        if let Some(nbt_end) = nbt.find('}') {
            let nbt = &nbt[..nbt_end + 1];
            ret.insert("nbt", nbt);
        }
    }

    ret
}

fn convert_block_state_predicates(
    item: &mut TransientItemStack,
    tag_key: &(impl AsRef<JavaStr> + ?Sized),
    component_key: impl Into<JavaString>,
    hide_in_tooltip: bool,
) {
    let Some(JValue::List(mut blocks)) = item.tag.remove(tag_key.as_ref()) else {
        return;
    };

    if let JList::String(block_strings) = blocks {
        blocks = JList::Compound(
            block_strings
                .iter()
                .map(|block| convert_block_state_predicate(block))
                .collect(),
        );
    }

    let mut block_predicates = jcompound! {
        "predicates" => blocks,
    };
    if hide_in_tooltip {
        block_predicates.insert("show_in_tooltip", false);
    }
    item.components.insert(component_key, block_predicates);
}

fn convert_adventure_mode(item: &mut TransientItemStack, flags: i32) {
    convert_block_state_predicates(
        item,
        "CanDestroy",
        "minecraft:can_break",
        (flags & TOOLTIP_FLAG_HIDE_CAN_DESTROY) != 0,
    );
    convert_block_state_predicates(
        item,
        "CanPlaceOn",
        "minecraft:can_place_on",
        (flags & TOOLTIP_FLAG_HIDE_CAN_PLACE) != 0,
    );
}

fn convert_attribute(input: JValue) -> JCompound {
    let mut ret = jcompound! {
        "name" => "",
        "amount" => 0.0,
        "operation" => "add_value",
    };

    let JValue::Compound(mut input) = input else {
        return ret;
    };

    if let Some(attribute_name) = input.remove("AttributeName") {
        ret.insert("type", attribute_name);
    }
    if let Some(slot) = input.remove("Slot") {
        ret.insert("slot", slot);
    }
    if let Some(uuid) = input.remove("UUID") {
        ret.insert("uuid", uuid);
    }
    if let Some(name) = input.remove("Name") {
        ret.insert("name", name);
    }
    if let Some(amount) = input.remove("Amount") {
        ret.insert("amount", amount);
    }

    if let Some(operation) = input.remove("Operation") {
        let operation = match operation.as_i32().unwrap_or(0) {
            1 => "add_multiplied_base",
            2 => "add_multiplied_total",
            _ => "add_value",
        };
        ret.insert("operation", operation);
    }

    ret
}

fn convert_attributes(item: &mut TransientItemStack, flags: i32) {
    let attributes = match item.tag.remove("AttributeModifiers") {
        Some(JValue::List(attributes)) => attributes.into_iter().map(convert_attribute).collect(),
        _ => Vec::new(),
    };

    let hide_modifiers = (flags & TOOLTIP_FLAG_HIDE_MODIFIERS) != 0;
    if !attributes.is_empty() || hide_modifiers {
        let mut modifiers = jcompound! {
            "modifiers" => JList::Compound(attributes),
        };
        if hide_modifiers {
            modifiers.insert("show_in_tooltip", false);
        }

        item.components
            .insert("minecraft:attribute_modifiers", modifiers);
    }
}

fn convert_map(item: &mut TransientItemStack) {
    item.migrate_tag_to_component("map", "minecraft:map_id");

    if let Some(JValue::List(decorations)) = item.tag.remove("Decorations") {
        let new_decorations: JCompound = decorations
            .into_iter()
            .rev() // reverse to make sure the first (not the last) of duplicate direction ids is kept
            .map(|decoration| {
                let mut decoration = match decoration {
                    JValue::Compound(decoration) => Some(decoration),
                    _ => None,
                };

                let id = decoration
                    .as_mut()
                    .and_then(|decoration| match decoration.remove("id") {
                        Some(JValue::String(id)) => Some(id),
                        _ => None,
                    })
                    .unwrap_or_else(JavaString::new);

                let type_id = decoration
                    .as_ref()
                    .and_then(|decoration| decoration.get("type"))
                    .and_then(|type_id| type_id.as_i32())
                    .unwrap_or(0);
                let x = decoration
                    .as_ref()
                    .and_then(|decoration| decoration.get("x"))
                    .and_then(|x| x.as_f64())
                    .unwrap_or(0.0);
                let z = decoration
                    .as_ref()
                    .and_then(|decoration| decoration.get("z"))
                    .and_then(|z| z.as_f64())
                    .unwrap_or(0.0);
                let rot = decoration
                    .as_ref()
                    .and_then(|decoration| decoration.get("rot"))
                    .and_then(|rot| rot.as_f32())
                    .unwrap_or(0.0);

                let new_decoration = jcompound! {
                    "type" => convert_map_decoration_id(type_id),
                    "x" => x,
                    "z" => z,
                    "rotation" => rot,
                };
                (id, JValue::Compound(new_decoration))
            })
            .collect();

        if !new_decorations.is_empty() {
            item.components
                .insert("minecraft:map_decorations", new_decorations);
        }
    }
}

fn convert_potion(item: &mut TransientItemStack) {
    let mut potion_contents = JCompound::new();

    if let Some(JValue::String(potion)) = item.tag.remove("Potion") {
        if potion != "minecraft:empty" {
            potion_contents.insert("potion", potion);
        }
    }

    item.migrate_tag_to("CustomPotionColor", &mut potion_contents, "custom_color");
    item.migrate_tag_to(
        "custom_potion_effects",
        &mut potion_contents,
        "custom_effects",
    );

    if !potion_contents.is_empty() {
        item.components
            .insert("minecraft:potion_contents", potion_contents);
    }
}

fn make_filtered_text(raw: JavaString, filtered: Option<JavaString>) -> JCompound {
    let mut ret = jcompound! {
        "raw" => raw,
    };
    if let Some(filtered) = filtered {
        ret.insert("filtered", filtered);
    }
    ret
}

fn convert_book_pages(item: &mut TransientItemStack) -> Option<Vec<JCompound>> {
    let Some(JValue::List(old_pages)) = item.tag.remove("pages") else {
        return None;
    };
    if old_pages.is_empty() {
        return None;
    };

    let mut filtered_pages = match item.tag.remove("filtered_pages") {
        Some(JValue::Compound(filtered_pages)) => Some(filtered_pages),
        _ => None,
    };

    Some(
        old_pages
            .into_iter()
            .enumerate()
            .map(|(index, page)| {
                let page = match page {
                    JValue::String(page) => page,
                    _ => JavaString::new(),
                };
                let filtered = match filtered_pages
                    .as_mut()
                    .and_then(|filtered_pages| filtered_pages.remove(&index.to_string()))
                {
                    Some(JValue::String(filtered)) => Some(filtered),
                    _ => None,
                };
                make_filtered_text(page, filtered)
            })
            .collect(),
    )
}

fn convert_writable_book(item: &mut TransientItemStack) {
    if let Some(pages) = convert_book_pages(item) {
        item.components.insert(
            "minecraft:writable_book_content",
            jcompound! {
                "pages" => JList::Compound(pages),
            },
        );
    }
}

fn convert_written_book(item: &mut TransientItemStack) {
    let mut book_content = JCompound::new();

    if let Some(pages) = convert_book_pages(item) {
        book_content.insert("pages", JList::Compound(pages));
    }

    let title = match item.tag.remove("title") {
        Some(JValue::String(title)) => title,
        _ => JavaString::new(),
    };
    let filtered_title = match item.tag.remove("filtered_title") {
        Some(JValue::String(filtered_title)) => Some(filtered_title),
        _ => None,
    };

    book_content.insert("title", make_filtered_text(title, filtered_title));

    item.migrate_tag_to("author", &mut book_content, "author");
    item.migrate_tag_to("resolved", &mut book_content, "resolved");
    item.migrate_tag_to("generation", &mut book_content, "generation");

    item.components
        .insert("minecraft:written_book_content", book_content);
}

fn convert_mob_bucket(item: &mut TransientItemStack) {
    let mut bucket_entity_data = JCompound::new();
    for old_key in BUCKETED_MOB_TAGS {
        item.migrate_tag_to(old_key, &mut bucket_entity_data, old_key);
    }
    if !bucket_entity_data.is_empty() {
        item.components
            .insert("minecraft:bucket_entity_data", bucket_entity_data);
    }
}

fn convert_compass(item: &mut TransientItemStack) {
    let lodestone_pos = item.tag.remove("LodestonePos");
    let lodestone_dim = item.tag.remove("LodestoneDimension");

    if lodestone_pos.is_none() && lodestone_dim.is_none() {
        return;
    }

    let mut lodestone_tracker = JCompound::new();

    if let (Some(lodestone_pos), Some(lodestone_dim)) = (lodestone_pos, lodestone_dim) {
        lodestone_tracker.insert(
            "target",
            jcompound! {
                "pos" => lodestone_pos,
                "dimension" => lodestone_dim,
            },
        );
    }

    if item
        .tag
        .remove("LodestoneTracker")
        .and_then(|o| o.as_bool())
        == Some(false)
    {
        lodestone_tracker.insert("tracked", false);
    }

    item.components
        .insert("minecraft:lodestone_tracker", lodestone_tracker);
}

fn convert_firework_explosion(input: &mut JCompound) {
    rename_key(input, "Colors", "colors");
    rename_key(input, "FadeColors", "fade_colors");
    rename_key(input, "Trail", "has_trail");
    rename_key(input, "Flicker", "has_twinkle");

    let new_type = match input.remove("Type").and_then(|o| o.as_i32()).unwrap_or(0) {
        1 => "large_ball",
        2 => "star",
        3 => "creeper",
        4 => "burst",
        _ => "small_ball",
    };
    input.insert("shape", new_type);
}

fn convert_firework_rocket(item: &mut TransientItemStack) {
    if !item.tag.contains_key("Fireworks") {
        return;
    }

    let Some(JValue::Compound(fireworks)) = item.tag.get_mut("Fireworks") else {
        item.components.insert(
            "minecraft:fireworks",
            jcompound! {
                "explosions" => JList::new(),
                "flight_duration" => 0i8,
            },
        );
        return;
    };

    let flight = fireworks
        .remove("Flight")
        .and_then(|o| o.as_i32())
        .unwrap_or(0);
    let mut explosions = match fireworks.remove("Explosions") {
        Some(JValue::List(explosions)) => explosions,
        _ => JList::new(),
    };

    if let JList::Compound(explosions) = &mut explosions {
        for explosion in explosions {
            convert_firework_explosion(explosion);
        }
    }

    if fireworks.is_empty() {
        item.tag.remove("Fireworks");
    }

    item.components.insert(
        "minecraft:fireworks",
        jcompound! {
            "flight" => flight as i8,
            "explosions" => explosions,
        },
    );
}

fn convert_firework_star(item: &mut TransientItemStack) {
    let Some(explosion) = item.tag.get_mut("Explosion") else {
        return;
    };
    let JValue::Compound(explosion) = explosion else {
        item.components
            .insert("minecraft:firework_explosion", explosion.clone());
        return;
    };

    let mut explosion_copy = explosion.clone();
    convert_firework_explosion(&mut explosion_copy);
    item.components
        .insert("minecraft:firework_explosion", explosion_copy);

    explosion.remove("Type");
    explosion.remove("Colors");
    explosion.remove("FadeColors");
    explosion.remove("Trail");
    explosion.remove("Flicker");

    if explosion.is_empty() {
        item.tag.remove("Explosion");
    }
}

fn is_valid_player_name(name: &JavaStr) -> bool {
    name.len() <= 16 && name.bytes().all(|char| char > 0x20 && char < 0x7f) // printable ascii
}

fn convert_properties(properties: JCompound) -> Vec<JCompound> {
    properties
        .into_iter()
        .filter_map(|entry| match entry {
            (property_key, JValue::List(property_values)) => Some((property_key, property_values)),
            _ => None,
        })
        .flat_map(|(property_key, property_values)| {
            property_values.into_iter().map(move |property| {
                let mut property = match property {
                    JValue::Compound(property) => Some(property),
                    _ => None,
                };

                let value = match property
                    .as_mut()
                    .and_then(|property| property.remove("Value"))
                {
                    Some(JValue::String(value)) => value,
                    _ => JavaString::new(),
                };

                let mut new_property = jcompound! {
                    "name" => property_key.clone(),
                    "value" => value,
                };

                if let Some(JValue::String(signature)) = property
                    .as_mut()
                    .and_then(|property| property.remove("Signature"))
                {
                    new_property.insert("signature", signature);
                }

                new_property
            })
        })
        .collect()
}

pub(crate) fn convert_profile(input: JValue) -> JCompound {
    match input {
        JValue::String(name) => {
            if is_valid_player_name(&name) {
                jcompound! {
                    "name" => name,
                }
            } else {
                JCompound::new()
            }
        }
        JValue::Compound(mut input) => {
            let mut ret = JCompound::new();

            let name = match input.remove("Name") {
                Some(JValue::String(name)) => name,
                _ => JavaString::new(),
            };
            if is_valid_player_name(&name) {
                ret.insert("name", name);
            }

            if let Some(id) = input.remove("Id") {
                ret.insert("id", id);
            }

            if let Some(JValue::Compound(properties)) = input.remove("Properties") {
                if !properties.is_empty() {
                    ret.insert(
                        "properties",
                        JList::Compound(convert_properties(properties)),
                    );
                }
            }

            ret
        }
        _ => {
            jcompound! {
                "name" => "",
            }
        }
    }
}

fn convert_skull(item: &mut TransientItemStack) {
    let Some(skull_owner) = item.tag.remove("SkullOwner") else {
        return;
    };

    item.components
        .insert("minecraft:profile", convert_profile(skull_owner));
}

pub(crate) fn convert_item(input: JCompound) -> JCompound {
    if !matches!(input.get("id"), Some(JValue::String(_)))
        || !input.get("Count").is_some_and(|count| count.is_number())
    {
        return input;
    }

    let mut item = TransientItemStack::new(input);

    item.migrate_int_to_component("Damage", "minecraft:damage", 0);
    item.migrate_int_to_component("RepairCost", "minecraft:repair_cost", 0);
    item.migrate_tag_to_component("CustomModelData", "minecraft:custom_model_data");

    if let Some(JValue::Compound(mut block_state_properties)) = item.tag.remove("BlockStateTag") {
        convert_block_state_properties(&mut block_state_properties);
        item.components
            .insert("minecraft:block_state", block_state_properties);
    }

    item.migrate_tag_to_component("EntityTag", "minecraft:entity_data");

    if let Some(JValue::Compound(mut tile_entity_tag)) = item.tag.remove("BlockEntityTag") {
        convert_tile_entity(&mut tile_entity_tag, &mut item);
        if tile_entity_tag.len() > 1
            || (tile_entity_tag.len() == 1 && !tile_entity_tag.contains_key("id"))
        {
            item.components
                .insert("minecraft:block_entity_data", tile_entity_tag);
        }
    }

    let flags = item
        .tag
        .remove("HideFlags")
        .and_then(|o| o.as_i32())
        .unwrap_or(0);

    if item
        .tag
        .remove("Unbreakable")
        .and_then(|o| o.as_i32())
        .is_some_and(|i| i != 0)
    {
        let mut unbreakable = JCompound::new();
        if (flags & TOOLTIP_FLAG_HIDE_UNBREAKABLE) != 0 {
            unbreakable.insert("show_in_tooltip", false);
        }
        item.components.insert("minecraft:unbreakable", unbreakable);
    }

    convert_enchantments(
        &mut item,
        "Enchantments",
        "minecraft:enchantments",
        (flags & TOOLTIP_FLAG_HIDE_ENCHANTMENTS) != 0,
    );

    convert_display(&mut item, flags);
    convert_adventure_mode(&mut item, flags);
    convert_attributes(&mut item, flags);

    if let Some(mut trim) = item.tag.remove("Trim") {
        if (flags & TOOLTIP_FLAG_HIDE_UPGRADES) != 0 {
            if let JValue::Compound(trim) = &mut trim {
                trim.insert("show_in_tooltip", false);
            }
        }

        item.components.insert("minecraft:trim", trim);
    }

    if (flags & TOOLTIP_FLAG_HIDE_ADDITIONAL) != 0 {
        item.components
            .insert("minecraft:hide_additional_tooltip", JCompound::new());
    }

    match item.id.as_bytes() {
        b"minecraft:enchanted_book" => {
            convert_enchantments(
                &mut item,
                "StoredEnchantments",
                "minecraft:stored_enchantments",
                (flags & TOOLTIP_FLAG_HIDE_ADDITIONAL) != 0,
            );
        }
        b"minecraft:crossbow" => {
            item.tag.remove("Charged");
            item.migrate_tag_non_empty_list_to_component(
                "ChargedProjectiles",
                "minecraft:charged_projectiles",
            );
        }
        b"minecraft:bundle" => {
            item.migrate_tag_non_empty_list_to_component("Items", "minecraft:bundle_contents");
        }
        b"minecraft:filled_map" => {
            convert_map(&mut item);
        }
        b"minecraft:potion"
        | b"minecraft:splash_potion"
        | b"minecraft:lingering_potion"
        | b"minecraft:tipped_arrow" => {
            convert_potion(&mut item);
        }
        b"minecraft:writable_book" => {
            convert_writable_book(&mut item);
        }
        b"minecraft:written_book" => {
            convert_written_book(&mut item);
        }
        b"minecraft:suspicious_stew" => {
            item.migrate_tag_to_component("effects", "minecraft:suspicious_stew_effects");
        }
        b"minecraft:debug_stick" => {
            item.migrate_tag_to_component("DebugProperty", "minecraft:debug_stick_state");
        }
        b"minecraft:pufferfish_bucket"
        | b"minecraft:salmon_bucket"
        | b"minecraft:cod_bucket"
        | b"minecraft:tropical_fish_bucket"
        | b"minecraft:axolotl_bucket"
        | b"minecraft:tadpole_bucket" => {
            convert_mob_bucket(&mut item);
        }
        b"minecraft:goat_horn" => {
            item.migrate_tag_to_component("instrument", "minecraft:instrument");
        }
        b"minecraft:knowledge_book" => {
            item.migrate_tag_to_component("Recipes", "minecraft:recipes");
        }
        b"minecraft:compass" => {
            convert_compass(&mut item);
        }
        b"minecraft:firework_rocket" => {
            convert_firework_rocket(&mut item);
        }
        b"minecraft:firework_star" => {
            convert_firework_star(&mut item);
        }
        b"minecraft:player_head" => {
            convert_skull(&mut item);
        }
        _ => {}
    }

    item.serialize()
}

struct TransientItemStack {
    id: JavaString,
    count: i32,
    components: JCompound,
    tag: JCompound,
    root: JCompound,
}

impl TransientItemStack {
    fn new(mut root: JCompound) -> Self {
        let id = root
            .remove("id")
            .and_then(|o| match o {
                JValue::String(id) => Some(id),
                _ => None,
            })
            .unwrap_or_else(|| JavaString::from("minecraft:air"));
        let count = root.remove("Count").and_then(|o| o.as_i32()).unwrap_or(0);
        let tag = root
            .remove("tag")
            .and_then(|o| match o {
                JValue::Compound(tag) => Some(tag),
                _ => None,
            })
            .unwrap_or_default();

        Self {
            id,
            count,
            components: JCompound::new(),
            tag,
            root,
        }
    }

    fn migrate_tag_to(
        &mut self,
        tag_key: &(impl AsRef<JavaStr> + ?Sized),
        dst: &mut JCompound,
        dst_key: impl Into<JavaString>,
    ) {
        if let Some(value) = self.tag.remove(tag_key.as_ref()) {
            dst.insert(dst_key, value);
        }
    }

    fn migrate_tag_to_component(
        &mut self,
        tag_key: &(impl AsRef<JavaStr> + ?Sized),
        component_key: impl Into<JavaString>,
    ) {
        if let Some(value) = self.tag.remove(tag_key.as_ref()) {
            self.components.insert(component_key, value);
        }
    }

    fn migrate_tag_non_empty_list_to_component(
        &mut self,
        tag_key: &(impl AsRef<JavaStr> + ?Sized),
        component_key: impl Into<JavaString>,
    ) {
        if let Some(JValue::List(list)) = self.tag.remove(tag_key.as_ref()) {
            if !list.is_empty() {
                self.components.insert(component_key, list);
            }
        }
    }

    fn migrate_int_to_component(
        &mut self,
        tag_key: &(impl AsRef<JavaStr> + ?Sized),
        component_key: impl Into<JavaString>,
        default_component: i32,
    ) {
        if let Some(int) = self.tag.remove(tag_key.as_ref()).and_then(|o| o.as_i32()) {
            if int != default_component {
                self.components.insert(component_key, int);
            }
        }
    }

    fn serialize(mut self) -> JCompound {
        let mut ret = self.root;
        ret.insert("id", self.id);
        ret.insert("count", self.count);
        if !self.tag.is_empty() {
            self.components.insert("minecraft:custom_data", self.tag);
        }

        if !self.components.is_empty() {
            ret.insert("components", self.components);
        }

        ret
    }
}
