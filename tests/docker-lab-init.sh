#!/bin/bash

# To have a passwordless root access, we need keys. If it hasn't already been done,
# let's generate keys in order to have it allowed in the containers later.
if [ ! -f controller_key ] || [ ! -f controller_key.pub ]
then
    rm -f controller_key*
    ssh-keygen -t ed25519 -f controller_key -N "" -q
fi

# The 'containers-info.json' file will be used by the actual integration tests, both as a
# file locking item and as a source of information on the hosts (inventory building for example).
# For now, we a building it in JSON format, as an array of objects.

# Opening an array in the JSON file
echo "[" >> containers-info.json

# Here we list all the Dockerfiles available
DOCKERFILES_LIST=$(find Dockerfiles-folder -type f -name "Dockerfile-*")

for Dockerfile in $DOCKERFILES_LIST
do
    # Building the image with an explicit name
    OsName=$(basename $Dockerfile | cut -d "-" -f 2)
    docker build -f $Dockerfile -t jet-host-$OsName:latest .

    # Running a container based on this image and retrieving informations on it
    ContainerID=$(docker run -d jet-host-$OsName)
    ContainerIP=$(docker inspect -f '{{range.NetworkSettings.Networks}}{{.IPAddress}}{{end}}' $ContainerID)
    ContainerPubKey=$(ssh-keyscan $ContainerIP)

    # Filling the JSON file with container's informations
    echo {\"container-name\" : \"jet-host-$OsName\", >> containers-info.json
    echo \"container-id\" : \"$ContainerID\", >> containers-info.json
    echo \"container-ip\" : \"$ContainerIP\", >> containers-info.json
    echo \"container-pubkey\" : \"$ContainerPubKey\"}, >> containers-info.json

    # Having the container's key allowed on the host to avoid the usual StrictHostKeyChecking issues
    if [ -e ~/.ssh/known_hosts ]
    then
        echo $ContainerPubKey >> ~/.ssh/known_hosts
    else
        touch ~/.ssh/known_hosts
        echo $ContainerPubKey >> ~/.ssh/known_hosts
    fi
    
done

# Closing the array of the JSON file
echo "]" >> containers-info.json