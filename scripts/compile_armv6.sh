#!/bin/bash
set -euo pipefail
IFS=$'\n\t'

# https://github.com/mdirkse/rust_armv6

WORKDIR="/work"
docker run --rm -t \
       -u "$(id -u):$(id -g)" \
       -e "HOME=${WORKDIR}" \
       -e "PKG_CONFIG_ALLOW_CROSS=1" \
       -w "${WORKDIR}" \
       -v "$(pwd):${WORKDIR}" \
       -v "${HOME}/.cargo/registry:/usr/local/cargo/registry" \
       mdirkse/rust_armv6:latest

