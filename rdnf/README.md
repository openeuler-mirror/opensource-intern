## 简介
+ 此项目是*tdnf*的rust实现版，主要依托于*rpm-devel*，*libsolv-devel*
+ 目前已经实现了核心功能即 *repolist*,*makecache*,*search*,*install*,*reinstall*,*remove*,*update*,*info*
+ 此项目准确的说是rpm-devel和libsolv-devel的整合品。故含有大量的代码是rust对c的ffi绑定，具有一定参考价值。
+ 项目局限性：tdnf主要是为了photon而生，而photon是轻量级容器环境，故软件包仓库的软件数量少，元数据信息少，tdnf的性能可以应付，但对于像fedora拥有丰富的软件包，tdnf解决软件包依赖冲突问题需要长达接近20s的时间。故本项目有进一步改进的空间。

### 项目解析
1. 首先解析命令行参数 clap是rust生态主流的命令行编程框架，简单高效。
2. 解析配置文件 */etc/dnf/dnf.conf*, 使用config crates包解析，映射成 rust对象。
3. 读取 */etc/yum.repos.d/* 文件夹下所有.repo文件，读取各个软件仓库的数据, 映射成rust对象。
4. 初始化libsolv，创建pool对象。

上述5个步骤，整合成Rdnf，形成全局上下文环境。
#### repolist
此命令在于简单显示已经读取的repo仓库信息，可显示已启用或禁用的repo仓库。
#### *makecache*
此命令至关重要，是整个项目的重要基石，即读取已启用的repo的仓库，下载镜像站的索引文件、软件包的元数据，基于libsolv生成.solv格式的缓存文件，若有缓存文件，跳过1,2,3。
1. 首先下载 repomd.xml文件，有两种方式：
    + 基于.repo配置文件，若有baseurl配置项，即baseurl+*repodata/repomd.xml* 即可得 repomd.xml的下载链接
    + 基于.repo配置文件，若无baseurl但有metalink配置项，下载metalink.xml文件，解析metalink.xml文件，可获得repomd.xml的下载链接。
2. 解析 repomd.xml文件，含有 primary,filelists,other,updateinfo等xml文件的元数据，再基于baseurl即可得对应的下载链接，这些xml文件含有对应repo镜像仓库的软件包元数据，保存于/var/cache/rdnf/repodata文件夹下。
3. 基于libsolv,将repo仓库的primary等xml文件生成.solv文件，保存于/var/cache/rdnf/solvcache/文件夹下
4. 再将.solv加载到pool中,pool可以理解为是libsolv环境中的上下文环境

#### search
搜索软件包，基于makecache命令，将xml文件生成.solv，再加载到pool，利用pool和libsolv提供的接口可轻松搜索软件包的元数据信息。

#### install,remove,update,reinstall
这几个命令操作类似。
主要思路为两大部分，先是基于libsolv解决软件包依赖冲突问题，再使用rpm-devel解决rpm软件包具体安装问题。
+ 使用是prepare_all_pkgs, 初步解决是否已经安装，安装的是否是最新版本的问题，将已经初步处理过，需要进一步解决的软件包id放入queue_goal中，在goal中完成主要通过*solver_solve*解决软件包依赖冲突问题，生成安装或卸载列表，然后基于列表中的软件包id获取软件包的详细信息，例如软件包架构，版本，大小的信息。
+ 生成rpmts，可以理解为rpm-devel环境中的上下文环境,根据上述的软件包列表，下载软件包，若有必要，需要解决gpgkey问题。根据需要设置rpm事务的参数flag，并设置rpm回调函数(由于c难以回调rust函数，故直接使用c实现)，由rpm-devel解决具体的rpm安装事务，在执行过程中，回调之前设置的函数，实现单个软件包安装进度的打印显示。

