use crate::versions::v100;
use java_string::JavaString;

const VERSION: u32 = 501;

pub(crate) fn register() {
    register_mob("PolarBear");
}

fn register_mob(id: impl Into<JavaString>) {
    v100::register_equipment(VERSION, id);
}
