// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

// This task checks the time from the sensorweb server and set up the local RTC when
// it receives a new message in the queue.

use alloc::arc::Arc;
use cc3200::rtc::RTC;
use cc3200::socket_channel::SocketChannel;
use config;
use freertos_rs::{Duration, FreeRtosError, Task, Queue};
use simple_json::Json;
use smallhttp::Client;
use smallhttp::traits::Channel;

fn update_rtc() {
    info!("Checking time from server at {}", config::RTC_URL);

    // let text = "{\"time\":1480457702,\"isoDate\":\"2016-11-29T22:15:02Z\"}";
    // let result = Json::parse(text).unwrap();
    // info!("parsed : {:?}", result);
    // return;

    let start = RTC::get();
    let mut client = Client::new(SocketChannel::new().unwrap());
    let response = client.get(config::RTC_URL)
        .open()
        .unwrap()
        .send(&[])
        .unwrap()
        .response(|_| false)
        .unwrap();

    let mut buffer = [0u8; 128];
    let len = buffer.len();
    if let Ok(text) = response.body.read_string_to_end(&mut buffer) {
        let end = RTC::get();
        info!("Received response from {} in {}s : {}",
              config::RTC_URL,
              end - start,
              text);
        let result = Json::parse(text).unwrap();
    } else {
        error!("Failed to read answer from {}", config::RTC_URL);
    }

}

// We use the message queue just as wakeup signal, so we don't queue message
// is not important.
pub fn setup_rtc_updater(queue: Arc<Queue<u8>>) -> Result<Task, FreeRtosError> {
    Task::new()
    .name("rtc_updater")
    .stack_size(2048) // 32-bit words
    .start(move || {
        if let Ok(_) = queue.receive(Duration::ms(10000)) {
            update_rtc();
        }
    })
}