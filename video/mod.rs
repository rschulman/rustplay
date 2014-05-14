extern crate core;
use core::option::{Some, None};
use core::iter::Iterator;

pub enum Color {
    Black       = 0,
    Blue        = 1,
    Green       = 2,
    Cyan        = 3,
    Red         = 4,
    Pink        = 5,
    Brown       = 6,
    LightGray   = 7,
    DarkGray    = 8,
    LightBlue   = 9,
    LightGreen  = 10,
    LightCyan   = 11,
    LightRed    = 12,
    LightPink   = 13,
    Yellow      = 14,
    White       = 15,
}

static VGAADDR: uint = 0xB8000;

let mut terminal_row: uint = 0;
let mut terminal_column: uint = 0;
let mut terminal_color: Color = Black;

pub fn terminal_setcolor(fg: Color) {
  terminal_color = fg;
}

fn make_color(fg:Color, bg: Color) -> u8 {
  fg as u8 | bg as u8 << 4
}

fn make_vgaentry(c: char, color: u8) -> u16 {
  c as u16 | color as u16 << 8
}

// No strlen required? Use ::core::str::len

pub unsafe fn clear_screen (background: Color) {
  for i in ::core::iter::range(0, 80*25) {
    *((VGAADDR + (i as uint) * 2) as *mut u16) = (background as u16) << 12;
  }
}

fn terminal_putentryat(c: char, fg: Color, x: u8, y: u8) {
  let index: u8 = y * 80 + x;
  unsafe { *((VGAADDR + i) * 2) as *mut u16) = make_vgaentry(c, make_color(fg, terminal_color)); }
}
