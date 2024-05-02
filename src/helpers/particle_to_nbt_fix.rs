use crate::helpers::resource_location::ResourceLocation;
use java_string::{JavaStr, JavaString};
use std::borrow::Cow;
use std::fmt::Display;
use std::str::FromStr;
use tracing::warn;
use valence_nbt::{compound, jcompound, snbt, Value};
use world_transmuter_engine::{compound_to_java, JCompound, JList};

fn parse_nbt(flat: &JavaStr) -> Option<JCompound> {
    fn fail(flat: impl Display, reason: impl Display) -> Option<JCompound> {
        warn!("Failed to parse nbt ({reason}): {flat}");
        None
    }

    match flat.as_str() {
        Ok(flat) => match snbt::from_snbt_str(flat) {
            Ok(Value::Compound(result)) => Some(compound_to_java(result)),
            Ok(_) => fail(flat, "not a compound"),
            Err(err) => fail(flat, err),
        },
        Err(err) => fail(flat, err),
    }
}

fn convert_item(nbt: &mut JCompound, data: &JavaStr) {
    let (item_id, item_tag) = match data.find('{') {
        Some(nbt_start) => (&data[..nbt_start], parse_nbt(&data[nbt_start..])),
        None => (data, None),
    };

    let item_id = ResourceLocation::make_correct(item_id);

    let mut item_nbt = jcompound! {
        "id" => item_id,
        "Count" => 1,
    };
    if let Some(item_tag) = item_tag {
        item_nbt.insert("tag", item_tag);
    }

    nbt.insert("item", item_nbt);
}

fn parse_properties(input: &JavaStr) -> Option<JCompound> {
    fn fail(input: &JavaStr) -> Option<JCompound> {
        warn!("Failed to parse block properties: {input}");
        None
    }

    // format: [p1=v1, p2=v2, p3=v3, ...]
    let Some(mut properties) = input.strip_prefix('[') else {
        return fail(input);
    };

    let mut ret = JCompound::new();

    properties = properties.trim_start();
    if !properties.starts_with(']') {
        while !properties.is_empty() {
            let Some((property, next_properties)) = read_string(properties) else {
                return fail(input);
            };
            properties = next_properties;

            properties = properties.trim_start();
            if !properties.starts_with('=') {
                return fail(input);
            }
            properties = &properties[1..];
            properties = properties.trim_start();

            let Some((value, next_properties)) = read_string(properties) else {
                return fail(input);
            };
            properties = next_properties;

            ret.insert(property, value);

            properties = properties.trim_start();
            if !properties.starts_with(',') {
                // invalid character or ']'
                break;
            }

            // skip ',' and move onto next entry
            properties = &properties[1..];
            properties = properties.trim_start();
        }
    }

    if !properties.starts_with(']') {
        return fail(input);
    }

    Some(ret)
}

fn convert_block(nbt: &mut JCompound, data: &JavaStr) {
    let (block_id, block_properties) = match data.find('[') {
        Some(props_start) => (&data[..props_start], parse_properties(&data[props_start..])),
        None => (data, None),
    };

    let block_id = ResourceLocation::make_correct(block_id);

    let mut block_nbt = jcompound! {
        "Name" => block_id,
    };
    if let Some(block_properties) = block_properties {
        block_nbt.insert("tag", block_properties);
    }

    nbt.insert("block_state", block_nbt);
}

fn parse_float_vector(input: &JavaStr) -> Option<(JList, &JavaStr)> {
    let (x, input) = read_number::<f32>(input)?;
    let input = input.strip_prefix(' ')?;
    let (y, input) = read_number::<f32>(input)?;
    let input = input.strip_prefix(' ')?;
    let (z, input) = read_number::<f32>(input)?;

    Some((JList::Float(vec![x, y, z]), input))
}

fn convert_dust(nbt: &mut JCompound, data: &JavaStr) {
    fn fail(data: &JavaStr) {
        warn!("Failed to parse dust particle: {data}");
    }

    let reader = data;
    let Some((color, reader)) = parse_float_vector(reader) else {
        return fail(data);
    };

    let Some(reader) = reader.strip_prefix(' ') else {
        return fail(data);
    };

    let Some((scale, _)) = read_number::<f32>(reader) else {
        return fail(data);
    };

    nbt.insert("color", color);
    nbt.insert("scale", scale);
}

fn convert_color_dust(nbt: &mut JCompound, data: &JavaStr) {
    fn fail(data: &JavaStr) {
        warn!("Failed to parse color transition dust particle: {data}");
    }

    let reader = data;

    let Some((from_color, reader)) = parse_float_vector(reader) else {
        return fail(data);
    };

    let Some(reader) = reader.strip_prefix(' ') else {
        return fail(data);
    };
    let Some((scale, reader)) = read_number::<f32>(reader) else {
        return fail(data);
    };

    let Some(reader) = reader.strip_prefix(' ') else {
        return fail(data);
    };
    let Some((to_color, _)) = parse_float_vector(reader) else {
        return fail(data);
    };

    nbt.insert("from_color", from_color);
    nbt.insert("scale", scale);
    nbt.insert("to_color", to_color);
}

fn convert_sculk(nbt: &mut JCompound, data: &JavaStr) {
    let Some((roll, _)) = read_number::<f32>(data) else {
        warn!("Failed to parse sculk particle: {data}");
        return;
    };

    nbt.insert("roll", roll);
}

fn convert_vibration(nbt: &mut JCompound, data: &JavaStr) {
    fn fail(data: &JavaStr) {
        warn!("Failed to parse vibration particle: {data}");
    }

    let reader = data;

    let Some((pos_x, reader)) = read_number::<f64>(reader) else {
        return fail(data);
    };

    let Some(reader) = reader.strip_prefix(' ') else {
        return fail(data);
    };
    let Some((pos_y, reader)) = read_number::<f64>(reader) else {
        return fail(data);
    };

    let Some(reader) = reader.strip_prefix(' ') else {
        return fail(data);
    };
    let Some((pos_z, reader)) = read_number::<f64>(reader) else {
        return fail(data);
    };

    let Some(reader) = reader.strip_prefix(' ') else {
        return fail(data);
    };
    let Some((arrival, _)) = read_number::<i32>(reader) else {
        return fail(data);
    };

    nbt.insert("arrival_in_ticks", arrival);
    nbt.insert(
        "destination",
        jcompound! {
            "type" => "minecraft:block",
            "pos" => JList::Double(vec![pos_x, pos_y, pos_z]),
        },
    );
}

fn convert_shriek(nbt: &mut JCompound, data: &JavaStr) {
    let Some((delay, _)) = read_number::<i32>(data) else {
        warn!("Failed to read shriek particle: {data}");
        return;
    };

    nbt.insert("delay", delay);
}

pub(crate) fn convert(flat: &JavaStr) -> JCompound {
    let mut split = flat.splitn(2, ' ');
    let name = ResourceLocation::make_correct(split.next().unwrap());

    let mut ret = JCompound::new();

    if let Some(data) = split.next() {
        match name.as_bytes() {
            b"minecraft:item" => convert_item(&mut ret, data),
            b"minecraft:block"
            | b"minecraft:block_marker"
            | b"minecraft:falling_dust"
            | b"minecraft:dust_pillar" => convert_block(&mut ret, data),
            b"minecraft:dust" => convert_dust(&mut ret, data),
            b"minecraft:dust_color_transition" => convert_color_dust(&mut ret, data),
            b"minecraft:sculk_charge" => convert_sculk(&mut ret, data),
            b"minecraft:vibration" => convert_vibration(&mut ret, data),
            b"minecraft:shriek" => convert_shriek(&mut ret, data),
            _ => {}
        }
    }

    ret.insert("type", name);

    ret
}

/// A function to read a float that matches brigadier's functions to read numbers
fn read_number<T: FromStr>(input: &JavaStr) -> Option<(T, &JavaStr)> {
    let end_index = input
        .find(|char| !('0'..='9').contains(&char) && char != '.' && char != '-')
        .unwrap_or_else(|| input.len());
    Some((input[..end_index].parse().ok()?, &input[end_index..]))
}

/// A function to read a possibly quoted string that matches brigadier's `readString` function
fn read_string(input: &JavaStr) -> Option<(Cow<JavaStr>, &JavaStr)> {
    fn read_quoted_string(mut input: &JavaStr, quote: char) -> Option<(Cow<JavaStr>, &JavaStr)> {
        input = &input[1..];

        // check if there is any need to allocate a new string
        let Some(end_quote_index) = input.find(quote) else {
            return None;
        };
        if !input
            .find('\\')
            .is_some_and(|backslash_index| backslash_index < end_quote_index)
        {
            return Some((
                Cow::Borrowed(&input[..end_quote_index]),
                &input[end_quote_index + 1..],
            ));
        }

        let mut result = JavaString::new();
        let mut escaped = false;
        for (index, char) in input.char_indices() {
            if escaped {
                if char == quote || char == '\\' {
                    result.push_java(char);
                    escaped = false;
                } else {
                    return None;
                }
            } else if char == '\\' {
                escaped = true;
            } else if char == quote {
                return Some((Cow::Owned(result), &input[index + 1..]));
            } else {
                result.push_java(char);
            }
        }

        None
    }

    if input.starts_with('"') {
        read_quoted_string(input, '"')
    } else if input.starts_with('\'') {
        read_quoted_string(input, '\'')
    } else {
        let end_index = input
            .find(|char| {
                !('0'..='9').contains(&char)
                    && !('A'..='Z').contains(&char)
                    && !('a'..='z').contains(&char)
                    && char != '_'
                    && char != '-'
                    && char != '.'
                    && char != '+'
            })
            .unwrap_or_else(|| input.len());
        Some((Cow::Borrowed(&input[..end_index]), &input[end_index..]))
    }
}
