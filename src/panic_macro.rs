/*
 * File: panic_macro.rs
 * Author: Leopold Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2022 Leopold Meinel & contributors
 * SPDX ID: GPL-3.0-or-later
 * URL: https://www.gnu.org/licenses/gpl-3.0-standalone.html
 * -----
 */

// bool because program stops after panic!
pub(crate) fn panic_macro() {
    panic_simple(false);
    call_panic_backtrace(false);
}
// RUST_BACKTRACE=full/1 cargo run to see full backtrace!

fn call_panic_backtrace(crash: bool) {
    panic_backtrace(crash, 11);
}

fn panic_backtrace(crash: bool, num: i32) {
    if num == 11 && crash {
        panic!("num is 11, which is not accepted!");
    }
}

fn panic_simple(crash: bool) {
    if crash {
        panic!("crash and burn!");
    }
}
