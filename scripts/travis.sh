#!/bin/sh

curl https://sh.rustup.rs -sSf -o rustup.sh
chmod +x ./rustup.sh
./rustup.sh -y
export PATH=/home/travis/.cargo/bin:$PATH
rustup override set nightly-2016-11-06
rustup component add rust-src
cargo install xargo
cargo --version
xargo --version
rustc --version
arm-none-eabi-gcc --version

cp src/config.rs.sample src/config.rs
./build.sh
./build.sh --release
