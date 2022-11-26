#! /bin/bash
set +x
image="gauss_zt"
container="gauss_with_zerotrust"

docker run -it -d --name="${container}" --device=/dev/video0 -u root ${image}