
# 设置国内源

```bash
    # 备份
    sudo cp /etc/apt/sources.list /etc/apt/sources.list.bak
    # 更改文件权限使其可编辑；
    sudo chmod 777 /etc/apt/sources.list 

    # 打开文件进行编辑；
    sudo gedit /etc/apt/sources.list 

    # 复制保存
    # deb cdrom:[Ubuntu 16.04 LTS _Xenial Xerus_ - Release amd64 (20160420.1)]/ xenial main restricted
    deb http://mirrors.tuna.tsinghua.edu.cn/ubuntu/ xenial main restricted
    deb http://mirrors.tuna.tsinghua.edu.cn/ubuntu/ xenial-updates main restricted
    deb http://mirrors.tuna.tsinghua.edu.cn/ubuntu/ xenial universe
    deb http://mirrors.tuna.tsinghua.edu.cn/ubuntu/ xenial-updates universe
    deb http://mirrors.tuna.tsinghua.edu.cn/ubuntu/ xenial multiverse
    deb http://mirrors.tuna.tsinghua.edu.cn/ubuntu/ xenial-updates multiverse
    deb http://mirrors.tuna.tsinghua.edu.cn/ubuntu/ xenial-backports main restricted universe multiverse
    deb http://mirrors.tuna.tsinghua.edu.cn/ubuntu/ xenial-security main restricted
    deb http://mirrors.tuna.tsinghua.edu.cn/ubuntu/ xenial-security universe
    deb http://mirrors.tuna.tsinghua.edu.cn/ubuntu/ xenial-security multiverse

    # 更新
    sudo apt update
```