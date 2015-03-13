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

extern "C" {
    pub fn is_gep_in_bounds(v: LLVMValueRef) -> bool;
    fn do_LLVMInitializeAllTargetInfos();
    fn do_LLVMInitializeAllTargets();
    fn do_LLVMInitializeAllTargetMCs();
    fn do_LLVMInitializeAllAsmPrinters();
    fn do_LLVMInitializeAllAsmParsers();
    fn do_LLVMInitializeAllDisassemblers();
    fn do_LLVMInitializeNativeTarget() -> bool;
    fn do_LLVMInitializeNativeAsmParser() -> bool;
    fn do_LLVMInitializeNativeAsmPrinter() -> bool;
    fn do_LLVMInitializeNativeDisassembler() -> bool;
}
pub unsafe fn LLVMInitializeAllTargetInfos() { do_LLVMInitializeAllTargetInfos() }
pub unsafe fn LLVMInitializeAllTargets() { do_LLVMInitializeAllTargets() }
pub unsafe fn LLVMInitializeAllAsmPrinters() { do_LLVMInitializeAllAsmPrinters() }
pub unsafe fn LLVMInitializeAllDisassemblers() { do_LLVMInitializeAllDisassemblers() }
pub unsafe fn LLVMInitializeAllTargetMCs() { do_LLVMInitializeAllTargetMCs() }
pub unsafe fn LLVMInitializeAllAsmParsers() { do_LLVMInitializeAllAsmParsers() }
pub unsafe fn LLVMInitializeNativeTarget() -> bool { do_LLVMInitializeNativeTarget() }
pub unsafe fn LLVMInitializeNativeAsmParser() -> bool { do_LLVMInitializeNativeAsmParser() }
pub unsafe fn LLVMInitializeNativeAsmPrinter() -> bool { do_LLVMInitializeNativeAsmPrinter() }
pub unsafe fn LLVMInitializeNativeDisassembler() -> bool { do_LLVMInitializeNativeDisassembler() }

include!(concat!(env!("OUT_DIR"), "/temp.rs"));
