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
 * Let’s take the idea of using Rust’s type system to ensure we have a valid value one step further
 * and look at creating a custom type for validation. Recall the guessing game in Chapter 2 in which
 * our code asked the user to guess a number between 1 and 100. We never validated that the user’s
 * guess was between those numbers before checking it against our secret number; we only validated
 * that the guess was positive. In this case, the consequences were not very dire: our output of
 * “Too high” or “Too low” would still be correct. But it would be a useful enhancement to guide
 * the user toward valid guesses and have different behavior when a user guesses a number that’s
 * out of range versus when a user types, for example, letters instead.
 * One way to do this would be to parse the guess as an i32 instead of only a u32 to allow
 * potentially negative numbers, and then add a check for the number being in range, like so:
 */
use std::cmp::Ordering;
use std::io;

use rand::Rng;

use self::struct_guess::Guess;

mod struct_guess;

pub(crate) fn custom_validation() {
    custom_validation_guessing_game_old();
    custom_validation_guessing_game();
}

fn custom_validation_guessing_game() {
    println!("Guess the number!");
    let secret_number: i32 = rand::thread_rng().gen_range(0..101);
    println!("The secret number is: {}", secret_number);
    loop {
        println!("Please input your guess!");
        let mut guess = String::new();
        match io::stdin().read_line(&mut guess) {
            Ok(string) => string,
            Err(_) => {
                println!("WARNING: Failed to read line!");
                continue;
            }
        };
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        let guess = Guess::new(guess);
        println!("You guessed: {}", guess.value);
    }
}

fn custom_validation_guessing_game_old() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(0..101);
    println!("The secret number is: {}", secret_number);
    loop {
        println!("Please input your guess!");
        let mut guess = String::new();
        match io::stdin().read_line(&mut guess) {
            Ok(string) => string,
            Err(_) => {
                println!("WARNING: Failed to read line!");
                continue;
            }
        };
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if !(1..=100).contains(&guess) {
            println!("The secret number will be between 1 and 100.");
            continue;
        }
        println!("You guessed: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("Correct!");
                break;
            }
            Ordering::Greater => println!("Too big!"),
        }
    }
}
