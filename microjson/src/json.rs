// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

use collections::string::String;
use core::convert::From;
use core::str;

#[derive(Debug, PartialEq)]
pub enum JsonToken {
    StartObject,
    EndObject,
    StartArray,
    EndArray,
    PropertyName(String),
    Literal(String),
    Null,
    True,
    False,
    Done,
}

#[derive(Debug, PartialEq)]
pub enum JsonError {
    UnexpectedCharacter,
    UnexpecteEof,
    InvalidString,
}

impl From<JsonError> for () {
    fn from(_: JsonError) -> () {
        ()
    }
}


#[derive(Debug, PartialEq)]
enum TokenizerState {
    Start,
    ExpectProperty,
    ExpectValue,
    AfterArray,
    InArray,
}

pub struct JsonTokenizer<'a> {
    buffer: &'a [u8],
    len: usize,
    pos: usize,
    state: TokenizerState,
    depth: usize,
}

macro_rules! error_if_eof {
    ($s:ident) => (
        if $s.eof() {
            return Err(JsonError::UnexpecteEof);
        }
    )
}

impl<'a> JsonTokenizer<'a> {
    pub fn new(text: &'a str) -> Self {
        let buffer = text.as_bytes();
        JsonTokenizer {
            buffer: buffer,
            len: buffer.len(),
            pos: 0,
            state: TokenizerState::Start,
            depth: 0,
        }
    }

    fn eof(&self) -> bool {
        self.pos >= self.len
    }

    fn next(&mut self) -> Result<u8, JsonError> {
        let c = self.peek()?;
        self.pos += 1;
        Ok(c)
    }

    fn peek(&mut self) -> Result<u8, JsonError> {
        if self.eof() {
            return Err(JsonError::UnexpecteEof);
        }
        let c = self.buffer[self.pos];
        Ok(c)
    }

    fn eat_ws(&mut self) {
        while let Ok(c) = self.next() {
            if c != b' ' && c != b'\t' && c != b'\r' && c != b'\n' {
                self.pos -= 1;
                return;
            }
        }
    }

    // This consumes the delimiter.
    fn advance_until(&mut self, what: u8) {
        while let Ok(c) = self.next() {
            if c == what {
                return;
            }
        }
    }

    fn as_literal(&self, slice: &[u8]) -> Result<JsonToken, JsonError> {
        if let Ok(value) = str::from_utf8(slice) {
            let s = String::from(value.trim());
            Ok(if s == "null" {
                JsonToken::Null
            } else if s == "true" {
                JsonToken::True
            } else if s == "false" {
                JsonToken::False
            } else {
                JsonToken::Literal(s)
            })
        } else {
            Err(JsonError::InvalidString)
        }
    }

    fn in_array(&mut self) -> Result<JsonToken, JsonError> {
        // Get the next value, which can be a Literal, an Object or an Array.
        self.eat_ws();
        error_if_eof!(self);
        // Look for either a comma or the closing tag of the array.
        let start = self.pos;
        loop {
            let c = self.next()?;
            if c == b',' {
                return self.as_literal(&self.buffer[start..self.pos - 1]);
            } else if c == b']' {
                if self.pos == start + 1 {
                    // No new value.
                    self.depth -= 1;
                    self.state = TokenizerState::AfterArray;
                    return Ok(JsonToken::EndArray);
                } else {
                    // Don't eat the array closing, and use what was scanned as the value.
                    self.pos -= 1;
                    return self.as_literal(&self.buffer[start..self.pos]);
                }
            }
        }
    }

    fn start(&mut self) -> Result<JsonToken, JsonError> {
        // We only support Objects and Arrays as top level constructs.
        match self.next()? {
            b'{' => {
                self.state = TokenizerState::ExpectProperty;
                self.depth += 1;
                Ok(JsonToken::StartObject)
            }
            b'[' => {
                self.state = TokenizerState::InArray;
                self.depth += 1;
                Ok(JsonToken::StartArray)
            }
            _ => Err(JsonError::UnexpectedCharacter),
        }
    }

    fn expect_value(&mut self) -> Result<JsonToken, JsonError> {
        // Check if this value is an Object, an Array or a Literal
        match self.next()? {
            b'{' => {
                self.state = TokenizerState::ExpectProperty;
                self.depth += 1;
                return Ok(JsonToken::StartObject);
            }
            b'[' => {
                self.state = TokenizerState::InArray;
                self.depth += 1;
                return Ok(JsonToken::StartArray);
            }
            b'}' => {
                self.eat_ws();
                if !self.eof() && self.next()? != b',' {
                    return Err(JsonError::UnexpectedCharacter);
                }
                self.state = TokenizerState::ExpectProperty;
                self.depth -= 1;
                return Ok(JsonToken::EndObject);
            }
            _ => {
                // It's a Literal.
                let start = self.pos - 1;
                let end;
                // We need to reach either a `,` or a `}`
                loop {
                    let c = self.next()?;
                    if c == b',' {
                        end = self.pos - 1;
                        self.state = TokenizerState::ExpectProperty;
                        break;
                    }
                    if c == b'}' {
                        self.pos -= 1;
                        end = self.pos;
                        break;
                    }
                }

                return self.as_literal(&self.buffer[start..end]);
            }
        }
    }

    fn expect_property(&mut self) -> Result<JsonToken, JsonError> {
        self.eat_ws();

        let c = self.next()?;
        // If the first character is no a `"` something is wrong.
        if c != b'"' {
            return Err(JsonError::UnexpectedCharacter);
        }
        // Look for the next `"` and for a `:`
        let start = self.pos;
        self.advance_until(b'"');
        error_if_eof!(self);
        let value = str::from_utf8(&self.buffer[start..self.pos - 1]);
        if value.is_err() {
            return Err(JsonError::InvalidString);
        }
        // Look for the `:`
        self.eat_ws();
        self.advance_until(b':');
        self.eat_ws();
        error_if_eof!(self);
        self.state = TokenizerState::ExpectValue;
        Ok(JsonToken::PropertyName(String::from(value.unwrap())))
    }

    pub fn next_token(&mut self) -> Result<JsonToken, JsonError> {
        // println!("next_token state={:?} pos={} depth={}",
        //          self.state,
        //          self.pos,
        //          self.depth);
        self.eat_ws();
        if self.depth == 0 && self.state != TokenizerState::Start {
            return Ok(JsonToken::Done);
        }

        if self.eof() && self.state == TokenizerState::Start {
            return Ok(JsonToken::Done);
        }
        error_if_eof!(self);

        match self.state {
            TokenizerState::Start => self.start(),
            TokenizerState::ExpectProperty => self.expect_property(),
            TokenizerState::ExpectValue => self.expect_value(),
            TokenizerState::InArray => self.in_array(),
            TokenizerState::AfterArray => {
                self.eat_ws();
                error_if_eof!(self);
                match self.next()? {
                    b'}' => {
                        self.depth -= 1;
                        self.eat_ws();
                        if !self.eof() {
                            let c = self.next()?;
                            if c != b',' {
                                return Err(JsonError::UnexpectedCharacter);
                            }
                        }
                        self.state = TokenizerState::ExpectProperty;
                        return Ok(JsonToken::EndObject);
                    }
                    b',' => {
                        self.state = TokenizerState::ExpectProperty;
                        self.next_token()
                    }
                    _ => Err(JsonError::UnexpectedCharacter),
                }
            }
        }
    }
}
