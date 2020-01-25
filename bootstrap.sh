#!/bin/sh

# install rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

cd frontend
npm i
npm run build
cp -r dist ../backend/dist
cd ..

cd backend
cargo run

# PORT_DOCKER=3000
# PORT_LOCAL=8080
# PROJECT_NAME="audio-guesser"
# docker build -t $PROJECT_NAME .
# docker run --env PORT=$PORT_DOCKER -p $PORT_LOCAL:$PORT_DOCKER -d $PROJECT_NAME
# echo "App is running on: 127.0.0.1:$PORT_LOCAL"
# xdg-open http://127.0.0.1:$PORT_LOCAL > /dev/null 2>&1 &
