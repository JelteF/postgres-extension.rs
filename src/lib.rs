#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

// dependencies
extern crate libc;
extern crate postgres_sys as pgsys;

// rust modules
#[macro_use]
pub mod rust_utils;
pub mod setjmp;

// PG modules
pub mod access;
pub mod c;
pub mod executor;
pub mod fmgr;
pub mod pg_config;
pub mod postgres;
pub mod postgres_ext;
pub mod postmaster;
pub mod utils;

#[global_allocator]
static ALLOCATOR: rust_utils::PostgresAllocator = rust_utils::PostgresAllocator;
