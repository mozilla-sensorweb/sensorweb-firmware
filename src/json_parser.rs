// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

// From https://github.com/Geal/nom/blob/master/tests/json.rs with modifications
// to build in a #![no_std] environment.

#![no_std]

use nom::{digit, alphanumeric, IResult};

use core::str::{self, FromStr};
use collections::string::String;
use collections::Vec;

// Poor man's replacement for a HashMap.
#[derive(Debug, PartialEq)]
struct KeyValSet<K, V> {
    data: Vec<(K, V)>,
}

impl<K, V> KeyValSet<K, V> {
    fn new() -> Self {
        KeyValSet { data: Vec::new() }
    }

    fn insert(&mut self, key: K, value: V) {
        self.data.push((key, value));
    }
}

#[derive(Debug, PartialEq)]
pub enum JsonValue {
    Str(String),
    Num(f32),
    Array(Vec<JsonValue>),
    Object(KeyValSet<String, JsonValue>),
}

// FIXME: since we already parsed a serie of digits and dots,
// we know it is correct UTF-8. no need to use from_utf8 to
// verify it is correct
// FIXME: use alt_complete (implement ws for alt_complete)
named!(unsigned_float <f32>, map_res!(
  map_res!(
    recognize!(
      alt_complete!(
        delimited!(digit, tag!("."), opt!(complete!(digit))) |
        delimited!(opt!(digit), tag!("."), digit)            |
        digit
      )
    ),
    str::from_utf8
  ),
  FromStr::from_str
));

named!(float<f32>, map!(
  pair!(
    opt!(alt!(tag!("+") | tag!("-"))),
    unsigned_float
  ),
  |(sign, value): (Option<&[u8]>, f32)| {
    sign.and_then(|s| if s[0] == ('-' as u8) { Some(-1f32) } else { None }).unwrap_or(1f32) * value
  }
));

// FIXME: verify how json strings are formatted
named!(string<&str>,
  delimited!(
    tag!("\""),
    map_res!(escaped!(call!(alphanumeric), '\\', is_a!(&b"\"n\\"[..])), str::from_utf8),
    tag!("\"")
  )
);

named!(array < Vec<JsonValue> >,
  ws!(
    delimited!(
      tag!("["),
      separated_list!(tag!(","), json_to_value),
      tag!("]")
    )
  )
);

named!(key_value<(&str,JsonValue)>,
  ws!(
    separated_pair!(
      string,
      tag!(":"),
      json_to_value
    )
  )
);

named!(hash< KeyValSet<String,JsonValue> >,
  ws!(
    map!(
      delimited!(
        tag!("{"),
        separated_list!(tag!(","), key_value),
        tag!("}")
        ),
      |tuple_vec| {
        let mut h: KeyValSet<String, JsonValue> = KeyValSet::new();
        for (k, v) in tuple_vec {
          h.insert(String::from(k), v);
        }
        h
      }
    )
  )
);

named!(json_to_value<JsonValue>,
  ws!(
    alt!(
      hash   => { |h|   JsonValue::Object(h)            } |
      array  => { |v|   JsonValue::Array(v)             } |
      string => { |s|   JsonValue::Str(String::from(s)) } |
      float  => { |num| JsonValue::Num(num)             }
    )
  )
);

pub fn parse_json(buffer: &[u8]) -> IResult<&[u8], JsonValue> {
    json_to_value(buffer)
}

// #[test]
// fn hash_test() {
// let test = &b"  { \"a\"\t: 42,
// \"b\": \"x\"
// }";
//
// FIXME: top level value must be an object?
// println!("{:?}", value(&test[..]));
// assert!(false);
// }
//
// #[test]
// fn parse_example_test() {
// let test = &b"  { \"a\"\t: 42,
// \"b\": [ \"x\", \"y\", 12 ] ,
// \"c\": { \"hello\" : \"world\"
// }
// }";
//
// FIXME: top level value must be an object?
// println!("{:?}", value(&test[..]));
// assert!(false);
// }
//