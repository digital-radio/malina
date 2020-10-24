#!/bin/bash

PWD=$(dirname $(cd `dirname $0` && pwd))

docker build $PWD --file development/Dockerfile --tag malina-dev
docker run -it -v $PWD:/app malina-dev bash
