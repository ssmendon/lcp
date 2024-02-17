// Small library to find a common prefix among strings.
// Copyright (C) 2024  Sohum Mendon
// SPDX-License-Identifier: MIT

//! Find the longest common prefix in a string.
//!
//! There are only two publicly exported methods:
//! [`longest_common_prefix`] and [`longest_common_prefix_in`].
//!
//! Example
//! ```rust
//! use lcp::longest_common_prefix;
//! use lcp::longest_common_prefix_in;
//!
//! let prefix = longest_common_prefix("HELLO WORLD", "HELLO world");
//! assert_eq!("HELLO ", prefix);
//!
//! let prefix = longest_common_prefix("nothing in", "common");
//! assert_eq!("", prefix);
//!
//! let intoiter = ["what's the", "whatever", "whatabout"];
//! assert_eq!(Some("what"), longest_common_prefix_in(intoiter));
//!
//! let intoiter = ["there's no", "common prefix", "here"];
//! assert_eq!(Some(""), longest_common_prefix_in(intoiter));
//! ```

#![no_std]
#![deny(clippy::all, clippy::pedantic)]
#![allow(clippy::must_use_candidate)]

use core::ptr;

/// Find the longest common prefix between two strings.
///
/// This returns a [`str`], which can be the empty string `""` if
/// there is no common prefix.
pub fn longest_common_prefix<'a>(a: &'a str, b: &'a str) -> &'a str {
    if ptr::eq(a, b) {
        return a;
    }

    for (i, (ac, bc)) in a.chars().zip(b.chars()).enumerate() {
        if ac != bc {
            return &a[..i];
        }
    }

    if a.len() < b.len() {
        a
    } else {
        b
    }
}

/// Find the longest prefix in an iterable.
///
/// This returns [`None`] if the passed in iterable is empty. Otherwise,
/// it returns a [`str`] (including the empty string `""` if there is
/// no common prefix).
pub fn longest_common_prefix_in<'a>(iter: impl IntoIterator<Item = &'a str>) -> Option<&'a str> {
    let mut iter = iter.into_iter();

    let lcp = iter.next();

    let mut lcp = lcp?;

    for cur in iter {
        lcp = longest_common_prefix(lcp, cur);

        if lcp.is_empty() {
            return Some(lcp);
        }
    }

    Some(lcp)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EMPTY: &str = "";
    const HELLO: &str = "hello";

    #[test]
    fn one_is_empty() {
        assert_eq!(EMPTY, longest_common_prefix(HELLO, EMPTY));
        assert_eq!(EMPTY, longest_common_prefix(EMPTY, HELLO));
    }

    #[test]
    fn both_are_same_ptr() {
        assert_eq!(EMPTY, longest_common_prefix(EMPTY, EMPTY));
        assert_eq!(HELLO, longest_common_prefix(HELLO, HELLO));
    }

    #[test]
    fn both_have_same_content() {
        let local_empty = "";
        let local = "hello";

        assert_eq!(EMPTY, longest_common_prefix(EMPTY, local_empty));
        assert_eq!(HELLO, longest_common_prefix(HELLO, local));
    }

    #[test]
    fn no_common_prefix() {
        let uncommon = "nothing";
        assert_eq!(EMPTY, longest_common_prefix(HELLO, uncommon));
        assert_eq!(EMPTY, longest_common_prefix(uncommon, HELLO));
    }

    #[test]
    fn common_prefix() {
        let common = "hel";
        let help = "help";

        assert_eq!(common, longest_common_prefix(HELLO, help));
        assert_eq!(common, longest_common_prefix(help, HELLO));
    }

    #[test]
    fn empty_iterable() {
        let iter = [];

        assert_eq!(None, longest_common_prefix_in(iter));
    }

    #[test]
    fn one_element_in_iterable() {
        let iter = [EMPTY];

        assert_eq!(Some(""), longest_common_prefix_in(iter));
    }

    #[test]
    fn no_common_prefix_in_iterable() {
        let iter = [EMPTY, HELLO, "info"];

        assert_eq!(Some(""), longest_common_prefix_in(iter));
    }

    #[test]
    fn common_prefix_in_iterable() {
        let iter = [HELLO, "helvetica", "help", "hell"];

        assert_eq!(Some("hel"), longest_common_prefix_in(iter));
    }
}
