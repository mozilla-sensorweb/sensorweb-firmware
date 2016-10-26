#!/bin/sh

cat <<END | sudo sh -c 'cat > /etc/udev/rules.d/49-cc3200.rules'
# Setup the FTDI chip on the cc3200 boards.
ATTRS{idVendor}=="0451", ATTRS{idProduct}=="c32a", MODE="0666", GROUP="dialout", RUN+="/sbin/modprobe ftdi-sio", RUN+="/bin/sh -c '/bin/echo 0451 c32a > /sys/bus/usb-serial/drivers/ftdi_sio/new_id'"
END

# Now that we've created the udev file - get it to run without requiring
# an unplug of the USB cable.

sudo udevadm control --reload-rules
sudo udevadm trigger
