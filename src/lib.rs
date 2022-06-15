//! # arson
//!
//! **arson** is a simple rust json library for parsing string. it has nice formatted colored output

mod json;
mod json_error;
mod parser;

pub use json::JSON;
pub use json_error::JSONError;
pub use parser::Parser;
