#!/bin/sh
### BEGIN INIT INFO
# Provides:          cc3200-serial
# Required-Start:    $local_fs $syslog
# Required-Stop:     $local_fs $syslog
# Default-Start:     2 3 4 5
# Default-Stop:      0 1 6
# Short-Description: cc3200 serial ports
# Description:       Brings up the serial ports for the custom FTDI VID/PID.
### END INIT INFO

if [ $(basename "$0") == "cc3200-serial.sh" ]; then
    # The script was run from the repository (because it has the .sh extension)
    sudo cp cc3200-serial.sh /etc/init.d/cc3200-serial
    sudo chmod +x /etc/init.d/cc3200-serial
    sudo chown root:root /etc/init.d/cc3200-serial
    sudo update-rc.d cc3200-serial defaults
    sudo update-rc.d cc3200-serial enable
    exit 0
fi

modprobe ftdi-sio
echo 0451 c32a > /sys/bus/usb-serial/drivers/ftdi_sio/new_id
