// Copyright (c) 2013 John Grosen
// Licensed under the terms described in the LICENSE file in the root of this repository

pub mod rusti {
    #[abi = "rust-intrinsic"]
    #[link_name = "rusti"]
    pub extern "rust-intrinsic" {
        fn forget<T>(x: T);

        fn transmute<T,U>(e: T) -> U;
    }
}

#[inline(always)]
pub fn as_buf<T>(s: &str, f: &fn(*u8, uint) -> T) -> T {
    unsafe {
        let v : *(*u8,uint) = rusti::transmute(&s);
        let (buf,len) = *v;
        f(buf, len)
    }
}

#[inline(always)]
pub fn len(s: &str) -> uint {
    do as_buf(s) |_p, n| { n - 1u }
}

#[inline(always)]
pub fn eachi(s: &str, it: &fn(uint, u8) -> bool) -> bool {
    let mut pos = 0;
    let length = len(s);

    while pos < length {
        if !it(pos, s[pos]) { return false; }
        pos += 1;
    }
    true
}

#[inline(always)]
pub fn each(s: &str, it: &fn(u8) -> bool) -> bool {
    eachi(s, |_i, b| it(b))
}
