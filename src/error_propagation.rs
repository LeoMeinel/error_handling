/*
 * File: error_propagation.rs
 * Author: Leopold Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2022 Leopold Meinel & contributors
 * SPDX ID: GPL-3.0-or-later
 * URL: https://www.gnu.org/licenses/gpl-3.0-standalone.html
 * -----
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
