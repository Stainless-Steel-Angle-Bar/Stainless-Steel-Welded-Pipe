// Copyright 2014 Jonathan Reem
// Copyright 2015 Jonathan Reem, Utkarsh Kukreti, Mark Schifflin,
//                Aleksey Kuznetsov
// Copyright 2016 Urban Hafner
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and
// associated documentation files (the "Software"), to deal in the Software without restriction,
// including without limitation the rights to use, copy, modify, merge, publish, distribute,
// sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or
// substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT
// NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
// DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

#![feature(plugin_registrar, quote, rustc_private)]
#![deny(missing_docs, warnings)]

//! > Stainless is a lightweight, flexible, unopinionated testing framework.
//!
//! **Note that stainless currently requires the nightly version of the Rust compiler!**
//!
//! ## Installation
//!
//! Add stainless as a dependency in your `Cargo.toml` file
//! ``` toml
//! [dev-dependencies]
//! stainless = "*"
//! ```
//!
//! Add the following lines to the top of your
//! [root module](https://doc.rust-lang.org/book/crates-and-modules.html).
//! That file is normally called `src/main.rs` for executables and
//! `src/lib.rs` for libraries:
//!
//! ``` rust
//! #![feature(plugin)]
//! #![cfg_attr(test, plugin(stainless))]
//! ```
//!
//! This will make stainless available when you run the tests using `cargo
//! test`.
//!
//! ## Overview
//!
//! Stainless exports the `describe!` syntax extension, which allows you
//! to quickly generate complex testing hierarchies and reduce boilerplate
//! through `before_each` and `after_each`.
//!
//! Stainless currently supports the following types of subblocks:
//!
//! * `before_each` and `after_each`
//! * `it`, `failing`, and `ignore`
//! * `bench`
//! * nested `describe!`
//!
//! `before_each` and `after_each` allow you to group common
//! initialization and teardown for a group of tests into a single block,
//! shortening your tests.
//!
//! `it` generates tests which use `before_each` and `after_each`.
//! `failing` does the same, except the generated tests are marked with
//! `#[should_panic]`. It optionally takes an argument which is matched against the
//! failure message. `ignore` is equivalent to marking a test with `#[ignore]` which
//! disables the test by default.
//!
//! `bench` allows you to generate benchmarks in the same fashion, though
//! *`before_each` and `after_each` blocks do not currently affect `bench`
//! blocks*.
//!
//! Nested `describe!` blocks allow you to better organize your tests into
//! small units and gives you granular control over where `before_each`
//! and `after_each` apply. Of course the `before_each` and `after_each`
//! blocks of the wrapping `describe!` blocks are executed as well.
//!
//! Together, these 4 types of subblocks give you more flexibility and
//! control than the built in testing infrastructure.
//!
//! ## Example
//!
//! ```rust
//! describe! stainless {
//!     before_each {
//!         // Start up a test.
//!         let mut stainless = true;
//!     }
//!
//!     it "makes organizing tests easy" {
//!         // Do the test.
//!         assert!(stainless);
//!     }
//!
//!     after_each {
//!         // End the test.
//!         stainless = false;
//!     }
//!
//!     bench "something simple" (bencher) {
//!         bencher.iter(|| 2 * 2)
//!     }
//!
//!     describe! nesting {
//!
//!         before_each {
//!           let mut inner_stainless = true;
//!         }
//!
//!         after_each {
//!           inner_stainless = false;
//!         }
//!
//!         it "makes it simple to categorize tests" {
//!             // It even generates submodules!
//!             assert_eq!(2, 2);
//!         }
//!     }
//! }
//! ```
//!
//! Expands to (roughly):
//!
//! ```rust
//! mod stainless {
//!     #[test]
//!     fn makes_organizing_tests_easy() {
//!         let mut stainless = true;
//!         assert!(stainless);
//!         stainless = false;
//!     }
//!
//!     #[bench]
//!     fn something_simple(bencher: &mut test::Bencher) {
//!         bencher.iter(|| 2 * 2)
//!     }
//!
//!     mod nesting {
//!         #[test]
//!         fn makes_it_simple_to_categorize_tests() {
//!             let mut stainless = true;
//!             let mut inner_stainless = true;
//!             assert_eq!(2, 2);
//!             inner_stainless = false;
//!             stainless = false;
//!         }
//!     }
//! }
//! ```
//!
//! ## Importing modules
//!
//! At this point it is not possible to put `use` statements inside the
//! `describe!` blocks. To allow usage of data structures from other
//! modules and crates each `describe!` block comes with a silent `pub use
//! super::*;` in it. That way everything you `pub use` in the containing
//! module is available in your tests.
//!
//! ```rust
//! #[cfg(test)]
//! mod tests {
//!     pub use std::collections::HashMap;
//!
//!     describe! stainless {
//!         it "can use HashMap" {
//!             let map = HashMap::new();
//!         }
//!     }
//! }
//! ```
//!
//! ## License
//!
//! MIT

extern crate syntax;
extern crate rustc_plugin;

use self::describe::describe;
use rustc_plugin as plugin;
use syntax::parse::token;

mod describe;
mod parse;
mod test;
mod bench;
mod generate;

#[plugin_registrar]
#[doc(hidden)]
pub fn plugin_registrar(reg: &mut plugin::Registry) {
    reg.register_syntax_extension(
        token::intern("describe"),
        syntax::ext::base::IdentTT(Box::new(describe), None, false)
    );
}
