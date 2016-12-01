// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

// We won't use the usual `main` function. We are going to use a different "entry point".
#![no_main]

// We won't use the standard library because it requires OS abstractions like threads and files and
// those are not available in this platform.
#![no_std]
#![feature(alloc)]
#![feature(collections)]

#[macro_use]
extern crate cc3200;
extern crate alloc;
extern crate freertos_rs;
extern crate freertos_alloc;
extern crate microjson;
extern crate smallhttp;

#[macro_use]
extern crate log;

#[macro_use]
extern crate collections;

use alloc::arc::Arc;
use cc3200::cc3200::{Board, LedEnum};
use cc3200::simplelink::{self, SimpleLink};

use core::str;

use freertos_rs::{CurrentTask, Duration, Queue, Task};

static VERSION: &'static str = "1.0";

#[derive(Clone, Copy)]
pub enum MessageKind {
    UpdateRtc,
}

mod config;
mod rtc_task;
mod wlan;

fn run(queue: Arc<Queue<MessageKind>>) -> Result<(), wlan::Error> {

    Board::led_configure(&[LedEnum::LED1]);

    try!(SimpleLink::start_spawn_task());
    try!(wlan::wlan_station_mode());

    // Wifi is up, set up the RTC task and ask for an update.
    #[allow(unused_must_use)]
    {
        rtc_task::setup_rtc_updater(queue.clone())
            .and_then(|_| queue.send(MessageKind::UpdateRtc, Duration::ms(15)));
    }

    loop {
        println!("sleep");
        CurrentTask::delay(Duration::ms(10000));
    }

    // Power off the network processor.
    try!(SimpleLink::stop(simplelink::SL_STOP_TIMEOUT));
    Ok(())
}

// Conceptually, this is our program "entry point". It's the first thing the microcontroller will
// execute when it (re)boots. (As far as the linker is concerned the entry point must be named
// `start` (by default; it can have a different name). That's why this function is `pub`lic, named
// `start` and is marked as `#[no_mangle]`.)
//
// Returning from this function is undefined because there is nothing to return to! To statically
// forbid returning from this function, we mark it as divergent, hence the `fn() -> !` signature.
#[no_mangle]
pub fn start() -> ! {

    Board::init();

    println!("Welcome to SensorWeb {}", VERSION);

    let queue = Arc::new(Queue::new(10).unwrap());

    let _client = {
        Task::new()
            .name("client")
            .stack_size(2048) // 32-bit words
            .start(|| {
                match run(queue) {
                    Ok(())  => { println!("sensorweb succeeded"); },
                    Err(e)  => { println!("sensorweb failed: {:?}", e); },
                };
                loop {}
            })
            .unwrap()
    };

    Board::start_scheduler();

    // The only reason start_scheduler should fail is if there wasn't enough
    // heap to initialize the IDLE and timer tasks

    loop {}
}
