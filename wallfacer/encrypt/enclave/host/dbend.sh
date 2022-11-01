#! /bin/bash
#test
echo "write start"
echo ""

echo "grinfo copy to docker and wait to wirte"
sudo docker cp grinfo opengauss:/home/omm
echo "container start"
echo "========================================================"
docker start opengauss
docker ps -a
docker exec -it opengauss sh

echo "container stop"
echo "========================================================"
docker stop opengauss
docker ps -a
echo "docker stop"
echo "========================================================"
systemctl stop docker

echo ""
echo "write stop"
