# SensorWeb Device Firmware

This package contains the firmware for SensorWeb devices. Please see
the file LICENSE for licensing information.

# Prerequisites

- Install [rustup](https://www.rustup.rs)
- Install a [gcc toolchain](https://launchpad.net/gcc-arm-embedded)
- Install openocd
```
sudo apt install openocd
```
- Setup rust nightly:
```
cd sensorweb-firmware
rustup override set nightly
```
- Install xargo
```
cargo install xargo
```

# Build
Build with `./build.sh`

Load on the board to debug with `./run.sh`

Flash with `./flash.sh`

Note that flashing requires the use of cc3200tool, which can be installed by following
the README [here](https://github.com/ALLTERCO/cc3200tool)
