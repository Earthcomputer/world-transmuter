use crate::helpers::bit_storage::ceil_log2;
use crate::MinecraftTypesMut;
use rust_dataconverter_engine::map_data_converter_func;
use valence_nbt::{List, Value};

const VERSION: u32 = 2527;

pub(crate) fn register(types: MinecraftTypesMut) {
    types.chunk().borrow_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            let Some(Value::Compound(level)) = data.get_mut("Level") else {
                return;
            };
            if let Some(Value::List(List::Compound(sections))) = level.get_mut("Sections") {
                for section in sections.iter_mut() {
                    if let Some(Value::Compound(palette)) = section.get("Palette") {
                        let bits = 4.max(ceil_log2(palette.len() as u32));
                        if bits.is_power_of_two() {
                            // fits perfectly
                            continue;
                        }
                        if let Some(Value::LongArray(states)) = section.get_mut("BlockStates") {
                            let new_states = add_padding(4096, bits as usize, states);
                            *states = new_states;
                        }
                    }
                }
            }

            if let Some(Value::Compound(heightmaps)) = level.get_mut("Heightmaps") {
                for heightmap in heightmaps.values_mut() {
                    if let Value::LongArray(heightmap) = heightmap {
                        let new_heightmap = add_padding(256, 9, heightmap);
                        *heightmap = new_heightmap;
                    }
                }
            }
        }),
    );
}

// Assumes that bits is *not* a power of 2!
fn add_padding(size: usize, bits: usize, old: &[i64]) -> Vec<i64> {
    let old_len = old.len();
    if old_len == 0 {
        return Vec::new();
    }

    let mask = (1i64 << bits) - 1;
    let values_per_long = 64 / bits;
    let new_len = (size + values_per_long - 1) / values_per_long;
    let mut padded = vec![0i64; new_len];
    let mut new_word_index = 0;
    let mut used_bits = 0;
    let mut new_word = 0;
    let mut prev_old_word_index = 0;
    let mut old_word = old[0];
    let mut old_next_word = if old_len > 1 { old[1] } else { 0 };

    for index in 0..size {
        let old_bit_index = index * bits;
        let old_word_index = old_bit_index >> 6;
        let old_end_word_index = ((index + 1) * bits - 1) >> 6;
        let old_index_in_word = old_bit_index ^ (old_word_index << 6);
        if old_word_index != prev_old_word_index {
            old_word = old_next_word;
            old_next_word = if old_word_index + 1 < old_len {
                old[old_word_index + 1]
            } else {
                0
            };
            prev_old_word_index = old_word_index;
        }

        let value = if old_word_index == old_end_word_index {
            ((old_word as u64) >> old_index_in_word) as i64 & mask
        } else {
            let first_bits = 64 - old_index_in_word;
            (((old_word as u64) >> old_index_in_word) as i64 | old_next_word << first_bits) & mask
        };

        let next_used_bits = used_bits + bits;
        if next_used_bits >= 64 {
            padded[new_word_index] = new_word;
            new_word_index += 1;
            new_word = value;
            used_bits = bits;
        } else {
            new_word |= value << used_bits;
            used_bits = next_used_bits;
        }
    }

    if new_word != 0 {
        padded[new_word_index] = new_word;
    }

    padded
}
