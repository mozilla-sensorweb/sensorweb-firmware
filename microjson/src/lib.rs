// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

#![no_std]
#![feature(collections)]

extern crate collections;

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
    ExpectComma,
    InArray,
}

pub struct JsonTokenizer<'a> {
    buffer: &'a [u8],
    len: usize,
    pos: usize,
    state: TokenizerState,
    depth: usize,
}

macro_rules! eof {
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
        loop {
            if self.eof() {
                return;
            }
            let c = self.next().unwrap(); // We know this won't panick since we're not eof.
            if c != b' ' && c != b'\t' && c != b'\r' && c != b'\n' {
                self.pos -= 1;
                break;
            }
        }
    }

    // This consumes the delimiter.
    fn advance_until(&mut self, what: u8) {
        loop {
            if self.eof() {
                return;
            }
            // We know this won't panick since we're not eof.
            if what == self.next().unwrap() {
                break;
            }
        }
    }

    fn in_array(&mut self) -> Result<JsonToken, JsonError> {
        // Get the next value, which can be a Literal, an Object or an Array.
        self.eat_ws();
        eof!(self);
        // Look for either a comma or the closing tag of the array.
        let start = self.pos;
        loop {
            let c = self.next()?;
            if c == b',' {
                let value = str::from_utf8(&self.buffer[start..self.pos - 1]);
                if value.is_err() {
                    return Err(JsonError::InvalidString);
                }
                return Ok(JsonToken::Literal(String::from(value.unwrap())));
            } else if c == b']' {
                if self.pos == start + 1 {
                    // No new value.
                    self.depth -= 1;
                    self.state = TokenizerState::ExpectComma;
                    return Ok(JsonToken::EndArray);
                } else {
                    // Don't eat the array closing, and use what was scanned as the value.
                    self.pos -= 1;
                    let value = str::from_utf8(&self.buffer[start..self.pos]);
                    if value.is_err() {
                        return Err(JsonError::InvalidString);
                    }
                    return Ok(JsonToken::Literal(String::from(value.unwrap())));
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
                return Ok(JsonToken::StartObject);
            }
            b'[' => {
                self.state = TokenizerState::InArray;
                self.depth += 1;
                return Ok(JsonToken::StartArray);
            }
            _ => return Err(JsonError::UnexpectedCharacter),
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

                let value = str::from_utf8(&self.buffer[start..end]);
                if value.is_err() {
                    return Err(JsonError::InvalidString);
                }

                return Ok(JsonToken::Literal(String::from(value.unwrap())));
            }
        }
    }

    fn expect_comma(&mut self) -> bool {
        self.eat_ws();
        if self.eof() {
            return false;
        }
        if self.next().unwrap() == b',' {
            self.state = TokenizerState::ExpectProperty;
            return true;
        }
        false
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
        eof!(self);
        let value = str::from_utf8(&self.buffer[start..self.pos - 1]);
        if value.is_err() {
            return Err(JsonError::InvalidString);
        }
        // Look for the `:`
        self.eat_ws();
        self.advance_until(b':');
        self.eat_ws();
        eof!(self);
        self.state = TokenizerState::ExpectValue;
        Ok(JsonToken::PropertyName(String::from(value.unwrap())))
    }

    pub fn next_token(&mut self) -> Result<JsonToken, JsonError> {
        self.eat_ws();
        if self.depth == 0 && self.state != TokenizerState::Start {
            return Ok(JsonToken::Done);
        }

        if self.eof() && self.state == TokenizerState::Start {
            return Ok(JsonToken::Done);
        }
        eof!(self);

        match self.state {
            TokenizerState::Start => self.start(),
            TokenizerState::ExpectProperty => self.expect_property(),
            TokenizerState::ExpectValue => self.expect_value(),
            TokenizerState::InArray => self.in_array(),
            TokenizerState::ExpectComma => {
                if self.expect_comma() {
                    self.next_token()
                } else {
                    Err(JsonError::UnexpectedCharacter)
                }
            }
        }
    }
}