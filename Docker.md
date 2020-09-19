
## \# 基本概念，名词或术语  

- 关于Docker的灵魂三问
  - Docker 是什么？  
    Docker提供简单易用的容器使用接口 属于 Linux 容器的一种封装。简单来说Docker可以让开发者打包他们的应用以及依赖包到一个可移植的镜像中，然后发布到任何流行的 Linux或Windows 机器上，也可以实现虚拟化。Docker本身并不是容器，它是创建容器的工具，是应用容器引擎   
  - 要解决什么问题？  
    （1）提供一次性的环境。比如，本地测试他人的软件、持续集成的时候提供单元测试和构建的环境。  
    （2）提供弹性的云服务。因为 Docker 容器可以随开随关，很适合动态扩容和缩容。  
    （3）组建微服务架构。通过多个容器，一台机器可以跑多个服务，因此在本机就可以模拟出微服务架构。  
  - 好处又在哪里？   
    （1）启动快  
    （2）资源占用少  
    （3）体积小  
-   docker CE 与 EE的简单区别
  - CE( Community Edition)是社区版，简单理解是免费使用，提供小企业与小的IT团队使用,希望从Docker开始，并尝试基于容器的应用程序部署。
  - EE(Docker Enterprise Edition)是企业版，收费。提供功能更强。适合大企业与打的IT团队。为企业开发和IT团队设计，他们在生产中构建、交付和运行业务关键应用程序
- Docker Toolbox：快速简单安装docker环境的套件，有mac和win2个版本。
  - Oracle VM VirtualBox：Oracle虚拟器（机）
  - Kitematic (Alpha)：Docker GUI 工具，它可以更快速、更简单的运行Docker
  - Docker Quickstart Terminal：快速启动docker的命令行工具
- [Docker Machine](https://docs.docker.com/machine/install-machine/)：
  - 创建和管理docker主机的工具，支持在不同版本的Linux上管理多个Docker主机
  - 从零开始学习Docker的重要工具
  - 提供集中创建和管理Docker主机的功能
  - 与Docker Swarm集成创建Docker集群


## \# Docker 安装   
手动安装适合单机安装docker环境，无需搭建docker网络的情况。

1. 卸载旧版本docker  
> 全新安装时，无需执行该步骤    
  ```shell
  $ sudo apt-get remove docker docker-engine docker.io
  ```

2. 更新系统软件     
  ```shell
  $ sudo apt-get update
  ```

3. 安装依赖包    
  ```shell
  # $ which curl # 检查curl是否存在
  # $ sudo apt-get install curl # 不存在则下载 

  # http://linux-command.org/en/docker.html
  # https://docs.docker.com/engine/install/ubuntu/
  $ sudo apt-get install \
      apt-transport-https \
      ca-certificates \
      curl \
      software-properties-common
  ```

4. 添加官方密钥       
执行该命令时，如遇到长时间没有响应说明网络连接不到docker网站，需要使用代-理进行。
  ```shell
  $ curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo apt-key add -
  ```

5. 添加仓库      
  ```shell
  $ sudo add-apt-repository \
    "deb [arch=amd64] https://download.docker.com/linux/ubuntu \
    $(lsb_release -cs) \
    stable"
  ```

6. 再次更新软件    
  ```shell
  $ sudo apt-get update
  ```

7. 安装docker  
docke有两个版本:docker-ce(社区版)和docker-ee(企业版)。如果想指定安装某一版本，可使用 `sudo apt-get install docker-ce=<VERSION>` 命令，把 `<VERSION>` 替换为具体版本即可。  
以下命令没有指定版本，默认就会安装最新版    
  ```shell
  $ sudo apt-get install docker-ce
  ```

8. 查看docker版本并测试docker     
  ```shell
  $ docker -v
  $ docker info
  $ docker run hub.c.163.com/library/busybox:latest echo hello world # => 运行一个docker容器hello world  
  ```

## \# Docker Compose安装
1. 下载docker-compose    
  ```shell
  $ sudo curl -L https://github.com/docker/compose/releases/download/1.17.0/docker-compose-`uname -s`-`uname -m` -o /usr/local/bin/docker-compose
  ```

2. 授权     
  ```shell
  $ sudo chmod +x /usr/local/bin/docker-compose
  ```

3. 查看版本信息      
  ```shell
  $ docker-compose --version
  ```


## \# [Windows下载并部署Docker](https://www.runoob.com/docker/windows-docker-install.html)  
- 如果是 win7、win8 等需要利用 docker toolbox 来安装，国内可以使用[阿里云的镜像](http://mirrors.aliyun.com/docker-toolbox/windows/docker-toolbox/)来下载    
- 现在 Docker 有专门的 Win10 专业版系统的安装包，需要 [开启Hyper-V](https://www.runoob.com/docker/windows-docker-install.html)。   


## \# 其他

- [windows 中的类似于sudo的命令（在cmd中以另一个用户的身份运行命令）](https://www.cnblogs.com/vanwoos/p/9866352.html)，例如我的Windows用户是Qing，则命令为：  
  ```shell
  runas /user:qing cmd.exe
  ```  
- 系统提示 `No def ault Boot2Docker IS0 found locally，downloading the latest release`     
  表示正在下载boot2docker.iso镜像文件，这个速度会非常慢，这时可以先按Ctrl+C取消安装，把boot2docker.iso文件通过迅雷等下载工具下载下来并放到指定目录（C:\Users\zsl-pc.docker\machine\cache\，此目录在不同电脑上会有所不同）下再安装。再次运行create创建Docker虚拟主机时就不会再去远程下载，而是使用本地的iso文件了。可参考：https://www.jianshu.com/p/f8bb86ff7650 。关于 [为什么需要用到boot2docker](https://www.cnblogs.com/52fhy/p/8413029.html)可以大概阅读下。  


## \# 遇到的问题

##### Q : 启动 `docker info` 时报错: `Error response from daemon: Bad response from Docker engine`
> [讨论](https://developer.aliyun.com/article/636667)
> [讨论](https://github.com/docker/for-win/issues/1028)


## \# 参考

- [Docker国际官网](https://www.docker.com/)
- [Docker 入门教程 - 阮一峰的网络日志](http://www.ruanyifeng.com/blog/2018/02/docker-tutorial.html)  
- [Docker命令大全](https://blog.csphere.cn/archives/22)
- [Docker中文文档](http://www.dockerinfo.net/document)
- [什么是Docker? - Docker入门教程 - docker中文社区](https://www.docker.org.cn/book/docker/what-is-docker-16.html)
- [Docker支持的安装方式 - Docker安装手册 - docker中文社区](https://www.docker.org.cn/book/install/supported-platform-17.html)
- [Docker 国内极速下载](http://get.daocloud.io/#install-docker-for-mac-windows)
- [Docker 国内极速下载 - 阿里云镜像](https://cr.console.aliyun.com/cn-hangzhou/new)
- [Docker 官网缓慢下载 - windows](https://docs.docker.com/docker-for-windows/install/)
- [Docker 入门 | 菜鸟教程](https://www.runoob.com/docker/ubuntu-docker-install.html)
- [Docker 镜像使用帮助](https://lug.ustc.edu.cn/wiki/mirrors/help/docker/)
- [Docker 国内镜像源设置](https://juejin.im/post/6844904111582740493)
- [Docker 国内镜像源设置](https://www.jianshu.com/p/405fe33b9032)
- [Docker镜像与Docker容器的相关知识与命令](http://www.heartthinkdo.com/?p=1652#31)
- [Docker安装ubuntu](https://www.jianshu.com/p/ec76a50bef0b)
