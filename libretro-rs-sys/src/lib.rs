#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(clippy::redundant_static_lifetimes)]

pub use libc;

include!(concat!(env!("OUT_DIR"), "/libretro.rs"));
