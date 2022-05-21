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
 * along with this program. If not, see https://github.com/TamrielNetwork/error_handling/blob/main/LICENSE
 */

/*
 * The if expression checks whether our value is out of range, tells the user about the problem, and
 * calls continue to start the next iteration of the loop and ask for another guess. After the if
 * expression, we can proceed with the comparisons between guess and the secret number knowing that
 * guess is between 1 and 100.
 * However, this is not an ideal solution: if it was absolutely critical that the program only operated
 * on values between 1 and 100, and it had many functions with this requirement, having a check like
 * this in every function would be tedious (and might impact performance).
 * Instead, we can make a new type and put the validations in a function to create an instance of the
 * type rather than repeating the validations everywhere. That way, it’s safe for functions to use the
 * new type in their signatures and confidently use the values they receive. Listing 9-13 shows one way
 * to define a Guess type that will only create an instance of Guess if the new function receives a value
 * between 1 and 100.
 */

pub struct Guess {
    pub(crate) value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if !(1..=100).contains(&value) {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
