

## \# Docker 安装

手动安装适合单机安装docker环境，无需搭建docker网络的情况。

1. 卸载旧版本docker  
全新安装时，无需执行该步骤  
```shell
$ sudo apt-get remove docker docker-engine docker.io
```

2. 更新系统软件  
```shell
$ sudo apt-get update
```

3. 安装依赖包  
```shell
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

8. 查看docker版本  
```shell
$ docker -v
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


## \# 遇到的问题

##### Q : 启动 `docker info` 时报错: `Error response from daemon: Bad response from Docker engine`
> [讨论](https://developer.aliyun.com/article/636667)
> [讨论](https://github.com/docker/for-win/issues/1028)

## \# 参考

- [Docker 官网](https://www.docker.com/)
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



