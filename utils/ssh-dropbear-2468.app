#!/bin/sh

port=2468

if [ ! -f "/mnt/secure/su" ]; then
    echo "Device not rooted, can't start ssh server"
    dialog 3 "" "Device not rooted, can't start ssh server" "OK"
fi

# Ensure enabled Wifi
if [ ! -d "/sys/class/net/eth0" ]; then
  netagent net on
fi
dialog 1 "" "Connecting, please wait..." "" & sleep 1; kill "$!"
# check connected to wifi network - device connects to last used and available network
if test "$(cat /sys/class/net/eth0/carrier)" -e 0; then
  sleep 5
  while test "$(cat /sys/class/net/eth0/carrier)" -e 0; do
    dialog 5 "" "Still attempting to connect to a wireless network!  Wait?" "Yes" "No"
    if [ $? != 1 ]; then exit; fi
    netagent connect
    sleep 3
  done
fi

echo "Starting SSH daemon (dropbear).."
/mnt/secure/su /sbin/dropbear -p $port -G ""
