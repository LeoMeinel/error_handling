/*
 * File: result.rs
 * Author: Leopold Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2022 Leopold Meinel & contributors
 * SPDX ID: GPL-3.0-or-later
 * URL: https://www.gnu.org/licenses/gpl-3.0-standalone.html
 * -----
 */

use std::fs::File;
use std::io::ErrorKind;

// error propagation:
// bool because program stops after panic!
pub(crate) fn result() {
    display_result_panic(false);
    display_result_panic_error_kind(false);
    display_result_panic_error_kind_closures(false);
    display_result_panic_unwrap(false);
    display_result_panic_expect(false);
}

fn display_result_panic_expect(crash: bool) {
    if crash {
        let _f = File::open("non_existent.txt").expect("Problem opening file: non_existent.txt");
    }
}

fn display_result_panic_unwrap(crash: bool) {
    if crash {
        let _f = File::open("non_existent.txt").unwrap();
    }
}

fn display_result_panic_error_kind_closures(crash: bool) {
    if crash {
        let _f = File::open("existent_1.txt").unwrap_or_else(|error| {
            if error.kind() == ErrorKind::NotFound {
                File::create("existent_1.txt").unwrap_or_else(|error| {
                    panic!("Problem creating file: {:#?}", error);
                })
            } else {
                panic!("Problem opening file: {:#?}", error);
            }
        });
    }
}

fn display_result_panic_error_kind(crash: bool) {
    if crash {
        let f = File::open("existent.txt");
        let _f = match f {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create("existent.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating file: {:#?}", e),
                },
                other_error => panic!("Problem opening file: {:#?}", other_error),
            },
        };
    }
}

fn display_result_panic(crash: bool) {
    if crash {
        let f = File::open("non_existent.txt");
        let _f = match f {
            Ok(file) => file,
            Err(error) => panic!("Problem opening file: {:#?}", error),
        };
    }
}
