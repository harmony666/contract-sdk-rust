## 编译
```sh
wasm-pack build
```
生成路径：target/wasm32-unknown-unknown/release/chainmaker_contract.wasm

## 环境依赖
```sh
# rustc cargo wasm-pack
rustc -V
  rustc 1.49.0-nightly (cf9cf7c92 2020-11-10)
cargo -V
  cargo 1.49.0-nightly (d5556aeb8 2020-11-04)
wasm-pack -V
  wasm-pack 0.9.1
```

## 安装rust cargo

> 官方地址: https://www.rust-lang.org/tools/install

**windows**

>  To install Rust on Windows, download and run [rustup-init.exe](https://win.rustup.rs/), then follow the onscreen instructions.
>
> https://win.rustup.rs/

**Linux、Mac**

> curl https://sh.rustup.rs -sSf | sh

## 安装wasm-pack

> sh https://rustwasm.github.io/wasm-pack/installer/init.sh

**若执行失败**
```sh
# 下载 https://rustwasm.github.io/wasm-pack/installer/init.sh
# 运行init.sh，若下载不下来，则手动下载，并注释60行附近download语句，添加： cp ./wasm-pack-v0.9.1-x86_64-unknown-linux-musl.tar.gz ${_file}
```



## 结构

SimContext
  SqlSimContext 
  KVSimContext




## 其他介绍
首先是一些编译优化flag，它们-O0，-O1，-O2，-Os，-Oz，-O3。

-O0：
不进行编译优化（这是默认情况）。当你刚开始移植项目是推荐使用它，因为它会包含许多断言。

-O1：
简单优化。推荐你在既想缩短编译时间又想编译优化时使用。它毕竟比-O2级别的优化编译起来快多了。它会进行asm.js和llvm的-O1进行优化，它会relooping，会删除运行时断言和C++异常捕获，它也会使得-s ALIASING_FUNCTION_POINTERS=1。
想要C++异常捕获重新可用，请设置：-s DISABLE_EXCEPTION_CATCHING=0。

-O2：
和-O1类似，不过多了JavaScript级别的优化以及一些llvm -O3的优化项。当你想发布项目的时候，推荐使用本级别优化。

-O3：
和-O2类似，不过比-O2又多了一些JavaScript优化，而且编译时间明显比-O2长。这个也推荐在发布版本的时候使用。

-Os:
和-O3类似，不过增加了额外的优化以减小生成的代码体积，代价是比-O3性能差一点。-Os优化会同时影响llvm bitcode 和JavaScript文件的生成。

-Oz:
和-Os类似，不过进一步减小了代码体积。



emcc参考：
https://blog.csdn.net/y601500359/article/details/95197654
https://segmentfault.com/a/1190000011335568


Cargo.toml配置参考：
https://rustwasm.github.io/wasm-pack/book/cargo-toml-configuration.html?highlight=wasm-opt#cargotoml-configuration# contract-sdk-rust
