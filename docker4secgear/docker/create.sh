#! /bin/bash
#test
echo "pull docker with sgx module"
echo ""
docker pull tozd/sgx:latest
echo "docker start"
echo "========================================================"
docker run -d --device /dev/isgx --device /dev/mei0 --name iccnsg tozd/sgx:ubuntu-xenial
echo "copy wallfacer enclave"
echo "========================================================"
docker cp ./docker4secgear/encrypt iccnsg:/home
docker cp ./docker4secgear/verify iccnsg:/home
echo "container start"
echo "========================================================"
docker exec -t -i iccnsg /bin/bash
echo "install sdk"
echo "========================================================"
yum install python
yum install g++
yum install secGear
yum install secGear-devel
