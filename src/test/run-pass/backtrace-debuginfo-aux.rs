// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// ignore-test: not a test, used by backtrace-debuginfo.rs to test file!()

#[inline(never)]
#[rustc_no_mir] // FIXME #31005 MIR missing debuginfo currently.
pub fn callback<F>(f: F) where F: FnOnce((&'static str, u32)) {
    f((file!(), line!()))
}

// LLVM does not yet output the required debug info to support showing inlined
// function calls in backtraces when targetting MSVC, so disable inlining in
// this case.
#[cfg_attr(not(target_env = "msvc"), inline(always))]
#[cfg_attr(target_env = "msvc", inline(never))]
#[rustc_no_mir] // FIXME #31005 MIR missing debuginfo currently.
pub fn callback_inlined<F>(f: F) where F: FnOnce((&'static str, u32)) {
    f((file!(), line!()))
}
