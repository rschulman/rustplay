#![no_std]
#![allow_ctypes]

extern crate core;

mod video;

#[lang="start"]
#[no_mangle]
#[no_split_stack]
pub fn kernel_main() {
  video::clear_screen(video::Red);
}
