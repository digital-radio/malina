#!/bin/bash

PWD=$(cd `dirname $0` && pwd)
ROOT_PATH=$(dirname $PWD)

docker build $PWD --file development/Dockerfile --tag malina-dev
docker run -it -v $ROOT_PATH:/app malina-dev bash
