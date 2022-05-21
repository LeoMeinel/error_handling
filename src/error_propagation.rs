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

use std::error::Error;
use std::fs::{self, File};
use std::io;
use std::io::Read;

pub(crate) fn error_propagation(crash: bool) {
    if crash {
        read_username_from_file_0().expect("Can't read username!");
        read_username_from_file_1().expect("Can't read username!");
        read_username_from_file_2().expect("Can't read username!");
        read_username_from_file_3().expect("Can't read username!");
        like_main().expect("Can't open file!");
    } else {
        let _a = read_username_from_file_0();
        let _b = read_username_from_file_1();
        let _c = read_username_from_file_2();
        let _d = read_username_from_file_3();
        let _e = like_main();
    }
}
// Box is trait object -> any kind of error

fn like_main() -> Result<(), Box<dyn Error>> {
    let _f = File::open("file_like_main.txt")?;
    Ok(())
}

fn read_username_from_file_3() -> Result<String, io::Error> {
    fs::read_to_string("file_3.txt")
}

fn read_username_from_file_2() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("file_2.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_1() -> Result<String, io::Error> {
    let mut f = File::open("file_1.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_0() -> Result<String, io::Error> {
    let f = File::open("file_0.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
