#!/usr/bin/env bash

SHARED_DIR=solution
IMG_NAME=corewar_play
CONTAINER_NAME=playground

if [[ ! -d $SHARED_DIR ]]; then 
    mkdir $SHARED_DIR
fi
if [[ $(docker ps -a | grep $CONTAINER_NAME) ]]; then
    echo "Attach to running container"
    docker start $CONTAINER_NAME
    docker exec -it $CONTAINER_NAME bash
    exit 1
fi
if [[ ! $(docker images | grep $IMG_NAME) ]]; then
    docker build --platform=linux/amd64 -t $IMG_NAME .
fi
echo "Start new container"
docker run -v "$(pwd)/$SHARED_DIR":/corewar/$SHARED_DIR --name=$CONTAINER_NAME -it $IMG_NAME 


