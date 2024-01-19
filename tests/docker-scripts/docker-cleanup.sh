#!/bin/bash

ContainerList=$1
ModuleName=$2

echo "Clean up script launched..."

cd tests
# This script is run at the end of the integration tests.

CONTAINERS_JET_HOST=$(docker ps | grep jet-host-$ModuleName | cut -d " " -f 1)
for ContainerJetHostID in $CONTAINERS_JET_HOST
do
    docker stop $ContainerJetHostID
    docker rm -f $ContainerJetHostID
done

# Here we stop and remove all the containers.
CONTAINERS_ID_LIST=$(cat $ContainerList | grep "container-id" | cut -d '"' -f 4)
for ContainerID in $CONTAINERS_ID_LIST
do
    docker stop $ContainerID
    docker rm -f $ContainerID
done

# Here we clean the 'know_hosts' file of the host
CONTAINERS_PUBKEY_LIST=$(cat $ContainerList | grep "container-pubkey" | cut -d '"' -f 4)
for ContainerPubKey in $CONTAINERS_PUBKEY_LIST
do
    sed -i "/$ContainerPubKey/d" ~/.ssh/known_hosts
done

# Finally, we remove $ContainerList.
rm -f $ContainerList