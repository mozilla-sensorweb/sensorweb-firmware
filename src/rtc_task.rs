// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

// This task checks the time from the sensorweb server and set up the local RTC when
// it receives a new message in the queue.

use alloc::arc::Arc;
use cc3200::rtc::RTC;
use cc3200::socket_channel::SocketChannel;
use config;
use core::str;
use freertos_rs::{Duration, FreeRtosError, Task, Queue};
use microjson::{JsonToken, JsonTokenizer};
use MessageKind;
use smallhttp::{Client, HttpHeader};
use smallhttp::traits::Channel;

fn update_rtc() -> Result<(), ()> {
    info!("Checking time from server at {}", config::RTC_URL);

    let start = RTC::get();
    let mut client = Client::new(SocketChannel::new().unwrap());
    let response = client.get(config::RTC_URL)
        .open()?
        .header(HttpHeader::Connection, "close")?
        .response(|_| false)?;

    let mut buffer = [0u8; 128];
    if let Ok(text) = response.body.read_string_to_end(&mut buffer) {
        let end = RTC::get();
        info!("Received response from {} in {}s : {}",
              config::RTC_URL,
              end - start,
              text);
        // We receive a json string like : {"time":1480556487,"isoDate":"2016-12-01T01:41:27Z"}
        let mut tokenizer = JsonTokenizer::new(&text);
        loop {
            let token = tokenizer.next_token()?;
            debug!("Token: |{:?}|", token);
            match token {
                JsonToken::PropertyName(prop_name) => {
                    info!("prop_name is {}", prop_name);
                    if prop_name == "time" {
                        match tokenizer.next_token()? {
                            JsonToken::Literal(value) => {
                                info!("Setting RTC to {}", value);
                                if let Ok(seconds) = value.parse::<u64>() {
                                    RTC::set(seconds as i64);
                                    break;
                                }
                            }
                            _ => {}
                        }
                    }
                }
                JsonToken::Done => {
                    break;
                }
                _ => {}
            }
        }
    } else {
        error!("Failed to read answer from {}", config::RTC_URL);
        return Err(());
    }
    Ok(())
}

// We use the message queue just as wakeup signal, so we don't queue message
// is not important.
pub fn setup_rtc_updater(queue: Arc<Queue<MessageKind>>) -> Result<Task, FreeRtosError> {
    Task::new()
    .name("rtc_updater")
    .stack_size(2048) // 32-bit words
    .start(move || {
        if let Ok(_) = queue.receive(Duration::ms(10000)) {
            // We don't really care about failures there.
            #[allow(unused_must_use)]
            { update_rtc(); }
        }
    })
}