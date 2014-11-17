use gen as al;
pub use gen::{LLVMValueRef, LLVMModuleRef, LLVMBasicBlockRef, LLVMUseRef, LLVMTargetDataRef, LLVMTypeRef, LLVMBuilderRef};

use std::{mem, c_str, ptr};
use libc;

unsafe fn from_cstr(s: &*const libc::c_char) -> &str {
    mem::transmute(c_str::CString::new(*s as *const i8, false).as_str().unwrap())
}

pub struct LLVMMessage(*mut i8);
impl LLVMMessage {
    pub unsafe fn get(&self) -> &str { mem::transmute(from_cstr(&(self.0 as *const i8))) }
}
impl Drop for LLVMMessage {
    fn drop(&mut self) { unsafe { al::LLVMDisposeMessage(self.0); } }
}


macro_rules! llvm_iter { ($inner:ty, $container:ty, $itername:ident, $first:expr, $last:expr, $next:expr) => {
    pub struct $itername { inr: $inner, end: $inner }
    impl $itername {
        pub unsafe fn new(f: $container) -> $itername {
            $itername { inr: $first(f), end: $last(f) }
        }
    }
    impl Iterator<$inner> for $itername {
        fn next(&mut self) -> Option<$inner> {
            if self.inr == ptr::null_mut() {
                None
            } else {
                let inr = self.inr;
                self.inr = if inr == self.end { ptr::null_mut() } else { unsafe { $next(inr) } };
                Some(inr)
            }
        }
    }
} }
llvm_iter!(LLVMBasicBlockRef, LLVMValueRef, BBIter, al::LLVMGetFirstBasicBlock, al::LLVMGetLastBasicBlock, al::LLVMGetNextBasicBlock)
//llvm_iter!(LLVMValueRef, LLVMModuleRef, GlobalIter, al::LLVMGetFirstGlobal, al::LLVMGetLastGlobal, al::LLVMGetNextGlobal)
//llvm_iter!(LLVMValueRef, LLVMModuleRef, FunctionIter, al::LLVMGetFirstFunction, al::LLVMGetLastFunction, al::LLVMGetNextFunction)
llvm_iter!(LLVMValueRef, LLVMValueRef, ParamIter, al::LLVMGetFirstParam, al::LLVMGetLastParam, al::LLVMGetNextParam)
llvm_iter!(LLVMValueRef, LLVMBasicBlockRef, InstIter, al::LLVMGetFirstInstruction, al::LLVMGetLastInstruction, al::LLVMGetNextInstruction)

pub struct UseIter { inr: LLVMUseRef }
impl UseIter {
    pub unsafe fn new(v: LLVMValueRef) -> UseIter {
        UseIter { inr: al::LLVMGetFirstUse(v) }
    }
}
impl Iterator<LLVMUseRef> for UseIter {
    fn next(&mut self) -> Option<LLVMUseRef> {
        if self.inr == ptr::null_mut() {
            None
        } else {
            let inr = self.inr;
            self.inr = unsafe { al::LLVMGetNextUse(inr) };
            Some(inr)
        }
    }
}

pub struct OperandIter { v: LLVMValueRef, i: uint, cnt: uint }
impl OperandIter {
    pub unsafe fn new(v: LLVMValueRef) -> OperandIter {
        OperandIter { v: v, i: 0, cnt: al::LLVMGetNumOperands(v) as uint }
    }
}
impl Iterator<LLVMValueRef> for OperandIter {
    fn next(&mut self) -> Option<LLVMValueRef> {
        if self.i == self.cnt {
            None
        } else {
            self.i += 1;
            Some(unsafe { al::LLVMGetOperand(self.v, (self.i - 1) as u32) })
        }
    }
}

pub unsafe fn get_function_insn_descs(f: LLVMValueRef) -> Vec<String> {
    // better ways to do this aren't even public C++ api.
    let msg = value_msg(f);
    let mut results = vec!();
    let mut multiline_str = String::new();
    let mut multiline = false;
    for line in msg.get().split('\n') {
        if line.starts_with(";") ||
           line.starts_with("define ") ||
           line.starts_with("}") ||
           line.is_empty() {
           continue;
        }
        if multiline && line.ends_with("]") {
            multiline_str.push_str(line);
            results.push(multiline_str);
            multiline = false;
            multiline_str = String::new();
        } else if multiline || line.ends_with("[") {
            multiline = true;
            multiline_str.push_str(line);
            multiline_str.push('\n');
        } else {
            results.push(line.into_string());
        }
    }
    /*
    for s in results.iter() {
        println!("<<{}>>", s);
    }
    */
    results
}

#[allow(unused)]
unsafe fn test_get_function_insn_descs(f: LLVMValueRef) {
    let descs = get_function_insn_descs(f);
    let mut it = descs.iter();
    for bb in BBIter::new(f) {
        for inst in InstIter::new(bb) {
            let a = value_msg(inst);
            let b = it.next().unwrap();
            println!("A: {}\nB: {}", a.get(), b);
        }
    }
    for rest in it {
        println!("REMAINING: {}", rest);
    }
}

// don't use this.
pub unsafe fn value_msg(v: LLVMValueRef) -> LLVMMessage {
    LLVMMessage(al::LLVMPrintValueToString(v))
}

