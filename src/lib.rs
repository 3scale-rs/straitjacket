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

#[cfg(client)]
pub mod client;

/// Public dependencies for downstream crate compatibility
pub mod deps {
    #[cfg(client)]
    pub use reqwest;
    pub use url;
}

#[cfg(client)]
pub use deps::reqwest;
pub use deps::url;

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
