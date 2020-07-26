#![forbid(future_incompatible, rust_2018_compatibility, warnings, clippy::all)]
#![deny(unsafe_code, nonstandard_style, unused, rust_2018_idioms)]

mod dto;
pub mod middleware;
pub mod provider;
mod query;
pub mod route;
mod serverstate;
pub use serverstate::ServerState;
