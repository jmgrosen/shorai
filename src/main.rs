// Copyright (c) 2013 John Grosen
// Licensed under the terms described in the LICENSE file in the root of this repository

#[allow(ctypes)];
#[no_std];
#[no_core];

use vga;
use term;
use util;

#[no_mangle]
pub fn memset() {}

fn wait() {
    do util::range(0, 1<<28) |_| {
        unsafe {
            asm!("nop");
        }
    }
}

#[no_mangle]
pub fn kernel_main() {
    let mut term = &vga::VgaTerm::new(vga::White, vga::LightRed, true) as &term::Terminal;
    term.put_string("1\n2\n3\n4\n5\n");
    term.put_string("6\n7\n8\n9\n10\n");
    term.put_string("11\n12\n13\n14\n15\n");
    term.put_string("16\n17\n18\n19\n20\n");
    term.put_string("21\n22\n23\n24\n25\n");
    term.put_string("26\n27\n28\n29\n30\n");
    wait();
    let mut foo = &vga::VgaTerm::new(vga::Black, vga::White, false) as &term::Terminal;
    foo.put_string("hi there\n");
    term.put_string("gasp");
    wait();
    term.set_displaying(false);
    term.put_string("\n:O :O");
    foo.set_displaying(true);
    wait();
    foo.set_displaying(false);
    term.put_string("\n;D");
    term.set_displaying(true);
    loop {}
}
