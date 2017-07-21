#![cfg_attr(all(test, feature = "nightly"), feature(test))]

//! Phoenix is a Phoenix client library written in Rust.
//!
//! 

#[cfg(all(feature = "nightly", test))]
extern crate test;

pub mod channel;
pub mod socket;

#[cfg(test)]
mod tests {
    use super::socket;
    #[test]
    fn set_callback_test() {
        let mut sock = socket::Socket::new("Hello");
        sock.set_callback_open(|| println!("Oh hello!"));
        sock.process_events();
        sock.set_callback_open(|| println!("Mello yellow"));
        sock.process_events();
    }
}