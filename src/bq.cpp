// the Burning Question: is it inbounds?

#include <llvm/IR/Instructions.h>

extern "C" {
bool is_gep_in_bounds(llvm::GetElementPtrInst *v) {
    return v->isInBounds();
}
}
