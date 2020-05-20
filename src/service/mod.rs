/// Public dependencies for downstream crate compatibility
pub mod deps {
    pub use reqwest;
    pub use url;
}

pub use deps::reqwest;
pub use deps::url;

mod method;
mod service;
mod metric;
