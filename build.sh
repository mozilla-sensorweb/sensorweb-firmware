#!/bin/bash

set -e

OPT=debug
for opt in $@; do
    if [ x"$opt" == x"--release" ]; then
        OPT=release
    fi
done

TARGET=thumbv7em-none-eabi
ELF_DIR=target/${TARGET}/${OPT}
FIRMWARE_ELF=${ELF_DIR}/sensorweb-firmware
FIRMWARE_BIN=${ELF_DIR}/sensorweb-firmware.bin

rm -f ${FIRMWARE_ELF}
rm -f ${FIRMWARE_BIN}

xargo build --target=${TARGET} "$@"
arm-none-eabi-size ${FIRMWARE_ELF}
arm-none-eabi-objcopy -O binary ${FIRMWARE_ELF} ${FIRMWARE_BIN}
