#!/bin/bash

. ./parse-args.sh

set -x
cc3200tool -p ${PORT} --sop2 ~dtr --reset prompt write_file ${FIRMWARE_BIN} /sys/mcuimg.bin
