#!/usr/bin/env bash

# build the ÐecisionToolkit binary
cargo +stable build --release --target x86_64-unknown-linux-musl

# set the container name
NAME=dsntk

# set the container version taken from ÐecisionToolkit binary
VERSION=$(cargo +stable run --release --target x86_64-unknown-linux-musl -- --version)

# remove existing Docker container
echo ""
container_id="$(docker container list -af name=dsntk -q)"
if [ -z "$container_id" ]
then
  echo "$NAME container not found, skipping deletion"
else
  echo "$NAME container found, deleting"
  docker rm -f "$container_id"
  echo "$NAME container deleted"
fi
echo ""

# remove existing Docker image
echo ""
image_id="$(docker images -f reference=$NAME -q)"
if [ -z "$image_id" ]
then
  echo "$NAME image not found, skipping deletion"
else
  echo "$NAME image found, deleting"
  docker rmi "$image_id"
  echo "$NAME image deleted"
fi
echo ""

# build new Docker image
docker build -t "$NAME:$VERSION" .

# start new Docker container
docker run --name $NAME -d -p 22022:22022 "$NAME:$VERSION"

# display logs from running Docker container
docker logs -f "$NAME"

# press Ctrl+C to stop following the log file :)
