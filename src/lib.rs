#![warn(clippy::all)]
#![cfg_attr(nightly, feature(const_fn))]

//! A library to deal with 3scale Porta.
//!

// skip reformatting this line below, since it pulls in macros and macros
// need to be pulled in before usage.
#[rustfmt::skip]
#[macro_use]
pub mod resources;

pub mod api;

pub mod client;
pub mod proxy;
pub mod service;

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
