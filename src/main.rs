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

extern crate smallhttp;

#[macro_use]
extern crate log;

#[macro_use]
extern crate collections;

use cc3200::cc3200::{Board, I2C, I2COpenMode, LedEnum};
use cc3200::simplelink::{self, SimpleLink};
use cc3200::socket_channel::SocketChannel;

use cc3200::i2c_devices::TemperatureSensor;
use cc3200::tmp006::TMP006;

use freertos_rs::{CurrentTask, Duration, Task};
use smallhttp::Client;
use smallhttp::traits::Channel;

static VERSION: &'static str = "1.0";

mod config;
mod wlan;

fn run() -> Result<(), wlan::Error> {

    Board::led_configure(&[LedEnum::LED1]);

    try!(SimpleLink::start_spawn_task());
    try!(wlan::wlan_station_mode());

    let i2c = I2C::open(I2COpenMode::MasterModeFst).unwrap();
    let temp_sensor = TMP006::default(&i2c).unwrap();

    println!("Will now send {} temperature sensing to the server...",
             config::SENSOR_READING_COUNT);

    for _ in 0..config::SENSOR_READING_COUNT {
        let temperature: u32 = (temp_sensor.get_temperature().unwrap() * 10.0) as u32;
        info!("Feels like {}.{} C", temperature / 10, temperature % 10);
        let mut client = Client::new(SocketChannel::new().unwrap());
        let response = client.get(config::SERVER_URL)
            .open()
            .unwrap()
            .send(&[])
            .unwrap()
            .response(|_| false)
            .unwrap();
        let mut buffer = [0u8; 256];
        info!("Received {}",
              response.body.read_string_to_end(&mut buffer).unwrap());
        CurrentTask::delay(Duration::ms(1000))
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

    let _client = {
        Task::new()
            .name("client")
            .stack_size(2048) // 32-bit words
            .start(|| {
                match run() {
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
