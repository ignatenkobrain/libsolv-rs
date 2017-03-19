#![allow(bad_style)]

extern crate libc;
extern crate libsolv_sys;

use libc::*;
use libsolv_sys::*;

include!(concat!(env!("OUT_DIR"), "/all.rs"));
