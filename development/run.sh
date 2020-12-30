#!/bin/bash

PWD=$(cd `dirname $0` && pwd)
ROOT_PATH=$(dirname $PWD)

docker build $PWD --file development/Dockerfile --tag malina-dev
docker run --rm -it -v $ROOT_PATH:/app --name malina-dev malina-dev bash
