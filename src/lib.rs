#![feature(const_mut_refs)]
#![feature(explicit_generic_args_with_impl_trait)]
#![feature(int_log)]
#![feature(iter_intersperse)]
#![feature(maybe_uninit_array_assume_init)]
#![feature(maybe_uninit_uninit_array)]
#![feature(once_cell)]
#![feature(try_blocks)]

mod types;
mod versions;
mod helpers;

pub use types::*;

#[cfg(test)]
mod tests {

}
