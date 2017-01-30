#!/bin/bash

set -e

XARGO_ARGS="$@"

. ./parse-args.sh

rm -f ${FIRMWARE_ELF}
rm -f ${FIRMWARE_BIN}

xargo build --target=${TARGET} ${XARGO_ARGS}
arm-none-eabi-size ${FIRMWARE_ELF}
arm-none-eabi-objcopy -O binary ${FIRMWARE_ELF} ${FIRMWARE_BIN}
