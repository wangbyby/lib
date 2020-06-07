

# rustup 工具链安装
- 安装rust版本
  - ``rustup install 版本号``
  - Ex. 版本号:``nightly-2020-06-04`` 

- 查看工具链
  - ``rustup toolchain list``
- rustup show
- ``rustup override set nightly`` 即当前工作目录复盖为nightly

# 镜像

``echo 'export RUSTUP_DIST_SERVER=https://mirrors.tuna.tsinghua.edu.cn/rustup' >> ~/.bash_profile``