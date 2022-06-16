#![deny(missing_debug_implementations)]
#![deny(missing_docs)]

//! # arson
//!
//! **arson** is a simple rust json library for parsing string. it has nice formatted colored output
//!
//! ## Example
//!
//! ```rust
//! use arson::{JSONError, Parser, JSON, JSON::*};
//!
//! fn main() -> Result<(), JSONError> {
//!     // alternative A
//!     let json_str = std::fs::read_to_string("ex.json").unwrap();
//!     // alternative B
//!     let json_str = r#"{
//!         "name": "John Doe",
//!         "age": 43,
//!         "address": {
//!             "street": "10 Downing Street",
//!             "city": "London"
//!         },
//!         "phones": [
//!             "+44 1234567",
//!             "+44 2345678"
//!         ]
//!     } "#;
//!
//!     // alternative 1
//!     let json = json_str.parse::<JSON>().expect("Failed to parse json");
//!     // alternative 2
//!     let json = Parser::parse(json_str.chars())?;
//!
//!     println!("{:?}", json);
//!
//!     match json {
//!         Array(val) => {}  // Vec<JSON>
//!         Object(val) => {} // HashMap<String, JSON>
//!         String(val) => {} // String
//!         Number(val) => {} // f64
//!         Bool(val) => {}   // bool
//!         Null => {}
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! Output
//!
//! ```
//! {
//!     "address": {
//!         "city": "London",
//!         "street": "10 Downing Street",
//!     },
//!     "name": "John Doe",
//!     "age": 43,
//!     "phones": [
//!         +44 1234567,
//!         +44 2345678,
//!     ],
//! }
//! ```

mod json;
mod json_error;
mod parser;

pub use json::JSON;
pub use json_error::JSONError;
pub use parser::Parser;
