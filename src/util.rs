// Copyright (c) 2013 John Grosen
// Licensed under the terms described in the LICENSE file in the root of this repository

#[no_std];
#[no_core];

pub fn range(lo: uint, hi: uint, it: &fn(uint)) {
    let mut iter = lo;
    while iter < hi {
        it(iter);
        iter += 1;
    }
}
