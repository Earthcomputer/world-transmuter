use java_string::JavaStr;
use world_transmuter_engine::{JCompound, JValue};

pub(crate) fn flatten_block_pos(data: &mut JCompound, path: &(impl AsRef<JavaStr> + ?Sized)) {
    let Some(pos) = data.get_mut(path.as_ref()) else {
        return;
    };

    let JValue::Compound(pos_compound) = pos else {
        return;
    };

    let Some(x) = pos_compound.get("X").and_then(|o| o.as_i32()) else {
        return;
    };
    let Some(y) = pos_compound.get("Y").and_then(|o| o.as_i32()) else {
        return;
    };
    let Some(z) = pos_compound.get("Z").and_then(|o| o.as_i32()) else {
        return;
    };

    *pos = JValue::IntArray(vec![x, y, z]);
}
