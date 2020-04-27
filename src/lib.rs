#![warn(rust_2018_idioms)]
#![warn(clippy::all)]
#![feature(exclusive_range_pattern)]
#![feature(half_open_range_patterns)]

mod dto;
pub mod middleware;
pub mod provider;
mod query;
pub mod route;
mod serverstate;
pub use serverstate::ServerState;
