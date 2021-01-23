FROM rust:1.47

RUN apt-get update \
    && apt-get install -y \
    make \
    build-essential \
    pkg-config \
    libdbus-glib-1-dev \
    dbus-x11 \
    qdbus

WORKDIR /app

# To run dbus process
# dbus-launch --auto-syntax >> $HOME/.bashrc && source $HOME/.bashrc
# export $(dbus-launch)

# To run malina process
# ./rust/target/debug/malina /app/bash/

# To connect to wifi
# curl -XPOST -H 'Content-type: application/json' -d '{"ssid": "test", "password": "blah"}' localhost:8080/networks