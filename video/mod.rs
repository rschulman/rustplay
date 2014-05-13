//use core;

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

fn make_color(fg:Color, bg: Color) -> u8 {
  fg | bg << 4;
}

fn make_vgaentry(c: char, color: u8) -> u16 {
  c as u16 | color as u16 << 8;
}

// No strlen required? Use ::core::str::len

unsafe fn clear_screen (background: Color) {
  for i in range(0, 80*25) {
    *((VGAADDR + i *2) as *mut u16) = (background as u16) << 12;
  }
}
