FROM openeuler/openeuler:20.03-lts-sp2
# 安装依赖包
RUN yum update -y &&\
    yum install -y tar unzip wget git vim cmake make passwd readline-devel libaio-devel.x86_64 bison m4 flex ncurses-devel glibc-devel patch libcgroup-devel.x86_64 zstd-devel.x86_64 ntpdate gdb tcpdump 

# 安装Python2.7
WORKDIR /root
RUN wget https://www.python.org/ftp/python/2.7.9/Python-2.7.9.tgz &&\
    tar -zxvf Python-2.7.9.tgz -C /usr/local &&\
    rm -f Python-2.7.9.tgz
WORKDIR /usr/local/Python-2.7.9
RUN ./configure --prefix=/usr/local/python-2.7.9 &&\
    make &&\
    make install &&\
    ln -s /usr/local/python-2.7.9/bin/python /usr/bin/python

# 创建omm用户，openGauss基于postgresql，和postresql一样，不能在root用户下启动
RUN echo '123456' | passwd --stdin root &&\
    groupadd dbgrp -g 2000 &&\
    useradd omm -g 2000 -u 2000 &&\
    echo '123456' | passwd --stdin omm &&\
    # 赋予访问摄像头的权限
    usermod -a -G video omm &&\
    usermod -a -G dbus omm

# 下载源代码及编译所需第三方依赖
RUN mkdir /opt/gaussdb &&\
    mkdir /usr2 &&\
    mkdir /usr2/compile
WORKDIR /usr2/compile
RUN wget https://opengauss.obs.cn-south-1.myhuaweicloud.com/latest/binarylibs/openGauss-third_party_binarylibs_openEuler_x86_64.tar.gz &&\
    tar zxvf openGauss-third_party_binarylibs_openEuler_x86_64.tar.gz &&\
    mv openGauss-third_party_binarylibs_openEuler_x86_64 binarylibs &&\
    rm -f openGauss-third_party_binarylibs_openEuler_x86_64.tar.gz &&\
    git clone https://gitee.com/pengbo2/opensource-internship.git &&\
    cd opensource-internship &&\
    git checkout zerotrust-5 &&\
    cp env.sh /etc/profile.d &&\
    chmod 777 /etc/profile.d/env.sh &&\
    chown -R omm /opt/gaussdb &&\
    chown -R omm /usr2/compile &&\
    chmod -R 755 /usr2/compile &&\
    chmod -R 700 /opt/gaussdb

# 安装需要的Python包
WORKDIR /root
RUN wget https://opengauss.obs.cn-south-1.myhuaweicloud.com/3.1.0/x86_openEuler/openGauss-3.1.0-openEuler-x86_64-Python.tar.gz &&\
    tar zxvf openGauss-3.1.0-openEuler-x86_64-Python.tar.gz -C /usr/lib/python3.7/site-packages &&\
    chmod -R 755 /usr/lib/python3.7/site-packages &&\
    rm -f openGauss-3.1.0-openEuler-x86_64-Python.tar.gz &&\
    python3 -m pip install numpy==1.19.5 cicflowmeter requests pandas scikit-learn opencv-python-headless &&\
    export MS_VERSION=1.7.1 &&\
    python3 -m pip install https://ms-release.obs.cn-north-4.myhuaweicloud.com/${MS_VERSION}/MindSpore/cpu/x86_64/mindspore-${MS_VERSION/-/}-cp37-cp37m-linux_x86_64.whl --trusted-host ms-release.obs.cn-north-4.myhuaweicloud.com -i https://pypi.tuna.tsinghua.edu.cn/simple

#使用omm用户进行编译
USER omm
WORKDIR /usr2/compile/opensource-internship/openGauss-server
# 开始编译
RUN source /etc/profile.d/env.sh &&\
    ./configure --gcc-version=7.3.0 CC=g++ CFLAGS="-O2 -g3 -pthread" --prefix=$GAUSSHOME --3rd=$BINARYLIBS --enable-thread-safety --with-readline --without-zlib &&\
    make -sj4 &&\
    make -sj4 install &&\
    gs_initdb --pgdata=/opt/gaussdb/ --nodename=dn_6001 --encoding=utf-8 -w gauss@123 &&\
    touch /home/omm/zerotrust.log

USER ROOT
WORKDIR /root