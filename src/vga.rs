// Copyright (c) 2013 John Grosen
// Licensed under the terms described in the LICENSE file in the root of this repository

use util;
use term::Terminal;
use core::str;

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

pub static VGA_WIDTH: uint = 80;
pub static VGA_HEIGHT: uint = 25;

fn get_entry(i: u16) -> u16 {
    if i < (VGA_WIDTH * VGA_HEIGHT) as u16 {
        unsafe {
            *((0xb8000u + (2*i as uint)) as *u16)
        }
    } else {
        0
    }
}

fn set_entry(i: u16, entry: u16) {
    if i < (VGA_WIDTH * VGA_HEIGHT) as u16 {
        unsafe {
            *((0xb8000u + (2*i as uint)) as *mut u16) = entry;
        }
    }
}

pub fn set_char(x: u8, y: u8, chr: char, fg_color: Color, bg_color: Color) {
    if (x as uint) < VGA_WIDTH && (y as uint) < VGA_HEIGHT {
        let colored_char: u16 = (bg_color as u16) << 12 | (fg_color as u16) << 8 | (chr as u16);
        set_entry(((x as uint) + (y as uint) * VGA_WIDTH) as u16, colored_char);
    }
}

pub fn clear_screen(background: Color) {
    do util::range(0, VGA_WIDTH * VGA_HEIGHT) |i| {
        set_entry(i as u16, (background as u16) << 12);
    }
}

struct VgaTerm {
    x: u8,
    y: u8,
    fg_color: Color,
    bg_color: Color,
    displaying: bool,
    buffer: [u16, ..(VGA_WIDTH * VGA_HEIGHT)]
}

impl VgaTerm {
    pub fn new(fg_color: Color, bg_color: Color, displaying: bool) -> VgaTerm {
        if displaying {
            clear_screen(bg_color);
        }
        VgaTerm {
            x: 0,
            y: 0,
            fg_color: fg_color,
            bg_color: bg_color,
            displaying: displaying,
            buffer: [(bg_color as u16) << 12, ..(VGA_WIDTH * VGA_HEIGHT)]
        }
    }
    
    pub fn clear(&mut self) {
        do util::range(0, VGA_WIDTH * VGA_HEIGHT) |i| {
            self.set_entry(i as u16, (self.bg_color as u16) << 12)
        }
        if self.displaying {
            clear_screen(self.bg_color);
        }
    }
    
    pub fn set_colors(&mut self, fg_color: Color, bg_color: Color) {
        self.fg_color = fg_color;
        self.bg_color = bg_color;
    }

    fn scroll(&mut self) {
        do util::range(0, VGA_WIDTH * (VGA_HEIGHT - 1)) |p| {
            let entry = self.get_entry((p + VGA_WIDTH) as u16);
            self.set_entry(p as u16, entry);
        }
        do util::range(VGA_WIDTH * (VGA_HEIGHT - 1), VGA_WIDTH * VGA_HEIGHT) |p| {
            self.set_entry(p as u16, (self.bg_color as u16) << 12);
        }
    }
    
    fn get_entry(&self, i: u16) -> u16 {
        self.buffer[i]
    }
    
    fn set_entry(&mut self, i: u16, entry: u16) {
        self.buffer[i] = entry;
        if self.displaying {
            set_entry(i, entry);
        }
    }
    
    fn set_char(&mut self, x: u8, y: u8, chr: char, fg_color: Color, bg_color: Color) {
        if (x as uint) < VGA_WIDTH && (y as uint) < VGA_HEIGHT {
            let colored_char: u16 = (bg_color as u16) << 12 | (fg_color as u16) << 8 | (chr as u16);
            self.set_entry(((x as uint) + (y as uint) * VGA_WIDTH) as u16, colored_char);
        }
    }
}

impl Terminal for VgaTerm {
    pub fn put_char(&mut self, c: char) {
        if c == '\n' {
            self.x = 0;
            self.y += 1;
            if (self.y as uint) == VGA_HEIGHT {
                self.y -= 1;
                self.scroll();
            }
        } else {
            self.set_char(self.x, self.y, c, self.fg_color, self.bg_color);
            self.x += 1;
            if (self.x as uint) == VGA_WIDTH {
                self.x = 0;
                self.y += 1;
                if (self.y as uint) == VGA_HEIGHT {
                    self.y -= 1;
                    self.scroll();
                }
            }
        }
    }
    
    pub fn put_string(&mut self, s: &str) {
        for str::each(s) |c| {
            self.put_char(c as char);
        }
    }
    
    pub fn set_displaying(&mut self, displaying: bool) {
        let used_to_be_displaying = self.displaying;
        self.displaying = displaying;
        if !used_to_be_displaying && displaying {
            do util::range(0, VGA_WIDTH * VGA_HEIGHT) |i| {
                set_entry(i as u16, self.buffer[i]);
            }
        }
    }
}
