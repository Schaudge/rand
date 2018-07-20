// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// https://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! The xorshift random number generator.

#![doc(html_logo_url = "https://www.rust-lang.org/logos/rust-logo-128x128-blk.png",
       html_favicon_url = "https://www.rust-lang.org/favicon.ico",
       html_root_url = "https://docs.rs/rand_xorshift/0.1.0")]

#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![doc(test(attr(allow(unused_variables), deny(warnings))))]

#![cfg_attr(not(feature="std"), no_std)]

#[cfg(feature="std")] extern crate std as core;
extern crate rand_core;

#[cfg(feature="serde1")] extern crate serde;
#[cfg(feature="serde1")] #[macro_use] extern crate serde_derive;
#[cfg(all(feature="serde1", test))] extern crate bincode;

mod xorshift;

pub use self::xorshift::XorShiftRng;
