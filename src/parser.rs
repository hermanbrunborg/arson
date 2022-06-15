use super::{JSONError, JSON};
use std::collections::HashMap;
use std::iter::Peekable;

/// JSON parser
pub struct Parser<I>
where
    I: Iterator<Item = char>,
{
    chars: Peekable<I>,
    line: usize,
    col: usize,
}

/// The json data to be parsed must be iterable, and it must contain characters
impl<I: Iterator<Item = char>> Parser<I> {
    /// Parse json data
    pub fn parse(chars: I) -> Result<JSON, JSONError> {
        Parser {
            chars: chars.peekable(),
            line: 1,
            col: 0,
        }
        .parse_object()
    }

    fn parse_object(&mut self) -> Result<JSON, JSONError> {
        if self.consume()? != '{' {
            return self.err("Object did not start with `{`".into());
        }

        if self.peek()? == '}' {
            self.consume().unwrap();
            return Ok(JSON::Object(HashMap::new()));
        }

        let mut map = HashMap::new();

        loop {
            let key = match self.parse_any()? {
                JSON::String(x) => x,
                x => return self.err(format!("Key of object must be `Str`, but found {:?}", x)),
            };

            let c = self.consume()?;
            if c != ':' {
                return self.err(format!("':' is expected after key, but found `{:?}`", c));
            }

            let value = self.parse_any()?;

            map.insert(key, value);

            match self.consume()? {
                '}' => return Ok(JSON::Object(map)),
                ',' => continue,
                c => return self.err(format!("Unexpected token {:?}", c)),
            };
        }
    }

    fn update_position(&mut self, c: char) {
        match c {
            ' ' => self.col += 1,
            '\n' => {
                self.line += 1;
                self.col = 0;
            }
            '\t' => self.col += 4,
            _ => self.col += 1,
        }
    }

    fn consume(&mut self) -> Result<char, JSONError> {
        if let Some(c) = self.next_char_token() {
            Ok(c)
        } else {
            self.eof_err()
        }
    }

    fn peek(&mut self) -> Result<char, JSONError> {
        while let Some(c) = self.chars.peek().copied() {
            if !self.is_whitespace(c) {
                return Ok(c);
            }
            self.update_position(c);
            self.chars.next();
        }
        self.eof_err()
    }

    fn next_char_token(&mut self) -> Option<char> {
        while let Some(c) = self.chars.next() {
            self.update_position(c);
            if !self.is_whitespace(c) {
                return Some(c);
            }
        }
        None
    }

    fn next_char(&mut self) -> Result<char, JSONError> {
        if let Some(c) = self.chars.next() {
            self.update_position(c);
            Ok(c)
        } else {
            self.eof_err()
        }
    }

    fn parse_string(&mut self) -> Result<JSON, JSONError> {
        let c = self.consume()?;
        if c != '"' {
            return self.err(format!("String must start with `\"`, found `{}`", c));
        }
        let mut string = String::new();

        loop {
            match self.next_char()? {
                '\\' => {
                    let c = match self.next_char()? {
                        '"' => '"',
                        '\\' => '\\',
                        '/' => '/',
                        'b' => '\u{0008}',
                        'f' => '\u{000C}',
                        'n' => '\n',
                        'r' => '\r',
                        't' => '\t',
                        'u' => {
                            let u: u32 = (0..4)
                                .map(|i| match self.consume()?.to_digit(16) {
                                    Some(x) => Ok((1 << (4 * (3 - i))) * x), // shift the number to the correct place
                                    None => self.err(format!("Unable to parse `{}` as hex", c)),
                                })
                                .sum::<Result<u32, JSONError>>()?;

                            match std::char::from_u32(u) {
                                Some(c) => c,
                                None => {
                                    return self.err(format!("\\u`{}` is not a valid character", u))
                                }
                            }
                        }
                        c => return self.err(format!("No escape token exists for `{}`", c)),
                    };

                    string.push(c);
                }
                '"' => return Ok(JSON::String(string)),
                c => string.push(c),
            }
        }
    }

    fn extract_digits(&mut self) -> Result<String, JSONError> {
        let mut num_str = String::new();
        loop {
            match self.peek()? {
                '0'..='9' => num_str.push(self.consume()?),
                _ => return Ok(num_str),
            }
        }
    }

    fn parse_number(&mut self) -> Result<JSON, JSONError> {
        let mut num_str = String::new();

        // parse optional - sign
        if self.peek()? == '-' {
            num_str.push(self.consume()?);
        }

        // parse integer part
        match self.peek()? {
            '0' => {
                num_str.push(self.consume()?);
                match self.peek()? {
                    '0'..='9' => {
                        return self.err(
                            "Number starting with `0` must be followed by `.` or nothing".into(),
                        )
                    }
                    _ => {}
                }
            }
            '1'..='9' => num_str.push_str(&self.extract_digits()?),
            c => {
                return self.err(format!(
                    "`{}` is not valid as part of integer part of a number",
                    c
                ))
            }
        }

        // parse fraction part
        if self.peek()? == '.' {
            num_str.push(self.consume()?);
            num_str.push_str(&self.extract_digits()?)
        }

        // parse exponent
        if self.peek()? == 'e' || self.peek()? == 'E' {
            num_str.push(self.consume()?);
            if self.peek()? == '-' || self.peek()? == '+' {
                num_str.push(self.consume()?);
            }
            num_str.push_str(&self.extract_digits()?)
        }

        return match num_str.parse() {
            Ok(num) => Ok(JSON::Number(num)),
            _ => self.err(format!("`{}` could not be parsed as a number", num_str)),
        };
    }

    fn parse_array(&mut self) -> Result<JSON, JSONError> {
        let c = self.consume()?;
        if c != '[' {
            return self.err(format!("Array must start with `[`, found `{}`", c));
        }

        let mut items = vec![];

        loop {
            match self.peek()? {
                ',' => {
                    self.consume()?;
                }
                ']' => {
                    self.consume()?;
                    return Ok(JSON::Array(items));
                }
                _ => items.push(self.parse_any()?),
            }
        }
    }

    fn parse_static_word(&mut self, c: char) -> Result<JSON, JSONError> {
        let word = match c {
            't' => "true",
            'f' => "false",
            'n' => "null",
            _ => return self.err(format!("No tokens start with the character `{}`", c)),
        };
        for c in word.chars() {
            match self.next_char() {
                Ok(x) if c != x => {
                    return self.err(format!(
                        "Could not parse word. Did you mean `{}`? (diff: `{}`/`{}`)",
                        word, c, x
                    ))
                }
                Ok(_) => {}
                _ => return self.err("Not able to collect next character".into()),
            };
        }

        match c {
            't' => Ok(JSON::Bool(true)),
            'f' => Ok(JSON::Bool(false)),
            'n' => Ok(JSON::Null),
            _ => return self.err(format!("No tokens start with the character `{}`", c)),
        }
    }

    fn parse_any(&mut self) -> Result<JSON, JSONError> {
        match self.peek()? {
            '0'..='9' | '-' => self.parse_number(),
            '"' => self.parse_string(),
            '{' => self.parse_object(),
            '[' => self.parse_array(),
            x @ 't' | x @ 'f' | x @ 'n' => self.parse_static_word(x),
            c => self.err(format!("Not able to parse `{}`", c)),
        }
    }

    fn is_whitespace(&self, c: char) -> bool {
        // from json.org
        match c {
            '\u{0020}' | '\u{000A}' | '\u{000D}' | '\u{0009}' => true,
            _ => false,
        }
    }

    fn raw_err(&self, msg: String) -> JSONError {
        JSONError::new(msg, self.line, self.col)
    }

    fn err<T>(&self, msg: String) -> Result<T, JSONError> {
        Err(self.raw_err(msg))
    }

    fn eof_err<T>(&self) -> Result<T, JSONError> {
        self.err("Unexpected end of file during parsing".into())
    }
}
