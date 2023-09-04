#![allow(clippy::collapsible_else_if)]

mod helpers;
mod types;
pub mod version_names;
mod versions;

pub use types::*;

pub mod json {
    pub use crate::helpers::json_parser::*;
}

#[cfg(test)]
mod tests {}
