#! /bin/bash
#test
echo "read start"
echo ""

docker -v
echo "docker start"
echo "========================================================"
systemctl start docker
docker ps -a
echo "container start"
echo "========================================================"
docker start opengauss
docker ps -a
docker exec -it opengauss sh
echo "container stop"
echo "========================================================"
docker stop opengauss
docker ps -a


echo "grinfo copy to host and wait to settle"
sudo cp /var/lib/docker/volumes/dbvol/_data/omm/grinfo  /opt/secGear/debug/examples/helloworld/host
sudo chmod 777 /opt/secGear/debug/examples/helloworld/host/grinfo


echo ""
echo "read stop"

