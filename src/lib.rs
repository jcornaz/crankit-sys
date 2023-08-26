#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#![no_std]

//! Generated bindings for the [playdate C API](https://sdk.play.date/inside-playdate-with-c)
//!
//! # Requirements
//!
//! The bindings are generated at build-time and require:
//!
//! * [Playdate SDK](https://play.date/dev/#cardSDK) must be installed
//! * An environment variable `PLAYDATE_SDK_PATH` must point to the playdate SDK installation folder
//! * `clang` must be installed

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
