// the Burning Question: is it inbounds?

#include <llvm/IR/Instructions.h>
#include <llvm-c/Target.h>
#include <stdbool.h>

extern "C" {
bool is_gep_in_bounds(llvm::GetElementPtrInst *v) {
    return v->isInBounds();
}

void do_LLVMInitializeAllTargetInfos() {
    return LLVMInitializeAllTargetInfos();
}
void do_LLVMInitializeAllTargets() {
    return LLVMInitializeAllTargets();
}
void do_LLVMInitializeAllTargetMCs() {
    return LLVMInitializeAllTargetMCs();
}
void do_LLVMInitializeAllAsmPrinters() {
    return LLVMInitializeAllAsmPrinters();
}
void do_LLVMInitializeAllAsmParsers() {
    return LLVMInitializeAllAsmParsers();
}
void do_LLVMInitializeAllDisassemblers() {
    return LLVMInitializeAllDisassemblers();
}
bool do_LLVMInitializeNativeTarget() {
    return LLVMInitializeNativeTarget();
}
bool do_LLVMInitializeNativeAsmParser() {
    return LLVMInitializeNativeAsmParser();
}
bool do_LLVMInitializeNativeAsmPrinter() {
    return LLVMInitializeNativeAsmPrinter();
}
bool do_LLVMInitializeNativeDisassembler() {
    return LLVMInitializeNativeDisassembler();
}
}
