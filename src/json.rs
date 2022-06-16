use super::{parser::Parser, JSONError};
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;
use termion::color;

/// Contains the JSON types and can be used to parse strings to json
#[derive(PartialEq)]
pub enum JSON {
    /// JSON Object
    Object(HashMap<String, JSON>),
    /// JSON Array
    Array(Vec<JSON>),
    /// JSON String
    String(String),
    /// JSON Number
    Number(f64),
    /// JSON Boolean
    Bool(bool),
    /// JSON Null
    Null,
}

impl FromStr for JSON {
    type Err = JSONError;

    fn from_str(json: &str) -> Result<JSON, JSONError> {
        Ok(Parser::parse(json.chars())?)
    }
}

impl fmt::Debug for JSON {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JSON::Array(x) => write!(f, "{:#?}", x),
            JSON::Number(x) => write!(
                f,
                "{}{}{}",
                color::Fg(color::Blue),
                x,
                color::Fg(color::Reset)
            ),
            JSON::String(x) => write!(
                f,
                r#"{}"{}"{}"#,
                color::Fg(color::LightGreen),
                x,
                color::Fg(color::Reset),
            ),
            JSON::Bool(x) => write!(
                f,
                "{}{}{}",
                color::Fg(color::Magenta),
                x,
                color::Fg(color::Reset),
            ),
            JSON::Null => write!(
                f,
                "{}null{}",
                color::Fg(color::Green),
                color::Fg(color::Reset),
            ),
            JSON::Object(x) => {
                write!(f, "{:#?}", x)
            }
        }
    }
}
