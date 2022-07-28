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
