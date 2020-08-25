
[学 Rust，免费拿树莓派](https://segmentfault.com/a/1190000023363546)


# Rust学习总结 - 写 Rust 函数，免费树莓派

本次学习Rust真是从0开始，之前从未使用过虚拟机，也从未独立搭建过Docker环境，这次可以尝试一下；  


## \# Rust 三问

Rust是什么？
Rust可以用来干啥？
Rust何其他同类型的语言有什么优势？


## \# 搭建Rust环境的整体思路

- 方法1  
  如果是Win7/8 则需要部署虚拟机，然后通过虚拟机安装 Ubuntu Server 20.04 TLS，如果是Win10则可以直接在应用市场安装Ubuntu Server 20.04 TLS。     
- 方法2  
  使用 Docker  

其中 :     
- 如果是Win7/8 则需要部署虚拟机，然后通过虚拟机安装 Ubuntu Server 20.04 TLS，如果是Win10则可以直接在应用市场安装Ubuntu Server 20.04 TLS。       
- 使用 Docker    


## \# 虚拟机搭配Ubuntu环境搭建Rust  
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

注意：ssvmup 使用 `wasm-bindgen` 在 JavaScript 和 Rust 源代码之间自动生成“胶水”代码，以便 JavaScript 和 Rust 可以使用各自的本机数据类型进行通信。没有 ssvmup，函数参数和返回值将限于 WebAssembly 本地支持的简单类型（即32位整数）。例如，如果没有 ssvmup 和 wasm-bindgen，则无法使用字符串或数组。

Rust 函数位于 src/lib.rs 文件中，只需在输入字符串前加上“ hello” 即可。注意，say() 函数使用＃[wasm_bindgen]进行了注释，从而使 ssvmup 可以生成必要的“管道”。基于此，我们可以从 TypeScript 调用 Rust 函数。


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
    2020年8月24日，大陆区微软商店中只有Ubuntu 20.04，而且降低版本的话也无法与本次课题推荐的版本不符了。  
  - some users report that exporting RUSTUP_IO_THREADS=1 mitigates the problem  
    使用这个命令，没有再提示异常，但再执行 `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh` 却不会有任何信息和进度展示  
  - use WSLv2  
    2020年8月24日，没有成功，我的Window 10 版本号是 1909，操作系统版本是 18363.1016    
    需要操作系统版本最低[18917](https://blogs.windows.com/windowsexperience/2019/06/12/announcing-windows-10-insider-preview-build-18917/)，更新方法见[如何如何手动安装Windows 10 WSLv2](https://www.groovypost.com/howto/manually-install-windows-10-1903-may-2019-update-now/)    
    更详细的安装/升级介绍可以查看微软官方文档——[Windows Linux子系统Windows 10安装指南](https://docs.microsoft.com/en-us/windows/wsl/install-win10)     

Q：Win10 —— 您的主机不满足在启用 Hyper-V 或 Device/Credential Guard 的情况下运行 VMware Workstation 的最低要求。    
  [解决办法](https://blog.csdn.net/qq_36761831/article/details/81175736)  


## \# 参考

- [Rust学习](https://blog.csdn.net/smallswan/article/details/107903034)
- [如何在 Deno TypeScript 应用程序中访问 Rust 函数？ - MikeTang的个人空间 - OSCHINA](https://my.oschina.net/u/4581704/blog/4415721)
- [🍹 树莓派上的高性能 Node.js 应用](https://www.secondstate.io/articles/get-started-with-raspberry-pi-zh/)



