#!/bin/sh

port=2468

if [ ! -f "/mnt/secure/su" ]; then
    echo "Device not rooted, can't start ssh server"
    dialog 3 "" "Device not rooted, can't start ssh server" "OK"
fi

echo "Starting SSH daemon (dropbear).."
/mnt/secure/su /sbin/dropbear -p $port -G ""
