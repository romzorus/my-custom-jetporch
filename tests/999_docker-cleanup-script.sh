#!/bin/bash

# This script is run at the end of the integration tests.

# Here we stop and remove all the containers.
CONTAINERS_ID_LIST=$(cat containers-info.json | grep "container-id" | cut -d '"' -f 4)
for ContainerID in $CONTAINERS_ID_LIST
do
    docker stop $ContainerID
    docker rm -f $ContainerID
done

# Here we clean the 'know_hosts' file of the host
CONTAINERS_PUBKEY_LIST=$(cat containers-info.json | grep "container-pubkey" | cut -d '"' -f 4)
for ContainerPubKey in $CONTAINERS_PUBKEY_LIST
do
    sed -i "/$ContainerPubKey/d" ~/.ssh/known_hosts
done

# Finally, we empty 'containers-info.json'.
cat /dev/null > containers-info.json