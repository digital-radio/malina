# https://github.com/diwic/dbus-rs/issues/184
# https://github.com/mdirkse/rust_armv6/blob/master/Dockerfile

FROM mdirkse/rust_armv6:latest

RUN apt-get update
RUN apt-get install -y \
    libdbus-glib-1-dev:armhf \
    libsystemd-dev:armhf \
    libgcrypt-dev:armhf \
    liblz4-dev:armhf \
    liblzma-dev:armhf \
    libselinux-dev:armhf

CMD cargo build
