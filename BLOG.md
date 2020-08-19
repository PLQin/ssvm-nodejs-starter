
[学 Rust，免费拿树莓派](https://segmentfault.com/a/1190000023363546)


# Rust学习总结 - 写 Rust 函数，免费树莓派


## \# Rust 三问

Rust是什么？
Rust可以用来干啥？
Rust何其他同类型的语言有什么优势？


## \# 整体思路

-   方法1  
    如果是Win7/8 则需要部署虚拟机，然后通过虚拟机安装 Ubuntu Server 20.04 TLS，如果是Win10则可以直接在应用市场安装Ubuntu Server 20.04 TLS。     
-   方法2  
    使用 Docker  


## \# 环境搭建

- 如果是Win7/8 则需要部署虚拟机，然后通过虚拟机安装 Ubuntu Server 20.04 TLS，如果是Win10则可以直接在应用市场安装Ubuntu Server 20.04 TLS。     
- 使用 Docker  


```shell
# Install Rust
# 既然我们要用 Rust 写函数，也需要安装 Rust 语言的编译器与工具。
# 如果觉得慢，可以使用中科大镜像，使用方法请自行百度
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



## \# 参考

- [Rust学习](https://blog.csdn.net/smallswan/article/details/107903034)
- [如何在 Deno TypeScript 应用程序中访问 Rust 函数？ - MikeTang的个人空间 - OSCHINA](https://my.oschina.net/u/4581704/blog/4415721)
- [🍹 树莓派上的高性能 Node.js 应用](https://www.secondstate.io/articles/get-started-with-raspberry-pi-zh/)

