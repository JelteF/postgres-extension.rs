use libc::*;
pub use pgsys::{
    CurrentMemoryContext, ErrorContext, MemoryContext, MemoryContextData, TopMemoryContext,
    ALLOCSET_DEFAULT_INITSIZE, ALLOCSET_DEFAULT_MAXSIZE, ALLOCSET_DEFAULT_MINSIZE,
};

pub mod c {
    use crate::utils::memutils::*;
    extern "C" {
        pub fn MemoryContextAlloc(context: MemoryContext, size: usize) -> *mut u8;
        pub fn MemoryContextReset(context: MemoryContext);
        pub fn AllocSetContextCreateInternal(
            parent: MemoryContext,
            name: *const c_char,
            minContextSize: size_t,
            initBlockSize: size_t,
            maxBlockSize: size_t,
        ) -> MemoryContext;
        pub fn pfree(ptr: *mut u8) -> ();
    }
}
