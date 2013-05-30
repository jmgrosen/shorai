// Copyright (c) 2013 John Grosen
// Licensed under the terms described in the LICENSE file in the root of this repository


pub trait Terminal {
    pub fn put_char(&mut self, c: char);
    pub fn put_string(&mut self, c: &str);
    pub fn set_displaying(&mut self, displaying: bool);
}
