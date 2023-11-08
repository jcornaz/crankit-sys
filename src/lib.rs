#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#![no_std]

//! Generated bindings for the [playdate C API](https://sdk.play.date/inside-playdate-with-c)
//!
//! ## Status: Discontinued
//!
//! This project is discontinued in favor of [`playdate-sys`](https://crates.io/crates/playdate-sys)
//!
//! # Requirements
//!
//! The bindings are generated at build-time and require:
//!
//! * [Playdate SDK](https://play.date/dev/#cardSDK) must be installed
//! * An environment variable `PLAYDATE_SDK_PATH` must point to the playdate SDK installation folder
//! * `clang` must be installed

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[no_mangle]
#[doc(hidden)]
#[cfg(not(test))]
extern "C" fn rust_eh_personality() {
    unimplemented!()
}
