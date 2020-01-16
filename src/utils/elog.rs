#![macro_use]
#![allow(non_snake_case)]

use crate::setjmp::sigjmp_buf;

use libc::*;
pub use pgsys::{elog_finish, elog_start, errfinish, errstart, pg_re_throw};
pub use pgsys::{errcode, errdetail, errhint, errmsg};
pub use pgsys::{
    DEBUG1, DEBUG2, DEBUG3, DEBUG4, DEBUG5, ERROR, FATAL, INFO, LOG, NOTICE, PANIC, WARNING,
};

// TODO: Don't re-export these, currently they are used by longjmp_panic!
pub use pgsys::{error_context_stack, ErrorContextCallback};

#[macro_export]
macro_rules! elog {
    ($elevel:expr, $($args:expr),+) => {
        ereport!($elevel, errmsg($($args),+));
    };
}

#[macro_export]
macro_rules! ereport {
    ($elevel:expr, $($kind:tt($($args:expr),*)),+$(,)?) => {
        unsafe {
            use postgres_extension::utils::elog;
            use postgres_extension::rust_utils::PanicType;

            if elog::pg_errstart($elevel as i32, file!(), line!()) {

                $(
                    pg_errfmt!($kind,$($args),+);
                )+

                if $elevel >= elog::ERROR {
                    panic!(PanicType::Errfinish);
                } else {
                    elog::errfinish(0);
                }
            }
        }
    }
}

#[macro_export]
macro_rules! pg_errfmt {
    (errcode, $arg:expr) => {
        errcode($arg);
    };
    ($kind:tt, $($args:expr),+) => {
        let s: &str = &format!($($args),+);
        let cstring = std::ffi::CString::new(s).unwrap();
        $kind(cstring.as_ptr());
    }
}

pub unsafe fn pg_errstart(elevel: i32, _filename: &str, lineno: u32) -> bool {
    //TODO: find a way to make a constant c string out of file!()
    let cfilename = std::ptr::null::<c_char>();
    let clineno = lineno as c_int;
    let cfuncname = std::ptr::null::<c_char>();
    let cdomain = std::ptr::null::<c_char>();

    return errstart(elevel, cfilename, clineno, cfuncname, cdomain);
}

pub const TEXTDOMAIN: *const c_char = std::ptr::null::<c_char>();

const fn pgsixbit(ch: char) -> u32 {
    return ((ch as u32) - ('0' as u32)) & 0x3f;
}
const fn make_sqlstate(ch1: char, ch2: char, ch3: char, ch4: char, ch5: char) -> i32 {
    return ((pgsixbit(ch1) << 0)
        + (pgsixbit(ch2) << 6)
        + (pgsixbit(ch3) << 12)
        + (pgsixbit(ch4) << 18)
        + (pgsixbit(ch5) << 24)) as i32;
}

pub const ERRCODE_EXTERNAL_ROUTINE_EXCEPTION: c_int = make_sqlstate('3', '8', '0', '0', '0');

pub const ERRCODE_FEATURE_NOT_SUPPORTED: c_int = make_sqlstate('0', 'A', '0', '0', '0');

extern "C" {
    pub static mut PG_exception_stack: *mut sigjmp_buf;
}
