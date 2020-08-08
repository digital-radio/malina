#!/bin/bash

set -e

if [ "$#" -ne 2 ]; then
    echo "Usage: ./wifi_start.sh <SSID> <PASSWORD>"
    exit 1
fi

DIRECTORY=$(cd `dirname $0` && pwd)
SSID=$1
PASSWORD=$2

cat "$DIRECTORY/interface.tmpl" | SSID="$1" PASSWORD="$2" envsubst > /etc/network/interfaces.d/wlan0
ifdown wlan0 && ifup wlan0
