#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(non_snake_case)]

use libc::{size_t,uintptr_t,off_t};

pub type int64_t  = i64;
pub type int32_t  = i32;
pub type int16_t  = i16;
pub type int8_t   = i8;
pub type uint64_t = u64;
pub type uint32_t = u32;
pub type uint16_t = u16;
pub type uint8_t  = u8;

#[link(name="bq")]
extern "C" {
    pub fn is_gep_in_bounds(v: LLVMValueRef) -> bool;
}

include!(concat!(env!("OUT_DIR"), "/link_args.rs"))
include!(concat!(env!("OUT_DIR"), "/temp.rs"))
