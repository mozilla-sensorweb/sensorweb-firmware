// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

#![no_std]
#![feature(collections)]

extern crate collections;

pub mod json;

pub use json::*;

#[cfg(test)]
mod test {

    use json::{JsonToken, JsonTokenizer};
    use collections::String;

    macro_rules! s {
        ($t:expr) => (String::from($t))
    }

    #[test]
    fn simple_object() {
        let text = r#"{"time":1480556487,"isoDate":"2016-12-01T01:41:27Z"}"#;
        let mut tokenizer = JsonTokenizer::new(&text);

        let expected = [JsonToken::StartObject,
                        JsonToken::PropertyName(s!("time")),
                        JsonToken::Literal(s!("1480556487")),
                        JsonToken::PropertyName(s!("isoDate")),
                        JsonToken::Literal(s!("\"2016-12-01T01:41:27Z\"")),
                        JsonToken::EndObject,
                        JsonToken::Done];

        for i in 0..expected.len() {
            assert_eq!(tokenizer.next_token().unwrap(), expected[i]);
        }
    }

    #[test]
    fn simple_array() {
        let text = r#"[1, "deux", 42 ]"#;
        let mut tokenizer = JsonTokenizer::new(&text);

        let expected = [JsonToken::StartArray,
                        JsonToken::Literal(s!("1")),
                        JsonToken::Literal(s!("\"deux\"")),
                        JsonToken::Literal(s!("42")),
                        JsonToken::EndArray,
                        JsonToken::Done];

        for i in 0..expected.len() {
            assert_eq!(tokenizer.next_token().unwrap(), expected[i]);
        }
    }

    #[test]
    fn array_in_object() {
        let text = r#"{"number":1480556487,"array":[1, "deux", 3]}"#;
        let mut tokenizer = JsonTokenizer::new(&text);

        let expected = [JsonToken::StartObject,
                        JsonToken::PropertyName(s!("number")),
                        JsonToken::Literal(s!("1480556487")),
                        JsonToken::PropertyName(s!("array")),
                        JsonToken::StartArray,
                        JsonToken::Literal(s!("1")),
                        JsonToken::Literal(s!("\"deux\"")),
                        JsonToken::Literal(s!("3")),
                        JsonToken::EndArray,
                        JsonToken::EndObject,
                        JsonToken::Done];

        for i in 0..expected.len() {
            assert_eq!(tokenizer.next_token().unwrap(), expected[i]);
        }
    }

    #[test]
    fn sensor_things_response() {
        let text = r#"{
  "@iot.id": "1",
  "@iot.selfLink": "http://localhost:8080/v1.0/Datastreams(1)",
  "Thing@iot.navigationLink": "http://localhost:8080/v1.0/Datastreams(1)/Thing",
  "Sensor@iot.navigationLink": "http://localhost:8080/v1.0/Datastreams(1)/Sensor",
  "ObservedProperty@iot.navigationLink": "http://localhost:8080/v1.0/Datastreams(1)/ObservedProperty",
  "Observations@iot.navigationLink": "http://localhost:8080/v1.0/Datastreams(1)/Observations",
  "unitOfMeasurement": {
    "name": "PM 2.5 Particulates (ug/m3)",
    "symbol": "μg/m³",
    "definition": "http://unitsofmeasure.org/ucum.html"
  },
  "observationType": "http://www.opengis.net/def/observationType/OGC-OM/2.0/OM_Measurement",
  "description": "Air quality readings",
  "name": "air_quality_readings",
  "observedArea": null
}"#;
        let mut tokenizer = JsonTokenizer::new(&text);

        let expected =
            [JsonToken::StartObject,
             JsonToken::PropertyName(s!("@iot.id")),
             JsonToken::Literal(s!("\"1\"")),
             JsonToken::PropertyName(s!("@iot.selfLink")),
             JsonToken::Literal(s!("\"http://localhost:8080/v1.0/Datastreams(1)\"")),
             JsonToken::PropertyName(s!("Thing@iot.navigationLink")),
             JsonToken::Literal(s!("\"http://localhost:8080/v1.0/Datastreams(1)/Thing\"")),
             JsonToken::PropertyName(s!("Sensor@iot.navigationLink")),
             JsonToken::Literal(s!("\"http://localhost:8080/v1.0/Datastreams(1)/Sensor\"")),
             JsonToken::PropertyName(s!("ObservedProperty@iot.navigationLink")),
             JsonToken::Literal(s!("\"http://localhost:8080/v1.\
                                    0/Datastreams(1)/ObservedProperty\"")),
             JsonToken::PropertyName(s!("Observations@iot.navigationLink")),
             JsonToken::Literal(s!("\"http://localhost:8080/v1.0/Datastreams(1)/Observations\"")),
             JsonToken::PropertyName(s!("unitOfMeasurement")),
             JsonToken::StartObject,
             JsonToken::PropertyName(s!("name")),
             JsonToken::Literal(s!("\"PM 2.5 Particulates (ug/m3)\"")),
             JsonToken::PropertyName(s!("symbol")),
             JsonToken::Literal(s!("\"μg/m³\"")),
             JsonToken::PropertyName(s!("definition")),
             JsonToken::Literal(s!("\"http://unitsofmeasure.org/ucum.html\"")),
             JsonToken::EndObject,
             JsonToken::PropertyName(s!("observationType")),
             JsonToken::Literal(s!("\"http://www.opengis.net/def/observationType/OGC-OM/2.\
                                    0/OM_Measurement\"")),
             JsonToken::PropertyName(s!("description")),
             JsonToken::Literal(s!("\"Air quality readings\"")),
             JsonToken::PropertyName(s!("name")),
             JsonToken::Literal(s!("\"air_quality_readings\"")),
             JsonToken::PropertyName(s!("observedArea")),
             JsonToken::Null,
             JsonToken::EndObject,
             JsonToken::Done];

        for i in 0..expected.len() {
            assert_eq!(tokenizer.next_token().unwrap(), expected[i]);
        }
    }
}
