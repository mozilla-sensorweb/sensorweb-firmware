# Helper script used by build.sh, run.sh, and flash.sh for parsing arguments

OPT=debug

while [[ $# -gt 0 ]]; do
    case "$1" in
        --release)
            OPT=release
            ;;
    esac
    shift
done

TARGET=thumbv7em-none-eabi
ELF_DIR=target/${TARGET}/${OPT}
FIRMWARE_ELF=${ELF_DIR}/sensorweb-firmware
FIRMWARE_BIN=${ELF_DIR}/sensorweb-firmware.bin
