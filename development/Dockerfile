FROM rust:1.47

RUN apt-get update \
    && apt-get install -y \
    make \
    build-essential \
    pkg-config \
    libdbus-glib-1-dev \
    dbus-x11

WORKDIR /app

# To run dbus process
# export $(dbus-launch)
