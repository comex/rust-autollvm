#![feature(link_args)]
#![feature(libc)]
extern crate libc;
pub use gen::*;
mod gen;
pub mod util;

