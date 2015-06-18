#![crate_name = "clipboard"]
#![crate_type = "lib"]
#![crate_type = "dylib"]
#![crate_type = "rlib"]

#![cfg_attr(target_os="linux", feature(vec_push_all))]
#![feature(cstr_to_str)]

#[cfg(target_os="linux")]
extern crate libc;
#[cfg(target_os="linux")]
extern crate x11;

#[cfg(target_os="linux")]
mod x11_clipboard;
#[cfg(target_os="linux")]
pub use x11_clipboard::*;

#[cfg(not(target_os="linux"))]
mod nop_clipboard;
#[cfg(not(target_os="linux"))]
pub use nop_clipboard::*;
