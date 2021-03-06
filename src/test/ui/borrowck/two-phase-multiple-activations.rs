// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// compile-flags: -Z borrowck=mir -Z two-phase-borrows

// run-pass

use std::io::Result;

struct Foo {}

pub trait FakeRead {
    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize>;
}

impl FakeRead for Foo {
    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize> {
        Ok(4)
    }
}

fn main() {
    let mut a = Foo {};
    let mut v = Vec::new();
    a.read_to_end(&mut v);
}
