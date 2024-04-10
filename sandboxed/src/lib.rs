#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::sync::Mutex;

pub struct Sandboxed(mpk::Sandbox);
pub static SANDBOXED: Mutex<Sandboxed> = Mutex::new(Sandboxed(mpk::Sandbox::new()));

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
