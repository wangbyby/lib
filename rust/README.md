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


# linux下安装rust


1. 设置rustup镜像
```bash
# import USTC mirror
echo "export RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static" >> ~/.bashrc
echo "export RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup" >> ~/.bashrc
source .bashrc

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 安装完毕后刷新环境变量
source ~/.cargo/env

```

2. 设置cargo镜像以及安装

```bash
# cargo代理
cat >~/.cargo/config <<EOF
[source.crates-io]
replace-with = 'tuna'

[source.tuna]
registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"
EOF

# 因为代码提示racer的一些#future功能不能在稳定版使用，无法安装
# 所以安装nightly版本的rust 并设置默认版本
rustup install nightly
rustup default nightly

# 安装RLS组建
rustup component add rls --toolchain nightly
rustup component add rust-analysis --toolchain nightly
rustup component add rust-src --toolchain nightly

# 安装racer
cargo install racer

```

3. 安装后记得要刷新路径

```bash
source ~/.profile
source ~/.cargo/env
```