# openEuler secGear Docker

#### 简介

该容器镜像为基于openEuler系统 带有Intel SGX SDK的Docker映像，于`\home`路径下封装了`Wallfacer`平台接口，身份认证模块位于`\home\wallfacer_code\verify`下，动态密钥生成模块位于`\home\wallfacer_code\encrypt`下。

#### 部署

由于`gitee`限制，无法上传完整镜像，请基于`/docker`路径下运行`./create.sh`进行容器端的基本支持环境部署。

#### 运行

必须在宿主机上加载Intel SGX内核模块 并且在运行它时必须将它提供给容器:

```bash
docker run -d --device /dev/isgx --device /dev/mei0 --name test-secgear tozd/sgx:ubuntu-xenial
docker exec -t -i test-sgx bash
```

secGear SDK部署于`/opt/intel/sgxsdk`下，需要在bash脚本中加载SDK环境。

```bash
source /opt/intel/sgxsdk/environment
```

## 云容器引擎CCE管理Wallfacer项目容器

#### 使用步骤：

完整的云容器引擎使用流程包含以下步骤

![img](https://support.huaweicloud.com/qs-cce/zh-cn_image_0000001325721374.png)

#### 创建集群

* 登录CCE控制台
* 在CCE集群下单击“创建”
* 配置集群参数：
  * 集群版本：建议选择最新的版本，对应Kubernetes社区基线版本。
  * 网络模型：默认即可。
  * 控制节点数：默认选择“3”。
  * 集群规模：按需设置。
  * 控制节点子网：根据实际部署需求设置集群Master节点所在的子网。
  * 容器网段：按默认配置即可。

#### 镜像服务

* 使用解压工具将`./image`路径下的`iccnsg.tar`文件进行解压缩
* 使用`UltraSO`工具，对解压得到的iccnsg文件夹进行刻录，得到iccnsg.iso镜像文件

* 选择私有镜像服务
* 选择导入镜像——导入私有镜像
* 选择系统盘镜像，并选择现有桶/或创建桶，将本地iccnsg.iso挂载入桶中
* 选择配置信息
  * ECS系统盘镜像
  * **选择x86架构**
  * BIOS启动方式
  * 操作系统选择openEuler 20.03 64bit

#### 创建节点

* 单击创建的集群，进入集群控制台
* 在左侧菜单栏选择节点管理，单击右上角“创建节点”，在弹出的页面中配置节点的参数。
