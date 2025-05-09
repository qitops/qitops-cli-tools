pub mod api;
pub mod perf;
pub mod sec;
pub mod web;
pub mod common;
pub mod error;

pub use error::Error;
pub type Result<T> = std::result::Result<T, Error>; 