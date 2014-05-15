extern crate core;
use core::option::{Some, None};
use core::iter::Iterator;
use core::str::Chars;

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
static VGAHEIGHT: u8 = 24;
static VGAWIDTH: u8 = 80;

static mut terminal_row: u8 = 0;
static mut terminal_column: u8 = 0;
static mut terminal_color: Color = Black;

pub unsafe fn terminal_setcolor(fg: Color) {
  terminal_color = fg;
}

fn make_color(fg:Color, bg: Color) -> u8 {
  fg as u8 | bg as u8 << 4
}

fn make_vgaentry(c: char, color: u8) -> u16 {
  c as u16 | color as u16 << 8
}

pub unsafe fn clear_screen (background: Color) {
  for i in ::core::iter::range(0, VGAWIDTH*VGAHEIGHT) {
    *((VGAADDR + (i as uint) * 2) as *mut u16) = (background as u16) << 12;
  }
}

fn terminal_putentryat(c: char, fg: Color, x: u8, y: u8) {
  let index: u8 = y * VGAWIDTH + x;
  unsafe { *(((VGAADDR + (index as uint)) * 2) as *mut u16) = make_vgaentry(c, make_color(fg, terminal_color)); }
}

fn terminal_push_row() {
  for y in ::core::iter::range(0, VGAHEIGHT) {
    for x in ::core::iter::range(0, VGAWIDTH) {
      let index = y * VGAWIDTH + x;
      let new_index = index + VGAWIDTH;
      unsafe { *(((VGAADDR + (index as uint)) * 2) as *mut u16) = *(((VGAADDR + (new_index as uint)) * 2) as *mut u16); }
    }
  }
}

unsafe fn terminal_putchar(c: char) {
  if (c == '\n') {
    terminal_column = 0;
    terminal_row += 1;
    if (terminal_row == VGAHEIGHT) { // We have to move everything up...
      terminal_push_row();
      terminal_row -= VGAHEIGHT;
    }
  } else {
    terminal_putentryat(c, White, terminal_row, terminal_column);
    terminal_column += 1;
    if (terminal_column == VGAWIDTH) { // Have to push to the next row...
      terminal_column = 0;
      terminal_row += 1;
      if (terminal_row == VGAHEIGHT) {
        terminal_push_row();
        terminal_row -= VGAHEIGHT;
      }
    }
  }
}

fn print(data: ~str) {
  for c in data.Chars {
    terminal_putchar(c);
  }
}
