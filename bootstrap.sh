#!/bin/sh
PORT_DOCKER=3000
PORT_LOCAL=8080
PROJECT_NAME="audio-guesse"
docker build -t $PROJECT_NAME .
docker run --env PORT=$PORT_DOCKER -p $PORT_LOCAL:$PORT_DOCKER -d $PROJECT_NAME
echo "App is running on: 127.0.0.1:$PORT_LOCAL"
xdg-open http://127.0.0.1:$PORT_LOCAL > /dev/null 2>&1 &
