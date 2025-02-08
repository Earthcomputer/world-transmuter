#![allow(clippy::collapsible_else_if)]
#![allow(unused)]

use crate::version_names::get_breakpoints;
use world_transmuter_engine::{
    AbstractDynamicDataType, AbstractMapDataType, AbstractValueDataType, DataVersion, JCompound,
    JValue, JValueMut,
};

mod helpers;
pub mod types;
pub mod version_names;
mod versions;

pub mod json {
    pub use crate::helpers::json_parser::*;
}

pub fn convert_map(
    typ: impl AbstractMapDataType,
    data: &mut JCompound,
    from_version: impl Into<DataVersion>,
    to_version: impl Into<DataVersion>,
) {
    iterate_breakpoints(from_version, to_version, |from, to| {
        typ.convert(data, from, to)
    });
}

pub fn convert_value(
    typ: impl AbstractValueDataType,
    data: &mut JValueMut,
    from_version: impl Into<DataVersion>,
    to_version: impl Into<DataVersion>,
) {
    iterate_breakpoints(from_version, to_version, |from, to| {
        typ.convert(data, from, to)
    });
}

pub fn convert_dyn(
    typ: impl AbstractDynamicDataType,
    data: &mut JValue,
    from_version: impl Into<DataVersion>,
    to_version: impl Into<DataVersion>,
) {
    iterate_breakpoints(from_version, to_version, |from, to| {
        typ.convert(data, from, to)
    });
}

fn iterate_breakpoints(
    from_version: impl Into<DataVersion>,
    to_version: impl Into<DataVersion>,
    mut f: impl FnMut(DataVersion, DataVersion),
) {
    let mut from_version = from_version.into();
    let to_version = to_version.into();
    let breakpoints = get_breakpoints();
    let mut breakpoint_index = breakpoints
        .binary_search(&from_version)
        .map(|i| i + 1)
        .unwrap_or_else(|i| i);
    while breakpoint_index < breakpoints.len() && breakpoints[breakpoint_index] < to_version {
        let breakpoint = breakpoints[breakpoint_index];
        f(from_version, breakpoint);
        from_version = breakpoint;
        breakpoint_index += 1;
    }
    f(from_version, to_version);
}

#[cfg(test)]
mod tests {}
