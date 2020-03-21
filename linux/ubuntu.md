# 设置国内源
```bash
    # 备份
    sudo cp /etc/apt/sources.list /etc/apt/sources.list.bak
    # 更改文件权限使其可编辑；
    sudo chmod 777 /etc/apt/sources.list 

    # 打开文件进行编辑；
    sudo gedit /etc/apt/sources.list 

    # 复制保存
    deb http://mirrors.aliyun.com/ubuntu/ xenial main restricted universe multiverse

    deb http://mirrors.aliyun.com/ubuntu/ xenial-security main restricted universe multiverse

    deb http://mirrors.aliyun.com/ubuntu/ xenial-updates main restricted universe multiverse

    deb http://mirrors.aliyun.com/ubuntu/ xenial-proposed main restricted universe multiverse

    deb http://mirrors.aliyun.com/ubuntu/ xenial-backports main restricted universe multiverse

    deb-src http://mirrors.aliyun.com/ubuntu/ xenial main restricted universe multiverse

    deb-src http://mirrors.aliyun.com/ubuntu/ xenial-security main restricted universe multiverse

    deb-src http://mirrors.aliyun.com/ubuntu/ xenial-updates main restricted universe multiverse

    deb-src http://mirrors.aliyun.com/ubuntu/ xenial-proposed main restricted universe multiverse

    deb-src http://mirrors.aliyun.com/ubuntu/ xenial-backports main restricted universe multiverse

    # 更新
    sudo apt update
```