

[学 Rust，免费拿树莓派](https://segmentfault.com/a/1190000023363546)


# Rust学习总结 - 写 Rust 函数，免费拿树莓派

本次学习Rust真是从0开始，之前也没尝试过虚拟机；废话不多说，我们开始吧


## \# Rust 三问

Rust是什么？  
Rust 是一门系统编程语言(Systems Programming Language)，兼顾安全(Safety)、性能(Speed)和并发(Concurrency)。

Rust可以用来干啥？  
Rust作为一门底层的系统编程语言，理论上，使用 C/C++ 的领域都可以使用Rust实现，例如对硬件需要精细控制的嵌入式编程、对性能要求极高的应用软件（数据库引擎、浏览器引擎，3D渲染引擎等）。

Rust和其他同类型的语言有什么优势？  
相对于 C/C++ 的系统性缺陷（内存管理不当造成的安全漏洞），Rust通过所有权(Ownership)机制在编译期间确保内存安全，无需垃圾回收(Garbage Collection, GC)，也不需要手动释放内存。


## \# 搭建Rust环境的整体思路

- 方法A：   
  如果是Win7/8 可以通过部署虚拟机，虚拟机再安装 Ubuntu Server 20.04 TLS和Rust，如果是Win10则可以选择在应用市场安装Ubuntu Server 20.04 TLS。     
- 方法B：   
  使用 Docker  

这里我使用的是Ubuntu虚拟机搭建Rust。


## \# 虚拟机Ubuntu环境搭建Rust  
```shell
# Install Rust
# 既然我们要用 Rust 写函数，也需要安装 Rust 语言的编译器与工具。
# 如果觉得慢，可以使用中科大镜像，使用方法请自行搜索
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
$ source $HOME/.cargo/env

# Install Node.js
$ curl -sL https://deb.nodesource.com/setup_14.x |  bash
$ apt-get install -y nodejs

# Install ssvmup toolchain
# 最后，ssvmup 工具自动执行构建过程并生成所有工件，使 Node 应用程序可以轻松调用 Rust 函数。同样，需要安装 ssvmup 依赖项。
$ npm install -g ssvmup # Append --unsafe-perm if permission denied

# OS dependencies for SSVM
$ sudo apt-get update
$ sudo apt-get -y upgrade
$ sudo apt install build-essential curl wget git vim libboost-all-dev

# Install the nodejs addon for SSVM
$ npm install ssvm
```


## \# 第一个Rust程序

```rust
fn main() {
  let txt = "Hello, world!";
  println!("I say : {}", txt);
}
```  
使用`fn`声明函数，和`JavaScript`倒是大不相同。`main()`是`Rust`程序的默认入口，`println!`表示打印文本到控制台。  

`Rust`输出文字的方式主要有两种：[println!()和print!()](https://www.runoob.com/rust/rust-println.html)。这两个"函数"都是向命令行输出字符串的方法，区别仅在于前者会在输出的最后附加输出一个换行符。


## \# 我做了什么

做了一个九九乘法表，哈哈哈！最终代码：[GitHub](https://github.com/PLQin/ssvm-nodejs-starter)

![image.png](https://segmentfault.com/img/bVbLUIE)


## \# 遇到的问题

Q：Win10 Linux子系统运行命令 rust init，速度太慢     
  把脚本中的 RUSTUP_UPDATE_ROOT 变量[改为中国科学技术大学的镜像](https://blog.csdn.net/inthat/article/details/106742193)  
  ```bash
  # 中国科学技术大学
  RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static
  RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup
  ```

Q：Win10 Linux子系统运行命令 rust init，提示异常
  ```bash
  thread 'main' panicked at 'assertion failed: `(left == right)
  ...
  thread panicked while panicking. aborting.
  Illegal instruction (core dumped)
  ```  
  关于此异常的讨论：  
  - [rustup panic with WSLv1 + glibc 2.31 · Issue #2245 · rust-lang/rustup · GitHub](https://github.com/rust-lang/rustup/issues/2245)
  - [在Linux的Windows子系统中的Ubuntu 20.04上安装rust](https://github.com/rust-lang/rustup/issues/2293)      
  - [在WSL2（Windows10）的Ubuntu中构建Rust语言开发环境](https://koma.blog/wsl2-ubuntu-rust/)      

  解决方法：  
  - use older linux versions  
    2020年8月24日，大陆区微软商店中只有Ubuntu 20.04，而且降低版本的话也与本次课题推荐的版本不符了。  
  - some users report that exporting RUSTUP_IO_THREADS=1 mitigates the problem  
    使用这个命令，没有再提示异常，但再执行 `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh` 却不会有任何信息和进度展示  
  - use WSLv2  
    2020年8月24日，没有成功，我的Window 10 版本号是 1909，操作系统版本是 18363.1016    
    需要操作系统版本最低[18917](https://blogs.windows.com/windowsexperience/2019/06/12/announcing-windows-10-insider-preview-build-18917/)，更新方法见[如何如何手动安装Windows 10 WSLv2](https://www.groovypost.com/howto/manually-install-windows-10-1903-may-2019-update-now/)    
    更详细的安装/升级介绍可以查看微软官方文档——[Windows Linux子系统Windows 10安装指南](https://docs.microsoft.com/en-us/windows/wsl/install-win10)     

Q：Win10 —— 您的主机不满足在启用 Hyper-V 或 Device/Credential Guard 的情况下运行 VMware Workstation 的最低要求。    
  [解决办法](https://blog.csdn.net/qq_36761831/article/details/81175736)  

Q：Ubuntu —— 通过 npm install -g ssvmup 添加了ssvmup，但是执行 ssvmup buil 系统又提示 :  You have not installed ssvmup。  
  其实需要在rust文件所在目录上运行，最后我将ssvmup安装在全局，并同时将ssvmup安装在单项目中，通过npm srcipt来运行ssvmup命令   


## \# 总结

Rust是一门年轻的且充满潜力的编程语言 [狗头]。


## \# 参考

- [🍹 树莓派上的高性能 Node.js 应用](https://www.secondstate.io/articles/get-started-with-raspberry-pi-zh/)
- [Linux Ubuntu常用命令总结](https://blog.csdn.net/simongeek/article/details/45271089)
- [Rust 简明教程 | 快速入门](https://geektutu.com/post/quick-rust.html)
- [了不起的Rust - 应用案例](https://github.com/rustcc/awesome-rust)

