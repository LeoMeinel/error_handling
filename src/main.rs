/*
 * File: main.rs
 * Author: Leopold Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2022 Leopold Meinel & contributors
 * SPDX ID: GPL-3.0-or-later
 * URL: https://www.gnu.org/licenses/gpl-3.0-standalone.html
 * -----
 */

/*
 * panic!() -> debug/exceptional where recovery is not possible
 * expect(), unwrap() -> example code/tests/prototype code/100% know success
 * Result<T, E> -> default
 * error_propagation ? -> default: simpler, decide how to handle best
 */

extern crate core;

use crate::custom_validation::custom_validation;
use crate::error_propagation::error_propagation;
use crate::know_success::know_success;
use crate::panic_macro::panic_macro;
use crate::result::result;

mod custom_validation;
mod error_propagation;
mod know_success;
mod panic_macro;
mod result;

fn main() {
    panic_macro();
    result();
    error_propagation(false);
    know_success();
    custom_validation();
}
