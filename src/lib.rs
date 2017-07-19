#![cfg_attr(all(test, feature = "nightly"), feature(test))]

//! Phoenix is a Phoenix client library written in Rust.
//!
//! 

#[cfg(all(feature = "nightly", test))]
extern crate test;

mod channel;
mod socket;
mod long_poller;