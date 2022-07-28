/*
 * error_handling is a commandline application.
 * Copyright © 2022 Leopold Meinel & contributors
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see https://github.com/LeoMeinel/error_handling/blob/main/LICENSE
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
