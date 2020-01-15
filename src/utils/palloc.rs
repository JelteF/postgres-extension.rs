use crate::utils::memutils::{CurrentMemoryContext, MemoryContext};

pub unsafe fn MemoryContextSwitchTo(context: MemoryContext) -> MemoryContext {
    let old = CurrentMemoryContext;
    CurrentMemoryContext = context;
    return old;
}
