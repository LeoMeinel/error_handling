/*
 * File: know_success.rs
 * Author: Leopold Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2022 Leopold Meinel & contributors
 * SPDX ID: GPL-3.0-or-later
 * URL: https://www.gnu.org/licenses/gpl-3.0-standalone.html
 * -----
 */

use std::net::IpAddr;

pub(crate) fn know_success() {
    // Not if string is dynamic in any way
    let localhost: IpAddr = "127.0.0.1".parse().unwrap();
    println!("localhost is: {}", localhost);
}
