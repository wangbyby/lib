# 一些工具
- rustfmt
    ```bash
        #安装rustfmt
         rustup component add rustfmt
        #使用fmt
        cargo fmt
    ```
- rustfix修复代码
    ```bash
        cargo fix
    ```
- clippy: 更强大的lint工具 
    ```bash
        #安装clippy
        rustup component add clippy
        #使用clippy
        cargo clippy
    ```

# 镜像

## cargo镜像

1. 清华源
``.cargo/config``
```bash
[source.crates-io]
replace-with = 'tuna'

[source.tuna]
registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"

```


2. cargo 更新库
``cargo update -p time``


## rustup镜像

1. rustup 工具链安装
- 安装rust版本
  - ``rustup install 版本号``
  - Ex. 版本号:``nightly-2020-06-04`` 

- 查看工具链
  - ``rustup toolchain list``
- rustup show
- ``rustup override set nightly`` 即当前工作目录复盖为nightly

2. 镜像

``echo 'export RUSTUP_DIST_SERVER=https://mirrors.tuna.tsinghua.edu.cn/rustup' >> ~/.bash_profile``