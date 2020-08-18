## \# 基本概念

-   关于Docker的灵魂三问
    -   Docker 到底是什么？  
        Docker 属于 Linux 容器的一种封装，提供简单易用的容器使用接口。  
    -   要解决什么问题？  
        （1）提供一次性的环境。比如，本地测试他人的软件、持续集成的时候提供单元测试和构建的环境。  
        （2）提供弹性的云服务。因为 Docker 容器可以随开随关，很适合动态扩容和缩容。  
        （3）组建微服务架构。通过多个容器，一台机器可以跑多个服务，因此在本机就可以模拟出微服务架构。  
    -   好处又在哪里？   
        （1）启动快  
        （2）资源占用少  
        （3）体积小  
-   docker CE 与 EE的简单区别
    -   CE( Community Edition)是社区版，简单理解是免费使用，提供小企业与小的IT团队使用,希望从Docker开始，并尝试基于容器的应用程序部署。
    -   EE(Docker Enterprise Edition)是企业版，收费。提供功能更强。适合大企业与打的IT团队。为企业开发和IT团队设计，他们在生产中构建、交付和运行业务关键应用程序


## \# 部署流程

-   [Windows下载并部署Docker](https://www.runoob.com/docker/windows-docker-install.html)  
    -   如果是 win7、win8 等需要利用 docker toolbox 来安装，国内可以使用[阿里云的镜像](http://mirrors.aliyun.com/docker-toolbox/windows/docker-toolbox/)来下载
    -   现在 Docker 有专门的 Win10 专业版系统的安装包，需要 [开启Hyper-V](https://www.runoob.com/docker/windows-docker-install.html)。




## \# 其他

-   [windows 中的类似于sudo的命令（在cmd中以另一个用户的身份运行命令）](https://www.cnblogs.com/vanwoos/p/9866352.html)，例如我的Windows用户是Qing，则命令为：  
    ```shell
    runas /user:qing cmd.exe
    ```
-   系统提示 `No def ault Boot2Docker IS0 found locally，downloading the latest release`  
    表示正在下载boot2docker.iso镜像文件，这个速度会非常慢，这时可以先按Ctrl+C取消安装，把boot2docker.iso文件通过迅雷等下载工具下载下来并放到指定目录（C:\Users\zsl-pc.docker\machine\cache\，此目录在不同电脑上会有所不同）下再安装。再次运行create创建Docker虚拟主机时就不会再去远程下载，而是使用本地的iso文件了。可参考：https://www.jianshu.com/p/f8bb86ff7650 。关于 [为什么需要用到boot2docker](https://www.cnblogs.com/52fhy/p/8413029.html)可以大概阅读下。  



## \# 参考

-   [Docker国际官网](https://www.docker.com/)
-   [Docker 入门教程 - 阮一峰的网络日志](http://www.ruanyifeng.com/blog/2018/02/docker-tutorial.html)  
-   [docker中文社区](https://www.docker.org.cn/index.html)
-   [Docker命令大全](https://blog.csphere.cn/archives/22)
-   [Docker中文文档](http://www.dockerinfo.net/document)


## \# 最后



