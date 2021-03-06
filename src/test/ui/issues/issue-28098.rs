// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

fn main() {
    let _ = Iterator::next(&mut ());
    //~^ ERROR `()` is not an iterator

    for _ in false {}
    //~^ ERROR `bool` is not an iterator

    let _ = Iterator::next(&mut ());
    //~^ ERROR `()` is not an iterator

    other()
}

pub fn other() {
    // check errors are still reported globally

    let _ = Iterator::next(&mut ());
    //~^ ERROR `()` is not an iterator

    let _ = Iterator::next(&mut ());
    //~^ ERROR `()` is not an iterator

    for _ in false {}
    //~^ ERROR `bool` is not an iterator
}
