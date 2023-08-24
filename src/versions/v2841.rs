use valence_nbt::{Compound, Value};

pub fn get_block_id(state: Option<&Compound>) -> &str {
    if let Some(state) = state {
        if let Some(Value::String(name)) = state.get("Name") {
            return &name[..];
        }
    }
    "minecraft:air"
}
