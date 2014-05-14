#![no_std]
#![allow_ctypes]
#![no_main]

extern crate core;

mod video;

extern "rust-intrinsic" { fn abort() -> !; }

#[no_mangle]
pub extern fn rust_stack_exhausted() {
  unsafe { abort() }
}

#[no_mangle]
pub extern "C" fn kernel_main () {
  unsafe {video::clear_screen(video::Red);}
}
