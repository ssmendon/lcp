// Sample program showing the use of the library.
// Copyright (C) 2024  Sohum Mendon
// SPDX-License-Identifier: MIT

#![deny(clippy::all, clippy::pedantic)]

use lcp::longest_common_prefix;

fn main() {
    let lcp = std::env::args()
        .skip(1)
        .reduce(|acc, s| longest_common_prefix(&acc, &s).to_owned());

    if let Some(lcp) = lcp {
        if lcp.is_empty() {
            println!("<empty>");
        } else {
            println!("{lcp}");
        }
    } else {
        eprintln!("Usage: lcp [word...]");
    }
}
