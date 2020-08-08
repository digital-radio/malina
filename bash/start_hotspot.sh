#!/bin/bash

# Before first run orig files from /etc/dnsmasq.conf, /etc/default/hostapd must be created - they will be restored after hotspot goes down
# sudo mv /etc/dnsmasq.conf /etc/dnsmasq.conf.orig
# sudo mv /etc/default/hostapd /etc/default/hostapd.orig

if [[ ! -f "/etc/dnsmasq.conf.orig" ]]; then
    echo "/etc/dnsmasq.conf.orig does not exists."
    return
fi


if [[ ! -f "/etc/default/hostapd.orig" ]]; then
    echo "/etc/default/hostapd.orig does not exists."
    return
fi

# Create /etc/dnsmasq.conf from dnsmasq.conf.hotspot
sudo cp ./dnsmasq.conf.hotspot /etc/dnsmasq.conf

# Create /etc/default/hostapd from /etc/default/hostapd
sudo cp /etc/default/hostapd.orig /etc/default/hostapd
echo 'DAEMON_CONF="/etc/hostapd/hostapd.conf"' >>  /etc/default/hostapd

# Stop hostapd and dnsmasq services if on
service hostapd stop
service dnsmasq stop

# Create the virtual device
/sbin/iw dev wlan0 interface add uap0 type __ap

# If channel is used, fetch current wifi channel. If not, fetch the first wifi channel. 
CHANNEL=`iwlist wlan0 channel | grep Current | sed 's/.*Channel \([0-9]*\).*/\1/g'`
if [ -z "$CHANNEL" ]; then
    CHANNEL=`iwlist wlan0 channel | grep Channel | head -n 1 | sed 's/.*Channel \([0-9]*\).*/\1/g'`
fi
export CHANNEL

# Create /etc/hostapd/hostapd.conf from template and fetched channel
cat ./hostapd.conf.tmpl | envsubst > /etc/hostapd/hostapd.conf

# Bring wifi interface (wlan0) down
ifdown wlan0

# Bring the hotspot interface (uap0) up
ip link set dev uap0 up

ip addr add 192.168.50.1/24 broadcast 192.168.50.255 dev uap0

# Wait
sleep 1

# Restart hostapd and dnsmasq
service hostapd restart
service dnsmasq restart

# Hotspot should be up and running!